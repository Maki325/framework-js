use std::{fs, path::Path, sync::Arc};

use anyhow::Context;
use swc::{self, try_with_handler, PrintArgs};
use swc_common::{util::take::Take, SourceMap, GLOBALS};
use swc_core::ecma::visit::{as_folder, FoldWith, VisitMut, VisitMutWith};
use swc_ecma_ast::{
  CallExpr, Callee, EsVersion, Expr, ExprOrSpread, JSXElement, JSXElementChild, JSXElementName,
  JSXExpr, KeyValueProp, Lit, ObjectLit, Prop, Tpl, TplElement,
};
use swc_ecma_parser::{Syntax, TsConfig};

pub struct TransformVisitor<'a> {
  compiler: &'a swc::Compiler,
}

pub struct TplWrapper {
  pub exprs: Vec<Box<Expr>>,
  pub quasis: Vec<TplElement>,
  pub is_expr_next: bool,
}

impl TplWrapper {
  pub fn new() -> TplWrapper {
    return TplWrapper {
      exprs: vec![],
      quasis: vec![],
      is_expr_next: false,
    };
  }
  pub fn append_expr(&mut self, expr: Box<Expr>) {
    if self.is_expr_next {
      self.exprs.push(expr);
      self.is_expr_next = false;
    } else {
      self.quasis.push(TplElement {
        raw: "".into(),
        ..TplElement::dummy()
      });
      self.exprs.push(expr);
      self.is_expr_next = false;
    }
  }

  pub fn append_quasi<S: AsRef<str>>(&mut self, quasi: S) {
    if self.is_expr_next {
      let last = self.quasis.pop().unwrap();

      let mut string = last.raw.as_str().to_owned();
      string.push_str(quasi.as_ref());
      self.quasis.push(TplElement {
        raw: string.into(),
        ..TplElement::dummy()
      });

      self.is_expr_next = true;
    } else {
      self.quasis.push(TplElement {
        raw: quasi.as_ref().into(),
        ..TplElement::dummy()
      });
      self.is_expr_next = true;
    }
  }

  pub fn build(mut self) -> Tpl {
    if self.is_expr_next == false {
      self.quasis.push(TplElement {
        tail: true,
        ..TplElement::dummy()
      });
    } else {
      if let Some(quasi) = self.quasis.last_mut() {
        quasi.tail = true;
      }
    }

    return Tpl {
      exprs: self.exprs,
      quasis: self.quasis,
      ..Tpl::dummy()
    };
  }
}

impl<'a> TransformVisitor<'a> {
  fn transform_expr(&self, tpl_wrapper: &mut TplWrapper, expr: Expr) {
    match expr {
      Expr::Lit(ref lit) => {
        let string = match lit {
          Lit::Str(value) => value.value.as_str().to_owned(),
          Lit::Bool(value) => value.value.to_string(),
          Lit::Null(_) => "null".to_owned(),
          Lit::Num(value) => value.value.to_string(),
          Lit::BigInt(value) => value.value.to_string(),
          Lit::Regex(_) => return tpl_wrapper.append_expr(Box::new(expr)),
          Lit::JSXText(value) => value.value.as_str().to_owned(),
        };
        tpl_wrapper.append_quasi(string);
      }
      Expr::Tpl(Tpl {
        mut exprs,
        mut quasis,
        ..
      }) => {
        exprs.reverse();
        quasis.reverse();
        let mut take_quasi = true;
        loop {
          if take_quasi {
            let quasi = quasis.pop();
            match quasi {
              None => return,
              Some(quasi) => tpl_wrapper.append_quasi(quasi.raw.as_str()),
            }
            take_quasi = false;
          } else {
            let expr = exprs.pop();
            match expr {
              None => return,
              Some(expr) => tpl_wrapper.append_expr(expr),
            }
            take_quasi = true;
          }
        }
      }
      _ => tpl_wrapper.append_expr(Box::new(expr)),
    }
  }

  fn transform_element_child(&self, tpl_wrapper: &mut TplWrapper, element: &JSXElementChild) {
    match element {
      JSXElementChild::JSXElement(el) => {
        let transformed = self.transform(el);

        self.transform_expr(tpl_wrapper, transformed);
      }
      JSXElementChild::JSXExprContainer(container) => {
        if let JSXExpr::Expr(expr) = &container.expr {
          self.transform_expr(tpl_wrapper, *expr.clone());
        }
      }
      JSXElementChild::JSXFragment(f) => {
        for child in &f.children {
          self.transform_element_child(tpl_wrapper, child);
        }
      }
      JSXElementChild::JSXSpreadChild(sc) => {
        tpl_wrapper.append_expr(sc.expr.clone());
      }
      JSXElementChild::JSXText(text) => {
        tpl_wrapper.append_quasi(text.value.as_str().to_owned());
      }
    }
  }

  fn transform(&self, jsx_element: &Box<JSXElement>) -> Expr {
    let name = &jsx_element.opening.name;

    // If it's custom, we pass the output as "children" props
    // And if it isn't, we just put the tags at the start and the end
    let custom_name = if let JSXElementName::Ident(name) = name {
      let name: &str = name.as_ref();
      match name
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
      {
        true => Some(name),
        false => None,
      }
    } else {
      None
    };

    let mut children = TplWrapper::new();
    for element in &jsx_element.children {
      self.transform_element_child(&mut children, element);
    }

    if let Some(custom_name) = custom_name {
      let expr = if children.exprs.len() == 0 {
        let html = children
          .quasis
          .pop()
          .map_or(String::new(), |q| q.raw.as_str().to_owned());
        Expr::Lit(Lit::Str(html.into()))
      } else {
        Expr::Tpl(children.build())
      };

      let props = swc_ecma_ast::PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: swc_ecma_ast::PropName::Ident("children".into()),
        value: Box::new(expr),
      })));

      let expr = ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(ObjectLit {
          props: vec![props],
          ..ObjectLit::dummy()
        })),
      };

      let call = CallExpr {
        callee: Callee::Expr(Box::new(Expr::Ident(custom_name.into()))),
        args: vec![expr],
        ..CallExpr::dummy()
      };

      return Expr::Call(call);
    } else {
      let name = self
        .compiler
        .print(name, PrintArgs::default())
        .unwrap()
        .code;

      if children.exprs.len() == 0 {
        let html = format!(
          "<{name}>{}</{name}>",
          children
            .quasis
            .pop()
            .map_or(String::new(), |q| q.raw.as_str().to_owned())
        );
        let expr_lit = Expr::Lit(Lit::Str(html.into()));
        return expr_lit;
      }

      let mut shell = TplWrapper::new();

      shell.append_quasi(format!("<{name}>"));
      self.transform_expr(&mut shell, Expr::Tpl(children.build()));
      shell.append_quasi(format!("</{name}>"));

      let expr_tpl = Expr::Tpl(shell.build());
      return expr_tpl;
    }
  }
}

impl<'a> VisitMut for TransformVisitor<'a> {
  fn visit_mut_expr(&mut self, n: &mut Expr) {
    n.visit_mut_children_with(self);

    let replace = if let Expr::JSXElement(jsx_element) = n {
      Some(self.transform(jsx_element))
    } else {
      None
    };

    if let Some(replace) = replace {
      n.map_with_mut(|_| replace);
    }
  }
}

pub fn main() {
  let cm = Arc::<SourceMap>::default();

  let c = swc::Compiler::new(cm.clone());

  let output = GLOBALS
    .set(&Default::default(), || {
      try_with_handler(cm.clone(), Default::default(), |handler| {
        let fm = cm
          .load_file(Path::new("test.tsx"))
          .expect("failed to load file");

        c.parse_js(
          fm,
          handler,
          EsVersion::EsNext,
          Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
          }),
          swc::config::IsModule::Bool(true),
          None,
        )
        .context("failed to parse file")
      })
    })
    .unwrap();

  let output = output.fold_with(&mut as_folder(TransformVisitor { compiler: &c }));

  fs::write(
    "./visited-outputed.tsx",
    c.print(&output, PrintArgs::default()).unwrap().code,
  )
  .unwrap();
  fs::write("./parsed-tsx", format!("{output:#?}")).unwrap();
}

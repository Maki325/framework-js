use std::{fs, path::Path, sync::Arc};

use anyhow::Context;
use swc::{self, try_with_handler, PrintArgs};
use swc_common::{util::take::Take, SourceMap, GLOBALS};
use swc_core::ecma::visit::{as_folder, FoldWith, VisitMut, VisitMutWith};
use swc_ecma_ast::{
  EsVersion, Expr, JSXElement, JSXElementChild, JSXElementName, Lit, Tpl, TplElement,
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

  pub fn build(self) -> Tpl {
    return Tpl {
      exprs: self.exprs,
      quasis: self.quasis,
      ..Tpl::dummy()
    };
  }
}

impl<'a> TransformVisitor<'a> {
  fn transform_element_child(&self, tpl_wrapper: &mut TplWrapper, element: &JSXElementChild) {
    match element {
      JSXElementChild::JSXElement(el) => {
        let transformed = self.transform(el);

        match transformed {
          Expr::Lit(ref lit) => {
            let string = match lit {
              Lit::Str(value) => value.value.as_str().to_owned(),
              Lit::Bool(value) => value.value.to_string(),
              Lit::Null(_) => "null".to_owned(),
              Lit::Num(value) => value.value.to_string(),
              Lit::BigInt(value) => value.value.to_string(),
              Lit::Regex(_) => return tpl_wrapper.append_expr(Box::new(transformed)),
              Lit::JSXText(value) => value.value.as_str().to_owned(),
            };
            tpl_wrapper.append_quasi(string);
          }
          _ => tpl_wrapper.append_expr(Box::new(transformed)),
        }

        // let code = self
        //   .compiler
        //   .print(&transformed, PrintArgs::default())
        //   .unwrap()
        //   .code;
        // println!("Code: {code}");
      }
      JSXElementChild::JSXExprContainer(_expr) => {
        // TODO
        unimplemented!();
      }
      JSXElementChild::JSXFragment(f) => {
        for child in &f.children {
          self.transform_element_child(tpl_wrapper, child);
        }
      }
      JSXElementChild::JSXSpreadChild(_sc) => {
        // TODO
        unimplemented!();
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

    let mut tpl_wrapper = TplWrapper::new();
    for element in &jsx_element.children {
      self.transform_element_child(&mut tpl_wrapper, element);
    }

    if let Some(_custom_name) = custom_name {
      // TODO
      unimplemented!();
    } else {
      let name = self
        .compiler
        .print(name, PrintArgs::default())
        .unwrap()
        .code;

      if tpl_wrapper.exprs.len() == 0 {
        let html = format!(
          "<{name}>{}</{name}>",
          tpl_wrapper
            .quasis
            .pop()
            .map_or(String::new(), |q| q.raw.as_str().to_owned())
        );
        let a = Expr::Lit(Lit::Str(html.into()));

        println!("Built expr str: {a:#?}");

        return a;
      }

      let mut outer = TplWrapper::new();

      outer.append_quasi(format!("<{name}>"));
      outer.append_expr(Box::new(Expr::Tpl(tpl_wrapper.build())));
      outer.append_quasi(format!("</{name}>"));

      // return Expr::Lit(Lit::Str(html.into()));
      let a = Expr::Tpl(outer.build());

      println!("Built expr tpl: {a:#?}");

      return a;
    }
  }
}

impl<'a> VisitMut for TransformVisitor<'a> {
  // If `visit_mut_lit` is implemented, this function is skipped
  // fn visit_mut_str(&mut self, n: &mut swc_ecma_ast::Str) {
  //   n.visit_mut_children_with(self);
  //   println!("visit_mut_str {n:#?}");
  //   n.value = "lit :)".into();
  //   n.raw = None;
  // }

  // fn visit_mut_jsx_element(&mut self, n: &mut swc_ecma_ast::JSXElement) {
  //   n.visit_mut_children_with(self);
  //   // println!("visit_mut_jsx_element {n:?}");
  // }

  // fn visit_mut_jsx_expr_container(&mut self, n: &mut swc_ecma_ast::JSXExprContainer) {
  //   println!("visit_mut_jsx_expr_container {n:?}");
  //   println!(
  //     "{}",
  //     self.compiler.print(&*n, PrintArgs::default()).unwrap().code
  //   );
  //   n.visit_mut_children_with(self);
  // }

  // fn visit_mut_exprs(&mut self, n: &mut Vec<Box<swc_ecma_ast::Expr>>) {
  //   n.visit_mut_children_with(self);
  //   println!("visit_mut_exprs {n:?}");
  // }

  fn visit_mut_expr(&mut self, n: &mut Expr) {
    n.visit_mut_children_with(self);
    // println!("visit_mut_expr {n:?}\n");

    let replace = if let Expr::JSXElement(jsx_element) = n {
      Some(self.transform(jsx_element))
    } else {
      None
    };

    if let Some(replace) = replace {
      n.map_with_mut(|_| replace);
    }
    // n.map_with_mut(|_self| swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Num(5.into())));
  }

  // fn visit_mut_expr_stmt(&mut self, n: &mut swc_ecma_ast::ExprStmt) {
  //   n.visit_mut_children_with(self);
  //   println!("visit_mut_expr_stmt {n:?}");
  // }

  // fn visit_mut_tpl(&mut self,n: &mut swc_ecma_ast::Tpl) {

  // }
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

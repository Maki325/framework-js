use crate::tpl_wrapper::TplWrapper;
use anyhow::Context;
use std::{fs, path::Path, sync::Arc};
use swc::{self, try_with_handler, PrintArgs};
use swc_common::{util::take::Take, SourceMap, GLOBALS};
use swc_core::ecma::visit::{as_folder, FoldWith, VisitMut, VisitMutWith};
use swc_ecma_ast::{
  CallExpr, Callee, EsVersion, Expr, ExprOrSpread, JSXElement, JSXElementName, JSXObject,
  KeyValueProp, Lit, ObjectLit, Prop,
};
use swc_ecma_parser::{Syntax, TsConfig};

pub struct TransformVisitor<'a> {
  #[allow(unused)]
  compiler: &'a swc::Compiler,
}

fn stringify_jsx_object(jsx_object: &JSXObject) -> String {
  match jsx_object {
    JSXObject::Ident(ident) => ident.to_string(),
    JSXObject::JSXMemberExpr(member) => {
      format!("{}.{}", stringify_jsx_object(&member.obj), member.prop)
    }
  }
}

fn stringify_jsx_element_name(name: &JSXElementName) -> String {
  match name {
    JSXElementName::Ident(ident) => ident.to_string(),
    JSXElementName::JSXNamespacedName(namespace) => {
      format!("{}:{}", namespace.ns, namespace.name)
    }
    JSXElementName::JSXMemberExpr(member) => {
      format!("{}.{}", stringify_jsx_object(&member.obj), member.prop)
    }
  }
}

pub fn transform(jsx_element: Box<JSXElement>) -> Expr {
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
  for element in jsx_element.children {
    children.append_element_child(element);
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
    let name = stringify_jsx_element_name(name);
    // let name = self
    //   .compiler
    //   .print(name, PrintArgs::default())
    //   .unwrap()
    //   .code;

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
    shell.append_tpl(children);
    shell.append_quasi(format!("</{name}>"));

    let expr_tpl = Expr::Tpl(shell.build());
    return expr_tpl;
  }
}

impl<'a> VisitMut for TransformVisitor<'a> {
  fn visit_mut_expr(&mut self, n: &mut Expr) {
    n.visit_mut_children_with(self);

    if let Expr::JSXElement(_) = n {
      n.map_with_mut(|n| {
        if let Expr::JSXElement(jsx_element) = n {
          transform(jsx_element)
        } else {
          unreachable!()
        }
      });
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

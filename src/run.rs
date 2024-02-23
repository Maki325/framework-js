use std::{fs, path::Path, sync::Arc};

use anyhow::Context;
use swc::{self, try_with_handler, PrintArgs};
use swc_common::{util::take::Take, SourceMap, GLOBALS};
use swc_core::ecma::visit::{as_folder, FoldWith, VisitMut, VisitMutWith};
use swc_ecma_ast::{EsVersion, Expr, JSXElement, JSXElementChild, JSXElementName, Lit, Tpl};
use swc_ecma_parser::{Syntax, TsConfig};

pub struct TransformVisitor<'a> {
  compiler: &'a swc::Compiler,
}

impl<'a> TransformVisitor<'a> {
  fn transform_element_child(&self, element: &JSXElementChild) -> String {
    match element {
      JSXElementChild::JSXElement(el) => {
        let transformed = self.transform(el);

        let code = self
          .compiler
          .print(&transformed, PrintArgs::default())
          .unwrap()
          .code;

        println!("Code: {code}");

        return code;
      }
      JSXElementChild::JSXExprContainer(expr) => {
        // TODO
        unimplemented!();
      }
      JSXElementChild::JSXFragment(f) => {
        let mut children = vec![];
        for child in &f.children {
          children.push(self.transform_element_child(child));
        }

        return children.join("");
      }
      JSXElementChild::JSXSpreadChild(sc) => {
        // TODO
        unimplemented!();
      }
      JSXElementChild::JSXText(text) => {
        return text.value.as_str().to_owned();
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

    let mut output = vec![];
    for element in &jsx_element.children {
      output.push(self.transform_element_child(element));
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

      let html = format!("<{name}>{}</{name}>", output.join(""));
      println!("HTML: {html}");
      return Expr::Lit(Lit::Str(html.into()));
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

use std::{fs, path::Path, sync::Arc};

use anyhow::Context;
use swc::{self, try_with_handler, PrintArgs};
use swc_common::{SourceMap, GLOBALS};
use swc_core::ecma::{
  transforms::testing::test_inline,
  visit::{as_folder, FoldWith, VisitMut},
};
use swc_ecma_ast::{EsVersion, ModuleDecl, ModuleItem, Program};
use swc_ecma_parser::{Syntax, TsConfig};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
  // If `visit_mut_lit` is implemented, this function is skipped
  fn visit_mut_str(&mut self, n: &mut swc_ecma_ast::Str) {
    println!("visit_mut_str {n:#?}");
    n.value = "lit :(".into();
    n.raw = None;
  }

  // fn visit_mut_lit(&mut self, n: &mut swc_ecma_ast::Lit) {
  //   println!("visit_mut_lit {n:#?}");
  //   if let swc_ecma_ast::Lit::Str(str) = n {
  //     // n = &mut swc_ecma_ast::Lit::Str("".into());
  //     str.value = "LIT :)".into();
  //     str.raw = None;
  //     // if let Some(_) = str.raw {
  //     //   str.raw = Some("'LIT'".into());
  //     // }
  //   }
  // }

  fn visit_mut_ts_type(&mut self, n: &mut swc_ecma_ast::TsType) {
    // println!("TsType: {n:#?}");
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

  // let out_writer = Box::new(std::io::stdout()) as Box<dyn Write>;

  let output = if let Program::Module(module) = output {
    // module.fold_with(TransformVisitor {}.into());
    Program::Module(module.fold_with(&mut as_folder(TransformVisitor)))
    // println!("Module!");
    // for item in &module.body {
    //   let a = c.print(item, PrintArgs::default()).unwrap();
    //   println!("A: {}", a.code);
    //   break;
    //   match item {
    //     ModuleItem::ModuleDecl(decl) => {
    //       if let ModuleDecl::TsImportEquals(a) = decl {
    //         // String::from(a);
    //         // println!("ModuleDecl::TsImportEquals: {a}\n");
    //       }
    //       // println!("decl: {decl:?}\n");
    //       // println!("decl: {decl}\n");
    //     }
    //     ModuleItem::Stmt(stmt) => {
    //       // println!("stmt: {stmt:?}\n");
    //     }
    //   }
    // }
  } else {
    output
  };

  fs::write(
    "./visited-outputed.tsx",
    c.print(&output, PrintArgs::default()).unwrap().code,
  )
  .unwrap();
  fs::write("./parsed-tsx", format!("{output:#?}")).unwrap();
}

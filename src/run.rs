use std::{fs, path::Path, sync::Arc};

use anyhow::Context;
use swc::{self, try_with_handler, PrintArgs};
use swc_common::{SourceMap, GLOBALS};
use swc_core::ecma::visit::{as_folder, FoldWith, VisitMut};
use swc_ecma_ast::{EsVersion, Program};
use swc_ecma_parser::{Syntax, TsConfig};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
  // If `visit_mut_lit` is implemented, this function is skipped
  fn visit_mut_str(&mut self, n: &mut swc_ecma_ast::Str) {
    println!("visit_mut_str {n:#?}");
    n.value = "lit :(".into();
    n.raw = None;
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

  let output = if let Program::Module(module) = output {
    Program::Module(module.fold_with(&mut as_folder(TransformVisitor)))
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

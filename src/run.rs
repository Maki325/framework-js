use std::{fs, path::Path, sync::Arc};

use anyhow::Context;
use swc::{self, try_with_handler};
use swc_common::{SourceMap, GLOBALS};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{Syntax, TsConfig};

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

  fs::write("./parsed-tsx", format!("{output:#?}")).unwrap();
}

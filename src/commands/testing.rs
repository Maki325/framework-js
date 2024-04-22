use crate::{transpiler::TranspileVisitor, utils};
use anyhow::Context;
use clap::Args;
use std::{fs, path::PathBuf, sync::Arc};
use swc::{
  config::{Config, JscConfig, Options},
  try_with_handler,
};
use swc_common::{SourceMap, GLOBALS};
use swc_core::ecma::visit::{as_folder, FoldWith};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{Syntax, TsConfig};

#[derive(Debug, Args)]
pub struct TestCommandInfo {
  input: PathBuf,
  output: PathBuf,

  #[arg(short, long)]
  minify: bool,
}

pub fn testing(info: TestCommandInfo) {
  let input_file = utils::path::make_abs_path(info.input).unwrap();
  let output_file = utils::path::make_abs_path(info.output).unwrap();

  let cm = Arc::<SourceMap>::default();

  let c = swc::Compiler::new(cm.clone());

  let code = GLOBALS
    .set(&Default::default(), || {
      try_with_handler(cm.clone(), Default::default(), |handler| {
        let fm = cm.load_file(&input_file).expect("failed to load file");

        let output = c
          .parse_js(
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
          .context("failed to parse file")?;

        let output = output.fold_with(&mut as_folder(TranspileVisitor::new(&c)));

        c.process_js(
          handler,
          output,
          &Options {
            config: Config {
              minify: info.minify.into(),
              jsc: JscConfig {
                target: Some(EsVersion::EsNext),
                ..JscConfig::default()
              },
              ..Config::default()
            },
            ..Options::default()
          },
        )
        .context("failed to process file")
      })
    })
    .unwrap();

  fs::write(output_file, code.code).unwrap();
}

use crate::{specs::type_info::TypeInfo, utils};
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct TypecheckCommandInfo {
  input: PathBuf,
  output: PathBuf,
}

pub fn typecheck(info: TypecheckCommandInfo) {
  let input_file = utils::path::make_abs_path(info.input).unwrap();
  let output_file = utils::path::make_abs_path(info.output).unwrap();

  TypeInfo::of_file(input_file, output_file).unwrap();
}

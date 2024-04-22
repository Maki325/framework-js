#![feature(debug_closure_helpers)]

use clap::{Parser, Subcommand};

mod commands;
mod specs;
mod tpl_wrapper;
mod transpiler;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Test(commands::TestCommandInfo),
  Typecheck(commands::TypecheckCommandInfo),
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Test(info) => commands::testing(info),
    Commands::Typecheck(info) => commands::typecheck(info),
  }
}

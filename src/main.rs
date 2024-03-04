use clap::{Parser, Subcommand};

mod commands;
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
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Test(info) => commands::testing(info),
  }
}

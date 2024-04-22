mod command;
mod visitor;

pub use command::{typecheck, TypecheckCommandInfo};
pub use visitor::TypecheckerVisitor;

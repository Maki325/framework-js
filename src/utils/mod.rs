use crate::{
  specs::type_info::ExportType,
  transpiler::{ComponentType, ToCreateAsync, TransfromedJSX, TranspileVisitor},
};
use rand::{distributions::Alphanumeric, Rng};
use stringify::Stringify;
use swc::PrintArgs;
use swc_common::{util::take::Take, Span};
use swc_ecma_ast::{
  AwaitExpr, CallExpr, Callee, Expr, ExprOrSpread, Ident, MemberExpr, MemberProp,
};

pub mod path;
pub mod stringify;
mod style;
pub use style::style_object_to_string;

pub fn expr_to_string(compiler: &swc::Compiler, expr: &Expr) -> String {
  return compiler.print(expr, PrintArgs::default()).unwrap().code;
}

pub fn generate_random_variable_name(len: usize) -> String {
  return format!(
    "_{}",
    rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(len)
      .map(char::from)
      .collect::<String>(),
  );
}

pub fn call_framework_fn<S: AsRef<str>>(fn_name: S, args: Vec<ExprOrSpread>) -> Expr {
  return Expr::Call(CallExpr {
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      obj: Box::new(Expr::Ident("global".into())),
      prop: MemberProp::Ident(fn_name.as_ref().into()),
      ..MemberExpr::dummy()
    }))),
    args,
    ..CallExpr::dummy()
  });
}

pub fn call_framework_stringify(expr: Box<Expr>, later_create_ident: Ident) -> Expr {
  return call_framework_fn(
    "___FRAMEWORK_JS_STRINGIFY___",
    vec![
      expr.into(),
      Box::new(Expr::Ident(later_create_ident)).into(),
    ],
  );
}

pub enum Processed {
  Async(String),
  Sync(Expr),
}

pub fn process_transformed_jsx(
  (transformed, custom): TransfromedJSX,
  v: &TranspileVisitor,
  to_create: &mut ToCreateAsync,
) -> Processed {
  if let ComponentType::Custom(name) = custom {
    let is_async = v.typechecker.get_variable_type(&name.stringify()).map_or(
      // We match `true` by default, because if it's async,
      // and we didn't treat it as such code will break
      true,
      |t| match t {
        // If the type is VarType::Other, chances are that
        // We fell thru in the typechecker, so we're gonna
        // Just going with the default `true`
        ExportType::Other => true,
        _ => t.is_async(),
      },
    );

    if false == is_async {
      return Processed::Sync(call_framework_stringify(
        Box::new(transformed),
        v.typechecker.later_create_ident.clone(),
      ));
    }

    let id = generate_random_variable_name(12);

    to_create.push((
      id.clone(),
      Expr::Await(AwaitExpr {
        arg: Box::new(transformed),
        span: Span::dummy(),
      }),
    ));

    return Processed::Async(format!("<div id=\"{id}\"></div>"));
  } else {
    return Processed::Sync(transformed);
  }
}

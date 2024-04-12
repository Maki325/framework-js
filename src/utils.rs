use std::{
  env,
  error::Error,
  path::{Component, PathBuf},
};

use rand::{distributions::Alphanumeric, Rng};
use swc::PrintArgs;
use swc_common::{util::take::Take, Span};
use swc_ecma_ast::{
  AwaitExpr, CallExpr, Callee, Expr, Ident, JSXAttrName, JSXElementName, JSXMemberExpr, JSXObject,
  Lit, MemberExpr, MemberProp,
};

use crate::transpiler::{ComponentType, ToCreateAsync, TransfromedJSX, TranspileVisitor};

pub trait Stringify {
  fn stringify(self) -> String;
}

impl Stringify for Ident {
  fn stringify(self) -> String {
    return self.sym.to_string();
  }
}

impl Stringify for Lit {
  fn stringify(self) -> String {
    return match self {
      Lit::Str(value) => value.value.as_str().to_owned(),
      Lit::Bool(value) => value.value.to_string(),
      Lit::Null(_) => "null".to_owned(),
      Lit::Num(value) => value.value.to_string(),
      Lit::BigInt(value) => value.value.to_string(),
      Lit::Regex(re) => format!("/{}/{}", re.exp.as_str(), re.flags.as_str()),
      Lit::JSXText(value) => value.value.as_str().to_owned(),
    };
  }
}

pub fn stringify_jsx_object(jsx_object: JSXObject) -> String {
  return match jsx_object {
    JSXObject::Ident(ident) => ident.stringify(),
    JSXObject::JSXMemberExpr(member) => {
      format!(
        "{}.{}",
        stringify_jsx_object(member.obj),
        member.prop.stringify()
      )
    }
  };
}

pub fn stringify_jsx_member_expr(jsx_member_expr: JSXMemberExpr) -> String {
  return format!(
    "{}.{}",
    stringify_jsx_object(jsx_member_expr.obj),
    jsx_member_expr.prop.stringify()
  );
}

pub fn stringify_jsx_element_name(name: JSXElementName) -> String {
  return match name {
    JSXElementName::Ident(ident) => ident.stringify(),
    JSXElementName::JSXNamespacedName(namespace) => {
      format!(
        "{}:{}",
        namespace.ns.stringify(),
        namespace.name.stringify()
      )
    }
    JSXElementName::JSXMemberExpr(member) => stringify_jsx_member_expr(member),
  };
}

pub fn stringify_jsx_attr_name(name: JSXAttrName) -> String {
  match name {
    JSXAttrName::Ident(ident) => ident.stringify(),
    JSXAttrName::JSXNamespacedName(namespace) => {
      format!(
        "{}:{}",
        namespace.ns.stringify(),
        namespace.name.stringify()
      )
    }
  }
}

pub fn escape_string(unescaped: String) -> String {
  return unescaped.replace("\"", "\\\"");
}

pub fn normalize_path(path: &PathBuf) -> PathBuf {
  let mut components = path.components().peekable();
  let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
    components.next();
    PathBuf::from(c.as_os_str())
  } else {
    PathBuf::new()
  };

  for component in components {
    match component {
      Component::Prefix(..) => unreachable!(),
      Component::RootDir => {
        ret.push(component.as_os_str());
      }
      Component::CurDir => {}
      Component::ParentDir => {
        ret.pop();
      }
      Component::Normal(c) => {
        ret.push(c);
      }
    }
  }
  ret
}

pub fn make_abs_path(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
  let path = if path.is_absolute() {
    path
  } else {
    env::current_dir()?.join(path)
  };

  return Ok(normalize_path(&path));
}

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

pub fn call_framework_stringify(expr: Box<Expr>, later_create_ident: Ident) -> Expr {
  Expr::Call(CallExpr {
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      obj: Box::new(Expr::Ident("global".into())),
      prop: MemberProp::Ident("___FRAMEWORK_JS_STRINGIFY___".into()),
      ..MemberExpr::dummy()
    }))),
    args: vec![
      expr.into(),
      Box::new(Expr::Ident(later_create_ident)).into(),
    ],
    ..CallExpr::dummy()
  })
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
    let is_async = v.get_variable_type(&name.stringify()).map_or(
      // We match `true` by default, because if it's async,
      // and we didn't treat it as such code will break
      true,
      |t| t.is_async(),
    );
    if false == is_async {
      return Processed::Sync(call_framework_stringify(
        Box::new(transformed),
        v.later_create_ident.clone(),
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

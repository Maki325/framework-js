use std::{
  env,
  error::Error,
  path::{Component, PathBuf},
};

use rand::{distributions::Alphanumeric, Rng};
use swc::PrintArgs;
use swc_common::util::take::Take;
use swc_ecma_ast::{
  ArrowExpr, BlockStmtOrExpr, CallExpr, Callee, Expr, Ident, JSXAttrName, JSXElementName,
  JSXObject, Lit, ParenExpr,
};

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
    JSXElementName::JSXMemberExpr(member) => {
      format!(
        "{}.{}",
        stringify_jsx_object(member.obj),
        member.prop.stringify()
      )
    }
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

pub fn create_self_invoking_function(block_stmt_or_expr: BlockStmtOrExpr) -> CallExpr {
  let arrow_fn = Expr::Arrow(ArrowExpr {
    is_async: true,
    body: Box::new(block_stmt_or_expr),
    ..ArrowExpr::dummy()
  });

  return CallExpr {
    callee: Callee::Expr(Box::new(Expr::Paren(ParenExpr {
      expr: Box::new(arrow_fn),
      ..ParenExpr::dummy()
    }))),

    ..CallExpr::dummy()
  };
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

use swc_ecma_ast::{Ident, JSXAttrName, JSXElementName, JSXMemberExpr, JSXObject, Lit};

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

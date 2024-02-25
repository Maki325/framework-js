use crate::{tpl_wrapper::TplWrapper, utils::*};
use anyhow::Context;
use phf::phf_map;
use std::{fs, path::Path, sync::Arc};
use swc::{self, try_with_handler, PrintArgs};
use swc_common::{util::take::Take, SourceMap, GLOBALS};
use swc_core::ecma::visit::{as_folder, FoldWith, VisitMut, VisitMutWith};
use swc_ecma_ast::{
  CallExpr, Callee, EsVersion, Expr, ExprOrSpread, JSXAttrOrSpread, JSXAttrValue, JSXElement,
  JSXElementName, JSXExpr, KeyValueProp, Lit, ObjectLit, Prop,
};
use swc_ecma_parser::{Syntax, TsConfig};

pub struct TransformVisitor<'a> {
  #[allow(unused)]
  compiler: &'a swc::Compiler,
}

static PROP_NAME_MAP: phf::Map<&'static str, &'static str> = phf_map! {
  "className" => "class",
};

fn expr_to_string(compiler: &swc::Compiler, expr: &Expr) -> String {
  return compiler.print(expr, PrintArgs::default()).unwrap().code;
}

pub fn transform(compiler: &swc::Compiler, jsx_element: Box<JSXElement>) -> Expr {
  let opening = jsx_element.opening;
  let name = opening.name;

  // If it's custom, we pass the output as "children" props
  // And if it isn't, we just put the tags at the start and the end
  let custom_name = if let JSXElementName::Ident(name) = &name {
    let name: &str = name.as_ref();
    match name
      .chars()
      .next()
      .map(|c| c.is_uppercase())
      .unwrap_or(false)
    {
      true => Some(name),
      false => None,
    }
  } else {
    None
  };

  let mut children = TplWrapper::new();
  for element in jsx_element.children {
    children.append_element_child(compiler, element);
  }

  if let Some(custom_name) = custom_name {
    let expr = if children.exprs.len() == 0 {
      let html = children
        .quasis
        .pop()
        .map_or(String::new(), |q| q.raw.as_str().to_owned());
      Expr::Lit(Lit::Str(html.into()))
    } else {
      Expr::Tpl(children.build())
    };

    let children_prop = swc_ecma_ast::PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: swc_ecma_ast::PropName::Ident("children".into()),
      value: Box::new(expr),
    })));

    let props = vec![children_prop];

    let expr = ExprOrSpread {
      spread: None,
      expr: Box::new(Expr::Object(ObjectLit {
        props,
        ..ObjectLit::dummy()
      })),
    };

    let call = CallExpr {
      callee: Callee::Expr(Box::new(Expr::Ident(custom_name.into()))),
      args: vec![expr],
      ..CallExpr::dummy()
    };

    return Expr::Call(call);
  }

  let mut props = TplWrapper::new();

  for attr in opening.attrs {
    props.append_quasi(" ");
    match attr {
      JSXAttrOrSpread::SpreadElement(spread) => {
        props.append_quasi("${Object.entries(");
        props.append_quasi(expr_to_string(&compiler, &spread.expr));
        props.append_quasi(").map(([key, value]) => `${key}=\"${value ? (typeof value === 'string' ? value : (value instanceof RegExp ? value.toString() : JSON.stringify(value))).replace(/\"/mg, '\\\\\"') : 'true'}\"`).join(' ')}");
      }
      JSXAttrOrSpread::JSXAttr(attr) => {
        let prop_name = stringify_jsx_attr_name(attr.name);

        let prop_name = match PROP_NAME_MAP.get(&prop_name) {
          Some(name) => name.to_string(),
          None => prop_name,
        };

        props.append_quasi(format!("{prop_name}=\""));
        match attr.value {
          None => props.append_quasi("true\""),
          Some(value) => {
            let value = match value {
              JSXAttrValue::Lit(lit) => lit.stringify(),
              JSXAttrValue::JSXExprContainer(container) => match container.expr {
                JSXExpr::JSXEmptyExpr(_) => "true".to_string(),
                JSXExpr::Expr(expr) => expr_to_string(&compiler, &expr),
              },
              JSXAttrValue::JSXElement(el) => {
                props.append_expr(transform(compiler, el));
                continue;
              }
              JSXAttrValue::JSXFragment(frag) => {
                for child in frag.children {
                  props.append_element_child(compiler, child);
                }
                continue;
              }
            };
            props.append_quasi(escape_string(value));
            props.append_quasi("\"");
          }
        }
      }
    }
  }

  let name = stringify_jsx_element_name(name);

  if children.exprs.len() == 0 && props.quasis.len() == 0 {
    let html = format!(
      "<{name}>{}</{name}>",
      children
        .quasis
        .pop()
        .map_or(String::new(), |q| q.raw.as_str().to_owned())
    );
    let expr_lit = Expr::Lit(Lit::Str(html.into()));
    return expr_lit;
  }

  let mut shell = TplWrapper::new();

  shell.append_quasi(format!("<{name} "));
  shell.append_tpl(props);
  shell.append_quasi(format!(">"));
  shell.append_tpl(children);
  shell.append_quasi(format!("</{name}>"));

  let expr_tpl = Expr::Tpl(shell.build());
  return expr_tpl;
}

impl<'a> VisitMut for TransformVisitor<'a> {
  fn visit_mut_expr(&mut self, n: &mut Expr) {
    n.visit_mut_children_with(self);

    if let Expr::JSXElement(_) = n {
      n.map_with_mut(|n| {
        if let Expr::JSXElement(jsx_element) = n {
          transform(self.compiler, jsx_element)
        } else {
          unreachable!()
        }
      });
    }
  }
}

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

  let output = output.fold_with(&mut as_folder(TransformVisitor { compiler: &c }));

  fs::write(
    "./visited-outputed.tsx",
    c.print(&output, PrintArgs::default()).unwrap().code,
  )
  .unwrap();
  fs::write("./parsed-tsx", format!("{output:#?}")).unwrap();
}

use std::collections::HashMap;

use crate::{
  tpl_wrapper::TplWrapper,
  utils::{self, generate_random_variable_name, Stringify},
};
use phf::phf_map;
use swc;
use swc_common::{util::take::Take, Span};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{
  AssignTarget, AwaitExpr, BinExpr, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, Decl, Expr,
  ExprOrSpread, Ident, IfStmt, JSXAttrOrSpread, JSXAttrValue, JSXElement, JSXElementName, JSXExpr,
  KeyValueProp, Lit, ObjectLit, Pat, Prop, PropName, PropOrSpread, ReturnStmt, SimpleAssignTarget,
  Stmt, VarDecl, VarDeclKind, VarDeclarator,
};

type IsVariableJsxMap = HashMap<Ident, bool>;

pub struct TranspileVisitor<'a> {
  #[allow(unused)]
  compiler: &'a swc::Compiler,

  response_ident: Ident,
  component_replace_id_ident: Ident,
  returns_jsx: bool,

  function_variable_types: Vec<IsVariableJsxMap>,
  last_arrow_function_is_jsx: bool,
}

impl TranspileVisitor<'_> {
  pub fn new(compiler: &'_ swc::Compiler) -> TranspileVisitor {
    return TranspileVisitor {
      compiler,

      response_ident: generate_random_variable_name(16).as_str().into(),
      component_replace_id_ident: generate_random_variable_name(16).as_str().into(),
      returns_jsx: false,

      function_variable_types: vec![],
      last_arrow_function_is_jsx: false,
    };
  }
}

static PROP_NAME_MAP: phf::Map<&'static str, &'static str> = phf_map! {
  "className" => "class",
};

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

    let children_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident("children".into()),
      value: Box::new(expr),
    })));

    let mut props = vec![children_prop];

    for attr in opening.attrs {
      let attr = match attr {
        JSXAttrOrSpread::SpreadElement(spread) => {
          props.push(PropOrSpread::Spread(spread));
          continue;
        }
        JSXAttrOrSpread::JSXAttr(attr) => attr,
      };

      let prop_name = match attr.name {
        swc_ecma_ast::JSXAttrName::Ident(ident) => ident,
        _ => {
          unimplemented!("Other types of attribute names are not implemented for prop propagation!")
        }
      };

      let key = PropName::Ident(match PROP_NAME_MAP.get(prop_name.sym.as_str()) {
        Some(name) => name.to_string().as_str().into(),
        None => prop_name,
      });

      let value = match attr.value {
        None => Box::new(Expr::Lit(Lit::Bool(true.into()))),
        Some(value) => match value {
          JSXAttrValue::Lit(lit) => Box::new(Expr::Lit(lit)),
          JSXAttrValue::JSXExprContainer(container) => match container.expr {
            JSXExpr::JSXEmptyExpr(_) => Box::new(Expr::Lit(Lit::Bool(true.into()))),
            JSXExpr::Expr(expr) => expr,
          },
          JSXAttrValue::JSXElement(el) => Box::new(transform(compiler, el)),
          JSXAttrValue::JSXFragment(frag) => {
            let mut children = TplWrapper::new();
            for child in frag.children {
              children.append_element_child(compiler, child);
            }
            Box::new(Expr::Tpl(children.build()))
          }
        },
      };

      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key,
        value,
      }))));
    }

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

    let var_name: Ident = utils::generate_random_variable_name(12).as_str().into();

    let var_decl = VarDecl {
      kind: VarDeclKind::Const,
      decls: vec![VarDeclarator {
        name: Pat::Ident(var_name.clone().into()),
        init: Some(Box::new(Expr::Call(call))),
        ..VarDeclarator::dummy()
      }],
      ..VarDecl::dummy()
    };

    let check_if_var_is_promise = Expr::Bin(BinExpr {
      left: Box::new(Expr::Ident(var_name.clone())),
      op: swc_ecma_ast::BinaryOp::InstanceOf,
      right: Box::new(Expr::Ident("Promise".into())),
      ..BinExpr::dummy()
    });

    let return_awaited = ReturnStmt {
      span: Span::dummy(),
      arg: Some(Box::new(Expr::Await(AwaitExpr {
        span: Span::dummy(),
        arg: Box::new(Expr::Ident(var_name.clone())),
      }))),
    };

    let return_not_awaited = ReturnStmt {
      span: Span::dummy(),
      arg: Some(Box::new(Expr::Ident(var_name.clone()))),
    };

    let if_stmt = IfStmt {
      span: Span::dummy(),
      test: Box::new(check_if_var_is_promise),
      cons: Box::new(Stmt::Return(return_awaited)),
      alt: Some(Box::new(Stmt::Return(return_not_awaited))),
    };

    let block_stmt = BlockStmt {
      stmts: vec![Stmt::Decl(Decl::Var(Box::new(var_decl))), Stmt::If(if_stmt)],
      ..BlockStmt::dummy()
    };

    let await_expr = Expr::Await(AwaitExpr {
      arg: Box::new(Expr::Call(utils::create_self_invoking_function(
        BlockStmtOrExpr::BlockStmt(block_stmt),
      ))),
      span: Span::default(),
    });

    return await_expr;
  }

  let mut props = TplWrapper::new();

  for attr in opening.attrs {
    props.append_quasi(" ");
    match attr {
      JSXAttrOrSpread::SpreadElement(spread) => {
        props.append_quasi("${Object.entries(");
        props.append_quasi(utils::expr_to_string(&compiler, &spread.expr));
        props.append_quasi(").map(([key, value]) => `${key}=\"${value ? (typeof value === 'string' ? value : (value instanceof RegExp ? value.toString() : JSON.stringify(value))).replace(/\"/mg, '\\\\\"') : 'true'}\"`).join(' ')}");
      }
      JSXAttrOrSpread::JSXAttr(attr) => {
        let prop_name = utils::stringify_jsx_attr_name(attr.name);

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
                JSXExpr::Expr(expr) => utils::expr_to_string(&compiler, &expr),
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
            props.append_quasi(utils::escape_string(value));
            props.append_quasi("\"");
          }
        }
      }
    }
  }

  let name = utils::stringify_jsx_element_name(name);

  let mut shell = TplWrapper::new();

  shell.append_quasi(format!("<{name}"));
  shell.append_tpl(props);
  shell.append_quasi(format!(">"));
  shell.append_tpl(children);
  shell.append_quasi(format!("</{name}>"));

  let expr_tpl = Expr::Tpl(shell.build());
  return expr_tpl;
}

impl TranspileVisitor<'_> {
  fn is_ident_jsx(&self, name: &Ident) -> bool {
    for map in self.function_variable_types.iter().rev() {
      match map.get(name) {
        None => continue,
        Some(value) => return *value,
      }
    }

    return false;
  }

  fn is_expr_jsx<E: AsRef<Expr>>(&self, expr: E) -> bool {
    match expr.as_ref() {
      Expr::JSXElement(_) | Expr::JSXFragment(_) => return true,
      Expr::Assign(assign) => self.is_expr_jsx(&assign.right),
      Expr::Await(expr) => self.is_expr_jsx(&expr.arg),
      Expr::Call(call) => match &call.callee {
        Callee::Expr(e) => self.is_expr_jsx(e),
        _ => false,
      },
      Expr::Cond(cond) => self.is_expr_jsx(&cond.cons) || self.is_expr_jsx(&cond.alt),
      // Expr::Fn() => Dont really know? Probably isnt JSX
      // Expr::Fn(_) => self.last_arrow_function_is_jsx,
      Expr::Ident(ident) => self.is_ident_jsx(ident),
      // Expr::Member() => TODO: Implement dis lol
      Expr::Paren(paren) => self.is_expr_jsx(&paren.expr),
      _ => return false,
    }
  }
}

impl<'a> VisitMut for TranspileVisitor<'a> {
  fn visit_mut_expr(&mut self, n: &mut Expr) {
    n.visit_mut_children_with(self);

    if let Expr::JSXElement(_) = n {
      n.map_with_mut(|n| {
        let Expr::JSXElement(jsx_element) = n else {
          unreachable!()
        };

        transform(self.compiler, jsx_element)
      });
    }
  }

  fn visit_mut_arrow_expr(&mut self, arrow: &mut swc_ecma_ast::ArrowExpr) {
    // Dis needs to be in is_expr_jsx I guess somehow??
    println!("visit_mut_arrow_expr: {:#?}", arrow);

    self.function_variable_types.push(IsVariableJsxMap::new());
    arrow.visit_mut_children_with(self);
    println!(
      "Popped: {:#?}",
      self.function_variable_types.pop().unwrap().into_iter()
    );

    self.last_arrow_function_is_jsx = self.returns_jsx;

    if self.returns_jsx {
      self.returns_jsx = false;

      arrow.params.insert(
        0,
        Pat::Ident(self.component_replace_id_ident.clone().into()).into(),
      );
      arrow
        .params
        .insert(0, Pat::Ident(self.response_ident.clone().into()).into());
    }
  }

  fn visit_mut_assign_expr(&mut self, assign: &mut swc_ecma_ast::AssignExpr) {
    println!("visit_mut_assign_expr: {:#?}", assign);

    let is_jsx = self.is_expr_jsx(&assign.right);
    if let Some(last) = self.function_variable_types.last_mut() {
      match &assign.left {
        AssignTarget::Simple(simple) => match simple {
          SimpleAssignTarget::Ident(ident) => {
            last.insert(ident.id.clone(), is_jsx);
          }
          _ => {}
        },
        _ => {}
      }
    }

    assign.visit_mut_children_with(self);
  }

  fn visit_mut_var_declarator(&mut self, declarator: &mut VarDeclarator) {
    println!("visit_mut_var_declarator: {:#?}", declarator);

    if let Some(init) = &declarator.init {
      let is_jsx = self.is_expr_jsx(init);
      if let Some(last) = self.function_variable_types.last_mut() {
        match &declarator.name {
          Pat::Ident(i) => {
            last.insert(i.id.clone(), is_jsx);
          }
          _ => {}
        }
      }
    }

    declarator.visit_mut_children_with(self);
  }

  fn visit_mut_return_stmt(&mut self, ret: &mut ReturnStmt) {
    println!("visit_mut_return_stmt: {:#?}", ret);

    let Some(arg) = &ret.arg else {
      ret.visit_mut_children_with(self);
      return;
    };

    self.returns_jsx = self.is_expr_jsx(arg);
    println!("Arg: {}", self.returns_jsx);

    ret.visit_mut_children_with(self);
  }

  fn visit_mut_fn_decl(&mut self, decl: &mut swc_ecma_ast::FnDecl) {
    println!("visit_mut_fn_decl: {:#?}", decl);

    // We are in function
    // We parse the function
    // If a special flag is set, we know that the function returns JSX
    // So we change the call mechanism
    // Otherwise, we keep it the same

    self.function_variable_types.push(IsVariableJsxMap::new());
    decl.visit_mut_children_with(self);
    println!(
      "Popped: {:#?}",
      self.function_variable_types.pop().unwrap().into_iter()
    );

    if let Some(last) = self.function_variable_types.last_mut() {
      last.insert(decl.ident.clone(), self.returns_jsx);
    }

    if self.returns_jsx {
      self.returns_jsx = false;

      decl.function.params.insert(
        0,
        Pat::Ident(self.component_replace_id_ident.clone().into()).into(),
      );
      decl
        .function
        .params
        .insert(0, Pat::Ident(self.response_ident.clone().into()).into());
    }
  }
}

use std::collections::HashMap;

use crate::{
  tpl_wrapper::TplWrapper,
  utils::{self, stringify::Stringify},
};
use phf::phf_map;
use swc;
use swc_common::{util::take::Take, Span};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{
  ArrayLit, ArrayPat, ArrowExpr, AssignTarget, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, Decl,
  Expr, ExprOrSpread, ExprStmt, Ident, JSXAttrOrSpread, JSXAttrValue, JSXElement, JSXElementName,
  JSXExpr, JSXMemberExpr, JSXObject, KeyValueProp, Lit, MemberExpr, MemberProp, ObjectLit,
  ParenExpr, Pat, Prop, PropName, PropOrSpread, Regex, ReturnStmt, SimpleAssignTarget, Stmt,
  VarDecl, VarDeclKind, VarDeclarator,
};

#[derive(Debug, Clone, Copy)]
pub enum VarType {
  JSX,
  AsyncJSX,
  Other,
  AsyncOther,
}

impl VarType {
  pub fn is_async(self) -> bool {
    match self {
      VarType::AsyncJSX | VarType::AsyncOther => true,
      VarType::JSX | VarType::Other => false,
    }
  }

  fn awaited(self) -> VarType {
    return match self {
      VarType::JSX => VarType::AsyncJSX,
      VarType::Other => VarType::AsyncOther,
      _ => self,
    };
  }

  // Biggest -> Lowest
  // Awaited JSX -> JSX -> Awaited Other -> Other
  fn priority(self) -> u8 {
    return match self {
      VarType::AsyncJSX => 3,
      VarType::JSX => 2,
      VarType::AsyncOther => 1,
      VarType::Other => 0,
    };
  }

  fn gt(self, other: VarType) -> VarType {
    if self.priority() > other.priority() {
      return self;
    }
    return other;
  }
}

type IsVariableJsxMap = HashMap<String, VarType>;

pub struct TranspileVisitor<'a> {
  #[allow(unused)]
  pub compiler: &'a swc::Compiler,

  return_type: VarType,
  pub later_create_ident: Ident,

  pub function_variable_types: Vec<IsVariableJsxMap>,
  last_arrow_function_return_type: VarType,
}

impl TranspileVisitor<'_> {
  pub fn new(compiler: &'_ swc::Compiler) -> TranspileVisitor {
    return TranspileVisitor {
      compiler,

      return_type: VarType::Other,
      later_create_ident: utils::generate_random_variable_name(16).as_str().into(),

      function_variable_types: vec![IsVariableJsxMap::new()],
      last_arrow_function_return_type: VarType::Other,
    };
  }

  fn is_ident_jsx<S: AsRef<str>>(&self, name: S) -> VarType {
    for map in self.function_variable_types.iter().rev() {
      match map.get(name.as_ref()) {
        None => continue,
        Some(value) => return *value,
      }
    }

    return VarType::Other;
  }

  fn get_expr_type<E: AsRef<Expr>>(&self, to_assign_expr: E) -> VarType {
    match to_assign_expr.as_ref() {
      Expr::JSXElement(_) | Expr::JSXFragment(_) => return VarType::JSX,
      Expr::Assign(assign) => self.get_expr_type(&assign.right),
      Expr::Await(expr) => self.get_expr_type(&expr.arg).awaited(),
      Expr::Call(call) => match &call.callee {
        Callee::Expr(e) => self.get_expr_type(e),
        _ => VarType::Other,
      },
      Expr::Cond(cond) => self
        .get_expr_type(&cond.cons)
        .gt(self.get_expr_type(&cond.alt)),
      // Expr::Fn() => Dont really know? Probably isnt JSX
      // Expr::Fn(_) => self.last_arrow_function_is_jsx,
      Expr::Arrow(_) => self.last_arrow_function_return_type,
      Expr::Ident(ident) => self.is_ident_jsx(ident),
      // Expr::Member() => TODO: Implement dis lol
      Expr::Paren(paren) => self.get_expr_type(&paren.expr),
      _ => return VarType::Other,
    }
  }

  pub fn get_variable_type<S: AsRef<str>>(&self, name: S) -> Option<VarType> {
    let name = name.as_ref();

    for map in self.function_variable_types.iter().rev() {
      let result = map.get(name);
      if let Some(vt) = result {
        return Some(*vt);
      }
    }

    None
  }
}

static PROP_NAME_MAP: phf::Map<&'static str, &'static str> = phf_map! {
  "className" => "class",
};

#[derive(Debug)]
pub enum CustomComponent {
  Ident(Ident),
  Member(JSXMemberExpr),
}

impl CustomComponent {
  pub fn expr(&self) -> Expr {
    match self {
      CustomComponent::Ident(i) => Expr::Ident(i.clone()),
      CustomComponent::Member(e) => Expr::Member(jsx_member_expr_to_member_expr(e.clone())),
    }
  }
}

impl Stringify for CustomComponent {
  fn stringify(self) -> String {
    match self {
      CustomComponent::Ident(i) => i.stringify(),
      CustomComponent::Member(m) => utils::stringify::stringify_jsx_member_expr(m),
    }
  }
}

#[derive(Debug)]
pub enum ComponentType {
  Custom(CustomComponent),
  HTML,
}

pub type ID = String;
pub type ToCreateAsync = Vec<(ID, Expr)>;
pub type TransfromedJSX = (Expr, ComponentType);

fn jsx_member_expr_to_member_expr(expr: JSXMemberExpr) -> MemberExpr {
  let obj = match expr.obj {
    JSXObject::Ident(i) => Expr::Ident(i),
    JSXObject::JSXMemberExpr(expr) => Expr::Member(jsx_member_expr_to_member_expr(*expr)),
  };
  return MemberExpr {
    span: Span::dummy(),
    obj: Box::new(obj),
    prop: MemberProp::Ident(expr.prop),
  };
}

// We return the main Expr -> ie the TPL
// And the ones that need to be created later are added to `created`
pub fn transform(
  v: &TranspileVisitor,
  jsx_element: Box<JSXElement>,
  to_create: &mut ToCreateAsync,
) -> TransfromedJSX {
  let compiler = &v.compiler;
  let opening = jsx_element.opening;
  let name = opening.name;

  let custom_name = match &name {
    JSXElementName::Ident(ident) => {
      let name: &str = ident.as_ref();
      match name
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
      {
        true => Some(CustomComponent::Ident(ident.clone())),
        false => None,
      }
    }
    JSXElementName::JSXMemberExpr(member) => Some(CustomComponent::Member(member.clone())),
    JSXElementName::JSXNamespacedName(_) => None,
  };

  let mut children = TplWrapper::new();
  for element in jsx_element.children {
    children.append_element_child(v, element, to_create);
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
          JSXAttrValue::JSXElement(el) => {
            match utils::process_transformed_jsx(transform(v, el, to_create), v, to_create) {
              utils::Processed::Async(div) => Box::new(Expr::Lit(Lit::Str(div.into()))),
              utils::Processed::Sync(transformed) => Box::new(transformed),
            }
          }
          JSXAttrValue::JSXFragment(frag) => {
            let mut children = TplWrapper::new();
            for child in frag.children {
              children.append_element_child(v, child, to_create);
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
      callee: Callee::Expr(Box::new(custom_name.expr())),
      args: vec![expr],
      ..CallExpr::dummy()
    };

    return (Expr::Call(call), ComponentType::Custom(custom_name));
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
        let prop_name = utils::stringify::stringify_jsx_attr_name(attr.name);

        if prop_name == "style" {
          let Some(value) = attr.value else {
            continue;
          };
          match value {
            JSXAttrValue::JSXExprContainer(container) => {
              let expr = match container.expr {
                JSXExpr::Expr(expr) => expr,
                JSXExpr::JSXEmptyExpr(empty) => {
                  println!("No `value` expression for style \"{:#?}\"?", empty.span);
                  continue;
                }
              };

              match *expr {
                Expr::Object(obj) => {
                  props.append_quasi(format!("style=\""));
                  props.append_expr(Expr::Tpl(utils::style_object_to_string(obj)));
                  props.append_quasi(format!("\""));
                }
                Expr::Ident(_) => {
                  props.append_quasi(format!("style=\""));
                  props.append_expr(utils::call_framework_fn(
                    "___FRAMEWORK_JS_STYLE_OBJECT___",
                    vec![expr.into()],
                  ));
                  props.append_quasi(format!("\""));
                }
                _ => unimplemented!("Unknown `value` expr type {:#?}!", expr),
              }

              continue;
            }
            _ => unreachable!(),
          }
        }

        let prop_name = match PROP_NAME_MAP.get(&prop_name) {
          Some(name) => name.to_string(),
          None => prop_name,
        };

        props.append_quasi(format!("{prop_name}=\""));
        match attr.value {
          None => props.append_quasi("true\""),
          Some(value) => {
            match value {
              JSXAttrValue::Lit(lit) => props.append_lit(lit),
              JSXAttrValue::JSXExprContainer(container) => match container.expr {
                JSXExpr::JSXEmptyExpr(_) => props.append_quasi("true"),
                JSXExpr::Expr(expr) => props.append_expr(*expr),
              },
              JSXAttrValue::JSXElement(el) => {
                match utils::process_transformed_jsx(transform(v, el, to_create), v, to_create) {
                  utils::Processed::Async(div) => props.append_quasi(div),
                  utils::Processed::Sync(transformed) => props.append_expr(transformed),
                }
              }
              JSXAttrValue::JSXFragment(frag) => {
                for child in frag.children {
                  props.append_element_child(v, child, to_create);
                }
              }
            };
            props.append_quasi("\"");
          }
        }
      }
    }
  }

  let name = utils::stringify::stringify_jsx_element_name(name);

  let mut shell = TplWrapper::new();

  shell.append_quasi(format!("<{name}"));
  shell.append_tpl(props);
  shell.append_quasi(format!(">"));
  shell.append_tpl(children);
  shell.append_quasi(format!("</{name}>"));

  let expr_tpl = Expr::Tpl(shell.build());
  return (expr_tpl, ComponentType::HTML);
}

impl<'a> VisitMut for TranspileVisitor<'a> {
  fn visit_mut_expr(&mut self, n: &mut Expr) {
    n.visit_mut_children_with(self);

    if let Expr::JSXElement(_) = n {
      n.map_with_mut(|n| {
        let Expr::JSXElement(jsx_element) = n else {
          unreachable!()
        };
        let mut created = ToCreateAsync::with_capacity(8);

        let transformed = transform(self, jsx_element, &mut created);

        let first = match utils::process_transformed_jsx(transformed, &self, &mut created) {
          utils::Processed::Async(div) => Expr::Lit(Lit::Str(div.into())),
          utils::Processed::Sync(transformed) => transformed,
        };

        let controller_name: Ident = utils::generate_random_variable_name(12).as_str().into();

        let html_ident: Ident = utils::generate_random_variable_name(12).as_str().into();
        let later_fn_ident: Ident = utils::generate_random_variable_name(12).as_str().into();

        let array_name: Ident = utils::generate_random_variable_name(12).as_str().into();

        let f = |(id, expr): (String, Expr)| {
          let decl = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
              name: Pat::Array(ArrayPat {
                elems: vec![
                  Some(Pat::Ident(html_ident.clone().into())),
                  Some(Pat::Ident(later_fn_ident.clone().into())),
                ],
                optional: false,
                type_ann: None,
                span: Span::dummy(),
              }),
              init: Some(Box::new(expr)),
              ..VarDeclarator::dummy()
            }],
            ..VarDecl::dummy()
          })));

          let script_id = utils::generate_random_variable_name(12);

          let mut tpl = TplWrapper::new();
          tpl.append_quasi(format!(
            "<script id=\"{script_id}\">document.getElementById(\"{id}\").outerHTML = \\`"
          ));

          tpl.append_expr(Expr::Call(CallExpr {
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
              obj: Box::new(Expr::Ident(html_ident.clone().into())),
              prop: MemberProp::Ident("replace".into()),
              ..MemberExpr::dummy()
            }))),
            args: vec![
              Box::new(Expr::Lit(Lit::Regex(Regex {
                exp: "`".into(),
                flags: "mg".into(),
                ..Regex::dummy()
              })))
              .into(),
              Box::new(Expr::Lit(Lit::Str("\\`".into()))).into(),
            ],
            ..CallExpr::dummy()
          }));

          tpl.append_quasi(format!(
            "\\`;document.getElementById(\"{script_id}\").remove();</script>"
          ));

          let enqueue = Stmt::Expr(ExprStmt {
            span: Span::default(),
            expr: Box::new(Expr::Call(CallExpr {
              callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                obj: Box::new(controller_name.clone().into()),
                prop: MemberProp::Ident("enqueue".into()),
                ..MemberExpr::dummy()
              }))),
              args: vec![ExprOrSpread::from(Box::new(tpl.build().into()))],
              ..CallExpr::dummy()
            })),
          });

          let later_fn_call = Stmt::Return(ReturnStmt {
            span: Span::default(),
            arg: Some(Box::new(Expr::Call(CallExpr {
              callee: Callee::Expr(Box::new(Expr::Ident(later_fn_ident.clone().into()))),
              args: vec![ExprOrSpread::from(Box::new(controller_name.clone().into()))],
              ..CallExpr::dummy()
            }))),
          });

          Stmt::Expr(ExprStmt {
            span: Span::default(),
            expr: Box::new(Expr::Call(CallExpr {
              callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                obj: Box::new(Expr::Ident(array_name.clone())),
                prop: MemberProp::Ident("push".into()),
                ..MemberExpr::dummy()
              }))),
              args: vec![Box::new(Expr::Call(CallExpr {
                callee: Callee::Expr(Box::new(Expr::Paren(ParenExpr {
                  expr: Box::new(Expr::Arrow(ArrowExpr {
                    is_async: true,
                    body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                      stmts: vec![decl, enqueue, later_fn_call],
                      ..BlockStmt::dummy()
                    })),
                    ..ArrowExpr::dummy()
                  })),
                  ..ParenExpr::dummy()
                }))),
                ..CallExpr::dummy()
              }))
              .into()],
              ..CallExpr::dummy()
            })),
          })
        };

        let call = Expr::Arrow(ArrowExpr {
          body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
            stmts: {
              let map_var: Ident = utils::generate_random_variable_name(12).as_str().into();
              let mut stmts: Vec<Stmt> = created.into_iter().map(f).collect();

              stmts.insert(
                0,
                Stmt::Decl(Decl::Var(Box::new(VarDecl {
                  kind: VarDeclKind::Const,
                  decls: vec![VarDeclarator {
                    name: Pat::Ident(array_name.clone().into()),
                    init: Some(Box::new(Expr::Call(CallExpr {
                      callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                        obj: Box::new(Expr::Ident(self.later_create_ident.clone().into())),
                        prop: MemberProp::Ident("map".into()),
                        ..MemberExpr::dummy()
                      }))),
                      args: vec![Box::new(Expr::Arrow(ArrowExpr {
                        params: vec![Pat::Ident(map_var.clone().into())],
                        body: Box::new(
                          Expr::Call(CallExpr {
                            callee: Callee::Expr(Expr::Ident(map_var).into()),
                            args: vec![Box::new(Expr::Ident(controller_name.clone())).into()],
                            ..CallExpr::dummy()
                          })
                          .into(),
                        ),
                        ..ArrowExpr::dummy()
                      }))
                      .into()],
                      ..CallExpr::dummy()
                    }))),
                    ..VarDeclarator::dummy()
                  }],
                  ..VarDecl::dummy()
                }))),
              );

              stmts.push(Stmt::Return(ReturnStmt {
                span: Span::dummy(),
                arg: Some(Box::new(Expr::Call(CallExpr {
                  callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                    obj: Box::new(Expr::Ident("Promise".into())),
                    prop: MemberProp::Ident("allSettled".into()),
                    ..MemberExpr::dummy()
                  }))),
                  args: vec![Box::new(Expr::Ident(array_name.clone())).into()],
                  ..CallExpr::dummy()
                }))),
              }));

              stmts
            },
            ..BlockStmt::dummy()
          })),
          params: vec![Pat::Ident(controller_name.into())],
          ..ArrowExpr::dummy()
        });

        Expr::Call(CallExpr {
          callee: Callee::Expr(Box::new(Expr::Paren(ParenExpr {
            expr: Box::new(Expr::Arrow(ArrowExpr {
              body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                stmts: vec![
                  Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    kind: VarDeclKind::Const,
                    declare: false,
                    decls: vec![VarDeclarator {
                      name: Pat::Ident(self.later_create_ident.clone().into()),
                      init: Some(Box::new(Expr::Array(ArrayLit::dummy()))),
                      ..VarDeclarator::dummy()
                    }],
                    ..VarDecl::dummy()
                  }))),
                  Stmt::Return(ReturnStmt {
                    span: Span::dummy(),
                    arg: Some(Box::new(Expr::Array(ArrayLit {
                      elems: vec![
                        Some(ExprOrSpread::from(first)),
                        Some(ExprOrSpread::from(call)),
                      ],
                      ..ArrayLit::dummy()
                    }))),
                  }),
                ],
                ..BlockStmt::dummy()
              })),
              ..ArrowExpr::dummy()
            })),
            ..ParenExpr::dummy()
          }))),
          ..CallExpr::dummy()
        })
      });
    }
  }

  fn visit_mut_arrow_expr(&mut self, arrow: &mut swc_ecma_ast::ArrowExpr) {
    self.function_variable_types.push(IsVariableJsxMap::new());

    let return_type = if let Some(expr) = arrow.body.as_expr() {
      Some(self.get_expr_type(expr))
    } else {
      None
    };

    arrow.visit_mut_children_with(self);

    if let Some(return_type) = return_type {
      self.return_type = return_type;
    }

    if arrow.is_async {
      self.return_type = self.return_type.awaited()
    };
    self.last_arrow_function_return_type = self.return_type;

    self.function_variable_types.pop();
  }

  fn visit_mut_assign_expr(&mut self, assign: &mut swc_ecma_ast::AssignExpr) {
    assign.visit_mut_children_with(self);

    let is_jsx = self.get_expr_type(&assign.right);
    if let Some(last) = self.function_variable_types.last_mut() {
      match &assign.left {
        AssignTarget::Simple(simple) => match simple {
          SimpleAssignTarget::Ident(ident) => {
            last.insert(ident.id.sym.as_str().to_owned(), is_jsx);
          }
          _ => {}
        },
        _ => {}
      }
    }
  }

  fn visit_mut_var_declarator(&mut self, declarator: &mut VarDeclarator) {
    declarator.visit_mut_children_with(self);

    if let Some(init) = &declarator.init {
      let is_jsx = self.get_expr_type(init);
      if let Some(last) = self.function_variable_types.last_mut() {
        match &declarator.name {
          Pat::Ident(i) => {
            last.insert(i.id.sym.as_str().to_owned(), is_jsx);
          }
          _ => {}
        }
      }
    }
  }

  fn visit_mut_return_stmt(&mut self, ret: &mut ReturnStmt) {
    let Some(arg) = &ret.arg else {
      ret.visit_mut_children_with(self);
      return;
    };

    self.return_type = self.get_expr_type(arg);

    ret.visit_mut_children_with(self);
  }

  fn visit_mut_fn_decl(&mut self, decl: &mut swc_ecma_ast::FnDecl) {
    self.function_variable_types.push(IsVariableJsxMap::new());
    decl.visit_mut_children_with(self);
    self.function_variable_types.pop();

    if let Some(last) = self.function_variable_types.last_mut() {
      if decl.function.is_async {
        self.return_type = self.return_type.awaited()
      };
      last.insert(decl.ident.sym.as_str().to_owned(), self.return_type.clone());
    }
  }
}

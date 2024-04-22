use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::{
  commands::common::{impl_typecheck_visits, TypecheckerCommon},
  tpl_wrapper::TplWrapper,
  utils::{self, stringify::Stringify},
};
use phf::phf_map;
use swc_common::{util::take::Take, Span};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{
  ArrayLit, ArrayPat, ArrowExpr, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, Decl, Expr,
  ExprOrSpread, ExprStmt, Ident, JSXAttrOrSpread, JSXAttrValue, JSXElement, JSXElementName,
  JSXExpr, JSXMemberExpr, JSXObject, KeyValueProp, Lit, MemberExpr, MemberProp, ObjectLit,
  ParenExpr, Pat, Prop, PropName, PropOrSpread, Regex, ReturnStmt, Stmt, VarDecl, VarDeclKind,
  VarDeclarator,
};

pub struct TranspileVisitor<'a> {
  pub typechecker: TypecheckerCommon<'a, Self>,
}

impl TranspileVisitor<'_> {
  pub fn new(compiler: &'_ swc::Compiler) -> Rc<RefCell<Box<TranspileVisitor<'_>>>> {
    let transpiler = Rc::new(RefCell::new(Box::new(TranspileVisitor {
      typechecker: TypecheckerCommon::new(compiler),
    })));

    let transpiler_clone = std::rc::Rc::clone(&transpiler);

    (&mut (*transpiler).borrow_mut())
      .typechecker
      .set_parent(transpiler_clone);

    // let a = *(transpiler.as_ref());

    // return a;

    // return A(transpiler);
    return transpiler;
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
  let compiler = &v.typechecker.compiler;
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
    println!("HERE! {n:?}");
    n.visit_mut_children_with(self);

    if let Expr::JSXElement(_) = n {
      println!("REPLACE!");
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
                        obj: Box::new(Expr::Ident(
                          self.typechecker.later_create_ident.clone().into(),
                        )),
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
                      name: Pat::Ident(self.typechecker.later_create_ident.clone().into()),
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

  impl_typecheck_visits!();
}

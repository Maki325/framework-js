use crate::{
  transpiler::{self, ToCreateAsync, TranspileVisitor},
  utils::{self, stringify::Stringify},
};
use swc_common::util::take::Take;
use swc_ecma_ast::{
  CallExpr, Callee, Expr, JSXElementChild, JSXExpr, Lit, MemberExpr, Tpl, TplElement,
};

pub struct TplWrapper {
  pub exprs: Vec<Box<Expr>>,
  pub quasis: Vec<TplElement>,
  pub is_expr_next: bool,
}

impl TplWrapper {
  pub fn new() -> TplWrapper {
    return TplWrapper {
      exprs: vec![],
      quasis: vec![],
      is_expr_next: false,
    };
  }

  pub fn append_lit(&mut self, lit: Lit) {
    self.append_quasi(lit.stringify());
  }

  pub fn append_expr(&mut self, expr: Expr) {
    let expr = match expr {
      Expr::Lit(lit) => {
        self.append_lit(lit);
        return;
      }
      Expr::Tpl(tpl) => {
        self.append_tpl(tpl);
        return;
      }
      _ => Box::new(expr),
    };

    if self.is_expr_next {
      self.exprs.push(expr);
      self.is_expr_next = false;
    } else {
      self.quasis.push(TplElement {
        raw: "".into(),
        ..TplElement::dummy()
      });
      self.exprs.push(expr);
      self.is_expr_next = false;
    }
  }

  pub fn append_quasi<S: AsRef<str>>(&mut self, quasi: S) {
    if self.is_expr_next {
      let last = self.quasis.pop().unwrap();

      let mut string = last.raw.as_str().to_owned();
      string.push_str(quasi.as_ref());
      self.quasis.push(TplElement {
        raw: string.into(),
        ..TplElement::dummy()
      });

      self.is_expr_next = true;
    } else {
      self.quasis.push(TplElement {
        raw: quasi.as_ref().into(),
        ..TplElement::dummy()
      });
      self.is_expr_next = true;
    }
  }

  pub fn append_tpl<TPL: DestructTpl>(&mut self, tpl: TPL) {
    let (mut quasis, mut exprs) = tpl.get_quasis_and_exprs();

    exprs.reverse();
    quasis.reverse();
    let mut take_quasi = true;
    loop {
      if take_quasi {
        let quasi = quasis.pop();
        match quasi {
          None => return,
          Some(quasi) => self.append_quasi(quasi.raw.as_str()),
        }
        take_quasi = false;
      } else {
        let expr = exprs.pop();
        match expr {
          None => return,
          Some(expr) => self.append_expr(*expr),
        }
        take_quasi = true;
      }
    }
  }

  pub fn append_element_child(
    &mut self,
    v: &TranspileVisitor,
    element: JSXElementChild,
    to_create: &mut ToCreateAsync,
  ) {
    match element {
      JSXElementChild::JSXElement(el) => {
        match utils::process_transformed_jsx(transpiler::transform(v, el, to_create), v, to_create)
        {
          utils::Processed::Async(div) => self.append_quasi(div),
          utils::Processed::Sync(transformed) => self.append_expr(transformed),
        };
      }
      JSXElementChild::JSXExprContainer(container) => {
        let JSXExpr::Expr(expr) = container.expr else {
          return;
        };

        let expr = match *expr {
          Expr::Array(_) => {
            self.append_expr(Expr::Call(CallExpr {
              args: vec![Expr::Lit(Lit::Str("".into())).into()],
              callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                obj: expr,
                prop: swc_ecma_ast::MemberProp::Ident("join".into()),
                ..MemberExpr::dummy()
              }))),
              ..CallExpr::dummy()
            }));
            return;
          }
          Expr::Lit(lit) => {
            self.append_lit(lit);
            return;
          }
          Expr::Object(_) => unimplemented!("Objects are not valid as a JSX child!"),
          _ => expr,
        };

        let expr = utils::call_framework_stringify(expr, v.typechecker.later_create_ident.clone());

        self.append_expr(expr);
      }
      JSXElementChild::JSXFragment(f) => {
        for child in f.children {
          self.append_element_child(v, child, to_create);
        }
      }
      JSXElementChild::JSXSpreadChild(sc) => {
        self.append_expr(*sc.expr);
      }
      JSXElementChild::JSXText(text) => {
        self.append_quasi(text.value.as_str().to_owned());
      }
    }
  }

  pub fn build(mut self) -> Tpl {
    if self.is_expr_next == false {
      self.quasis.push(TplElement {
        tail: true,
        ..TplElement::dummy()
      });
    } else {
      if let Some(quasi) = self.quasis.last_mut() {
        quasi.tail = true;
      }
    }

    return Tpl {
      exprs: self.exprs,
      quasis: self.quasis,
      ..Tpl::dummy()
    };
  }
}

pub trait DestructTpl {
  fn get_quasis_and_exprs(self) -> (Vec<TplElement>, Vec<Box<Expr>>);
}

macro_rules! impl_destruct_tpl {
  ($($name:ident),+) => {
    $(
      impl DestructTpl for $name {
        fn get_quasis_and_exprs(self) -> (Vec<TplElement>, Vec<Box<Expr>>) {
          return (self.quasis, self.exprs);
        }
      }
    )+
  };
}

impl_destruct_tpl!(TplWrapper, Tpl);

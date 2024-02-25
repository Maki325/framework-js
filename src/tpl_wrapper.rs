use crate::run::transform;
use swc_common::util::take::Take;
use swc_ecma_ast::{Expr, JSXElementChild, JSXExpr, Lit, Tpl, TplElement};

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

  pub fn append_expr(&mut self, expr: Expr) {
    let expr = match expr {
      Expr::Lit(ref lit) => {
        let string = match lit {
          Lit::Str(value) => Some(value.value.as_str().to_owned()),
          Lit::Bool(value) => Some(value.value.to_string()),
          Lit::Null(_) => Some("null".to_owned()),
          Lit::Num(value) => Some(value.value.to_string()),
          Lit::BigInt(value) => Some(value.value.to_string()),
          Lit::Regex(_) => None,
          Lit::JSXText(value) => Some(value.value.as_str().to_owned()),
        };
        if let Some(string) = string {
          self.append_quasi(string);
          return;
        }
        Box::new(expr)
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

  pub fn append_element_child(&mut self, element: JSXElementChild) {
    match element {
      JSXElementChild::JSXElement(el) => {
        let transformed = transform(el);

        self.append_expr(transformed);
      }
      JSXElementChild::JSXExprContainer(container) => {
        if let JSXExpr::Expr(expr) = container.expr {
          self.append_expr(*expr);
        }
      }
      JSXElementChild::JSXFragment(f) => {
        for child in f.children {
          self.append_element_child(child);
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

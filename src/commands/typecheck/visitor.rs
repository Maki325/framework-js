use crate::{
  specs::type_info::{ExportType, Exports, DEFAULT_EXPORT_KEY},
  utils::{self, stringify::Stringify},
};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{
  AssignTarget, BlockStmtOrExpr, Callee, Decl, DefaultDecl, Expr, Ident, Pat, ReturnStmt,
  SimpleAssignTarget, VarDeclarator,
};

pub struct TypecheckerVisitor<'a> {
  #[allow(unused)]
  pub compiler: &'a swc::Compiler,

  return_type: ExportType,
  pub later_create_ident: Ident,

  pub function_variable_types: Vec<Exports>,
  exports: &'a mut Exports,
  last_arrow_function_return_type: ExportType,
}

impl<'a> TypecheckerVisitor<'a> {
  pub fn new(compiler: &'a swc::Compiler, exports: &'a mut Exports) -> TypecheckerVisitor<'a> {
    return TypecheckerVisitor {
      compiler,

      return_type: ExportType::Other,
      later_create_ident: utils::generate_random_variable_name(16).as_str().into(),

      function_variable_types: vec![Exports::new()],
      exports,
      last_arrow_function_return_type: ExportType::Other,
    };
  }

  fn is_ident_jsx<S: AsRef<str>>(&self, name: S) -> ExportType {
    for map in self.function_variable_types.iter().rev() {
      match map.get(name.as_ref()) {
        None => continue,
        Some(value) => return *value,
      }
    }

    return ExportType::Other;
  }

  fn get_expr_type<E: AsRef<Expr>>(&self, to_assign_expr: E) -> ExportType {
    match to_assign_expr.as_ref() {
      Expr::JSXElement(_) | Expr::JSXFragment(_) => return ExportType::JSX,
      Expr::Assign(assign) => self.get_expr_type(&assign.right),
      Expr::Await(expr) => self.get_expr_type(&expr.arg).awaited(),
      Expr::Call(call) => match &call.callee {
        Callee::Expr(e) => self.get_expr_type(e),
        _ => ExportType::Other,
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
      _ => return ExportType::Other,
    }
  }

  pub fn get_variable_type<S: AsRef<str>>(&self, name: S) -> Option<ExportType> {
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

impl<'a> VisitMut for TypecheckerVisitor<'a> {
  fn visit_mut_arrow_expr(&mut self, arrow: &mut swc_ecma_ast::ArrowExpr) {
    self.function_variable_types.push(Exports::new());

    let return_type = match &*arrow.body {
      BlockStmtOrExpr::Expr(expr) => Some(self.get_expr_type(expr)),
      BlockStmtOrExpr::BlockStmt(block) => {
        if !block.stmts.iter().any(|s| s.is_return_stmt()) {
          Some(ExportType::Other)
        } else {
          None
        }
      }
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
    self.function_variable_types.push(Exports::new());
    decl.visit_mut_children_with(self);
    self.function_variable_types.pop();

    if let Some(body) = &decl.function.body {
      if !body.stmts.iter().any(|stmt| stmt.is_return_stmt()) {
        self.return_type = ExportType::Other;
      }
    }

    if let Some(last) = self.function_variable_types.last_mut() {
      if decl.function.is_async {
        self.return_type = self.return_type.awaited()
      };
      last.insert(decl.ident.sym.as_str().to_owned(), self.return_type.clone());
    }
  }

  fn visit_mut_export_all(&mut self, export_all: &mut swc_ecma_ast::ExportAll) {
    export_all.visit_mut_children_with(self);
    unimplemented!("ExportAll");
  }

  fn visit_mut_export_decl(&mut self, export_decl: &mut swc_ecma_ast::ExportDecl) {
    export_decl.visit_mut_children_with(self);
    let names: Vec<String> = match &export_decl.decl {
      Decl::Class(c) => vec![c.ident.clone().stringify()],
      Decl::Fn(f) => vec![f.ident.clone().stringify()],
      Decl::Var(v) => v
        .decls
        .iter()
        .filter_map(|decl| match &decl.name {
          Pat::Ident(i) => Some(i.id.clone().stringify()),
          _ => None,
        })
        .collect(),
      Decl::TsEnum(_) | Decl::TsInterface(_) | Decl::TsModule(_) | Decl::TsTypeAlias(_) => return,
      // New thing, most likely not JSX
      Decl::Using(_) => return,
    };

    for name in names {
      let value = self.get_variable_type(&name).unwrap_or(ExportType::Other);
      self.exports.insert(name, value);
    }
  }

  fn visit_mut_export_default_decl(&mut self, default: &mut swc_ecma_ast::ExportDefaultDecl) {
    default.visit_mut_children_with(self);

    let name = match &default.decl {
      DefaultDecl::Class(c) => match &c.ident {
        Some(i) => i.clone().stringify(),
        None => return,
      },
      DefaultDecl::Fn(f) => match &f.ident {
        Some(i) => i.clone().stringify(),
        None => {
          self
            .exports
            .insert(DEFAULT_EXPORT_KEY.to_string(), self.return_type);
          return;
        }
      },
      DefaultDecl::TsInterfaceDecl(_) => return,
    };

    let value = self.get_variable_type(&name).unwrap_or(ExportType::Other);
    self.exports.insert(name, value);
  }

  fn visit_mut_export_default_expr(&mut self, _default_expr: &mut swc_ecma_ast::ExportDefaultExpr) {
    unimplemented!();
  }
}

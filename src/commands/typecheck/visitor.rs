use super::common::{impl_typecheck_visits, TypecheckerCommon};
use crate::{
  specs::type_info::{ExportType, Exports, DEFAULT_EXPORT_KEY},
  utils::stringify::Stringify,
};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{Decl, DefaultDecl, Pat, ReturnStmt, VarDeclarator};

pub struct TypecheckerVisitor<'a> {
  typechecker: TypecheckerCommon<'a, Self>,
  exports: &'a mut Exports,
}

impl TypecheckerVisitor<'_> {
  pub fn new<'a>(
    compiler: &'a swc::Compiler,
    exports: &'a mut Exports,
  ) -> &'a mut TypecheckerVisitor<'a> {
    let transpiler = Box::new(TypecheckerVisitor {
      typechecker: TypecheckerCommon::new(compiler),
      exports,
    });

    let transpiler = Box::leak(transpiler);
    let ptr: *mut TypecheckerVisitor<'a> = transpiler;
    transpiler.typechecker.set_parent(ptr);
    return transpiler;
  }
}

impl<'a> VisitMut for TypecheckerVisitor<'a> {
  impl_typecheck_visits!();

  fn visit_mut_export_all(&mut self, export_all: &mut swc_ecma_ast::ExportAll) {
    export_all.visit_mut_children_with(self);
    unimplemented!("ExportAll: `export * from 'file.js';`");
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
      let value = self
        .typechecker
        .get_variable_type(&name)
        .unwrap_or(ExportType::Other);
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
            .insert(DEFAULT_EXPORT_KEY.to_string(), self.typechecker.return_type);
          return;
        }
      },
      DefaultDecl::TsInterfaceDecl(_) => return,
    };

    let value = self
      .typechecker
      .get_variable_type(&name)
      .unwrap_or(ExportType::Other);
    self.exports.insert(name, value);
  }

  fn visit_mut_export_default_expr(&mut self, _default_expr: &mut swc_ecma_ast::ExportDefaultExpr) {
    unimplemented!();
  }
}

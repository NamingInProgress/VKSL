// use crate::ast::expr::Expr;
// use crate::ast::stmt::{MethodDeclStmt, Stmt, StructStmt};
// use crate::ast::ty::Type;
// use crate::ast::{Ast, Ident};
// use crate::parser;
// use crate::parser::err::{ParseErr, ParseErrType};
// use crate::scope::{FnRef, InputRef, OutputRef, ProvideRef, PushConstRef, StructRef, Symbol, SymbolId, TupleRef, UniformRef, VarRef};
// use mvutils::enum_val;
// use mvutils::utils::IncDec;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::ops::DerefMut;
// use std::rc::Rc;
// use std::{iter, mem};
//
// #[derive(Clone, Debug)]
// pub struct Scope {
//     pub parent: Option<SharedScope>,
//     pub symbols: HashMap<String, Symbol>
// }
//
// impl Scope {
//     pub fn new(parent: Option<SharedScope>) -> SharedScope {
//         Rc::new(RefCell::new(Self {
//             parent,
//             symbols: HashMap::new()
//         }))
//     }
//
//     pub fn lookup_sym(&self, str: &String) -> Option<&Symbol> {
//         let sym = self.symbols.get(str);
//         match sym {
//             None => {
//                 match &self.parent {
//                     None => None,
//                     Some(parent) => {
//                         let guard = parent.borrow();
//                         guard.lookup_sym(str).map(|sym| {
//                             unsafe { mem::transmute::<&Symbol, &'static Symbol>(sym) }
//                         })
//                     }
//                 }
//             }
//             Some(sym) => Some(sym)
//         }
//     }
//
//     pub fn lookup_id(&self, str: &String) -> Option<SymbolId> {
//         self.lookup_sym(str).map(|sym| sym.get_id())
//     }
// }
//
// pub type SharedScope = Rc<RefCell<Scope>>;
//
// pub struct ScopeResolver {
//     pub scopes: Vec<SharedScope>,
//     second_pass: bool,
//     ast: Option<Ast>,
//     idx: usize,
// }
//
// const COUNTER_ID: &str = "SymbolId";
//
// type Result<T> = parser::Result<T>;
// type NoResult = parser::Result<()>;
//
// impl ScopeResolver {
//     pub fn new(ast: Ast) -> Self {
//         let s = Self {
//             scopes: vec![],
//             second_pass: false,
//             ast: Some(ast),
//             idx: 0,
//         };
//
//         s
//     }
//
//     pub fn parse(mut self) -> Result<Ast> {
//         let mut ast = self.ast.take().expect("is here");
//         self.handle_scope(ast.iter_mut(), None)?;
//         self.second_pass = true;
//         self.handle_scope(ast.iter_mut(), None)?;
//         Ok(ast)
//     }
//
//     pub fn handle_scope<I: Iterator<Item=S>, S: DerefMut<Target=Stmt>>(&mut self, stmts: I, parent: Option<SharedScope>) -> NoResult {
//         self.handle_scope_init::<I, S, fn(SharedScope) -> NoResult>(stmts, parent, None)
//     }
//
//     pub fn handle_scope_init<I: Iterator<Item=S>, S: DerefMut<Target=Stmt>, F: FnMut(SharedScope) -> NoResult>(&mut self, mut stmts: I, parent: Option<SharedScope>, init: Option<F>) -> NoResult {
//         let scope = if self.second_pass {
//             self.scopes[self.idx.inc() - 1].clone()
//         } else {
//             Scope::new(parent)
//         };
//
//         if let Some(mut init) = init {
//             init(scope.clone())?;
//         }
//
//         if !self.second_pass {
//             self.scopes.push(scope.clone());
//         }
//
//         while let Some(stmt) = stmts.next().as_deref_mut() {
//             match stmt {
//                 Stmt::Compound(inner_stmts) => {
//                     for stmt in &mut inner_stmts.components {
//                         self.handle_stmt(stmt, &scope)?;
//                     }
//                 }
//                 _ => self.handle_stmt(stmt, &scope)?,
//             }
//         }
//
//         Ok(())
//     }
//
//     fn handle_ident<F: Fn(SymbolId) -> Symbol>(second_pass: bool, ident: &mut Ident, current_scope: &SharedScope, generator: F) -> Result<(SymbolId, &'static Symbol)> {
//         let mut guard = current_scope.borrow_mut();
//         let sym = guard.lookup_sym(&ident.val);
//         if second_pass {
//             match sym {
//                 None => {
//                     Err(ParseErr {
//                         ty: ParseErrType::UnknownIdent(ident.val.clone()),
//                         ctx: ident.tkn.clone(),
//                         tail: "".to_string(),
//                         hint: None,
//                     })
//                 }
//                 Some(sym) => {
//                     let sym = unsafe { mem::transmute::<&Symbol, &'static Symbol>(sym) };
//                     ident.resolved_ident = Some(sym.get_id());
//                     Ok((sym.get_id(), sym))
//                 }
//             }
//         } else {
//             let id = mvutils::utils::next_id(COUNTER_ID);
//             guard.symbols.insert(ident.val.clone(), generator(id));
//             let sym = guard.symbols.get(&ident.val).expect("Look above");
//             let sym = unsafe { mem::transmute::<&Symbol, &'static Symbol>(sym) };
//             ident.resolved_ident = Some(id);
//             Ok((id, sym))
//         }
//     }
//
//     fn handle_type(&mut self, ty: &mut Type, current_scope: &SharedScope) -> Result<Option<SymbolId>> {
//         match ty {
//             Type::Primitive(_, _) => Ok(None),
//             Type::SingleType(st) => {
//                 if let Some(sym) = current_scope.borrow().lookup_sym(&st.name) {
//                     return match sym {
//                         Symbol::Struct(r) => {
//                             st.resolved_name = Some(r.sym);
//                             Ok(Some(r.sym))
//                         },
//                         _ => {
//                             Err(ParseErr {
//                                 ty: ParseErrType::NotATypeSymbol(st.name.clone()),
//                                 ctx: st.tkn.clone(),
//                                 tail: "".to_string(),
//                                 hint: None,
//                             })
//                         }
//                     };
//                 }
//                 if self.second_pass {
//                     return Err(ParseErr {
//                         ty: ParseErrType::UnknownType(st.name.clone()),
//                         ctx: st.tkn.clone(),
//                         tail: "".to_string(),
//                         hint: None,
//                     });
//                 }
//                 Ok(None)
//             }
//             Type::PathType(_) => todo!(),
//             Type::ArrayOf(arr) => {
//                 if let Some(dim_expr) = &mut arr.dimension {
//                     self.handle_expr(dim_expr, current_scope)?;
//                 }
//                 self.handle_type(&mut arr.component, current_scope)
//             }
//             Type::TupleOf(tp) => {
//                 let sym_id = mvutils::utils::next_id(COUNTER_ID);
//                 let mut name = String::new();
//                 for (typ, _) in &mut tp.types {
//                     self.handle_type(typ, current_scope)?;
//                     let s = format!("{typ}_");
//                     name.push_str(&s);
//                 }
//
//                 current_scope.borrow_mut().symbols.insert(format!("+tuple_{name}"), Symbol::Tuple(TupleRef(sym_id)));
//                 Ok(Some(sym_id))
//             }
//             Type::StructDef(st) => {
//                 self.handle_struct_def(&mut st.inner, current_scope)
//             }
//         }
//     }
//
//     fn handle_expr(&mut self, expr: &mut Expr, current_scope: &SharedScope) -> NoResult {
//         match expr {
//             Expr::Unary(un_op) => self.handle_expr(&mut un_op.expr, current_scope)?,
//             Expr::Binary(bin_op) => {
//                 self.handle_expr(&mut bin_op.lhs, current_scope)?;
//                 self.handle_expr(&mut bin_op.rhs, current_scope)?;
//             }
//             Expr::FnCall(fn_call) => {
//                 Self::handle_ident(true, &mut fn_call.ident, current_scope, |id| Symbol::Variable(VarRef(id)))?;
//                 for (arg, _) in fn_call.args.iter_mut() {
//                     self.handle_expr(arg, current_scope)?;
//                 }
//             }
//             Expr::Access(acc) => {
//                 self.handle_expr(&mut acc.parent, current_scope)?;
//             }
//             Expr::Variable(vr) => {
//                 Self::handle_ident(true, &mut vr.ident, current_scope, |id| Symbol::Variable(VarRef(id)))?;
//             }
//             Expr::Index(idx) => {
//                 self.handle_expr(&mut idx.array, current_scope)?;
//                 self.handle_expr(&mut idx.index, current_scope)?;
//             }
//             Expr::Ternary(tr) => {
//                 self.handle_expr(&mut tr.cond, current_scope)?;
//                 self.handle_expr(&mut tr.yes, current_scope)?;
//                 self.handle_expr(&mut tr.no, current_scope)?;
//             }
//             Expr::PreFix(pf) => {
//                 self.handle_expr(&mut pf.expr, current_scope)?;
//             }
//             Expr::PostFix(pf) => {
//                 self.handle_expr(&mut pf.expr, current_scope)?;
//             }
//             Expr::Tuple(tp) => {
//                 for (arg, _) in tp.args.iter_mut() {
//                     self.handle_expr(arg, current_scope)?;
//                 }
//             }
//             Expr::Array(ar) => {
//                 for (arg, _) in ar.args.iter_mut() {
//                     self.handle_expr(arg, current_scope)?;
//                 }
//             }
//             Expr::Block(bck) => {
//                 self.handle_scope(
//                     bck.block.iter_mut(),
//                     Some(current_scope.clone())
//                 )?;
//             }
//             Expr::Assign(ass) => {
//                 self.handle_expr(&mut ass.lhs, current_scope)?;
//                 self.handle_expr(&mut ass.rhs, current_scope)?;
//             }
//             Expr::Nonuniform(u) => {
//                 self.handle_expr(&mut u.expr, current_scope)?;
//             }
//             Expr::Literal(_) => {}
//         }
//         Ok(())
//     }
//
//     fn handle_stmt(&mut self, stmt: &mut Stmt, current_scope: &SharedScope) -> NoResult {
//         match stmt {
//             Stmt::If(if_stmt) => {
//                 self.handle_expr(&mut if_stmt.cond, current_scope)?;
//                 self.handle_scope(iter::once(&mut *if_stmt.branch), Some(current_scope.clone()))?;
//                 if let Some(else_br) = &mut if_stmt.else_branch {
//                     self.handle_scope(iter::once(&mut **else_br), Some(current_scope.clone()))?;
//                 }
//             }
//             Stmt::For(for_stmt) => {
//                 if let Some(cond) = &mut for_stmt.cond { self.handle_expr(cond, current_scope)?; }
//                 if let Some(run) = &mut for_stmt.after_run { self.handle_expr(run, current_scope)?; }
//                 if let Some(cond) = &mut for_stmt.start_cond { self.handle_expr(cond, current_scope)?; }
//
//                 self.handle_scope(iter::once(&mut *for_stmt.block), Some(current_scope.clone()))?;
//             }
//             Stmt::While(while_stmt) => {
//                 self.handle_expr(&mut while_stmt.cond, current_scope)?;
//
//                 self.handle_scope(iter::once(&mut *while_stmt.block), Some(current_scope.clone()))?;
//             }
//             Stmt::MethodDecl(meth_decl) => {
//                 Self::handle_ident(self.second_pass, &mut meth_decl.name, current_scope, |id| Symbol::Function(FnRef(id)))?;
//                 self.handle_meth_def(meth_decl, current_scope)?;
//             }
//             Stmt::VarDecl(var_decl) => {
//                 Self::handle_ident(self.second_pass, &mut var_decl.name, current_scope, |id| Symbol::Function(FnRef(id)))?;
//                 if let Some(ret_type) = &mut var_decl.ty {
//                     self.handle_type(ret_type, current_scope)?;
//                 }
//                 if let Some(init) = &mut var_decl.init {
//                     self.handle_expr(init, current_scope)?;
//                 }
//             }
//             Stmt::Return(ret_stmt) => {
//                 if let Some(ret_expr) = &mut ret_stmt.expr {
//                     self.handle_expr(ret_expr, current_scope)?;
//                 }
//             }
//             Stmt::Yield(yield_stmt) => {
//                 self.handle_expr(&mut yield_stmt.expr, current_scope)?;
//             }
//             Stmt::Break(_) => {}
//             Stmt::Continue(_) => {}
//             Stmt::Include(_include_stmt) => {
//                 todo!()
//             }
//             Stmt::Extension(_) => {}
//             Stmt::Input(inp_stmt) => {
//                 Self::handle_ident(self.second_pass, &mut inp_stmt.name, current_scope, |id| Symbol::Input(InputRef(id)))?;
//                 self.handle_type(&mut inp_stmt.ty, current_scope)?;
//             }
//             Stmt::Output(out_stmt) => {
//                 Self::handle_ident(self.second_pass, &mut out_stmt.name, current_scope, |id| Symbol::Output(OutputRef(id)))?;
//                 self.handle_type(&mut out_stmt.ty, current_scope)?;
//             }
//             Stmt::Provide(prv_stmt) => {
//                 Self::handle_ident(self.second_pass, &mut prv_stmt.name, current_scope, |id| Symbol::Provide(ProvideRef(id)))?;
//                 self.handle_type(&mut prv_stmt.ty, current_scope)?;
//             }
//             Stmt::PushConstants(pc_stmt) => {
//                 Self::handle_ident(self.second_pass, &mut pc_stmt.name, current_scope, |id| Symbol::PushConstant(PushConstRef(id)))?;
//                 self.handle_type(&mut pc_stmt.ty, current_scope)?;
//             }
//             Stmt::Uniform(uni) => {
//                 Self::handle_ident(self.second_pass, &mut uni.name, current_scope, |id| Symbol::Uniform(UniformRef(id)))?;
//             }
//             Stmt::Struct(struct_stmt) => {
//                 self.handle_struct_def(struct_stmt, current_scope)?;
//             }
//             Stmt::Block(b_stmt) => {
//                 self.handle_scope(b_stmt.stmts.iter_mut(), Some(current_scope.clone()))?;
//             }
//             Stmt::Expr(expr) => {
//                 self.handle_expr(&mut expr.expr, current_scope)?;
//             }
//             Stmt::Compound(inner_stmts) => {
//                 for stmt in &mut inner_stmts.components {
//                     self.handle_stmt(stmt, current_scope)?;
//                 }
//             }
//             Stmt::Semi(_) => {}
//         }
//         Ok(())
//     }
//
//     fn handle_struct_def(&mut self, stmt: &mut StructStmt, current_scope: &SharedScope) -> Result<Option<SymbolId>> {
//         let (id, sym) = Self::handle_ident(self.second_pass, &mut stmt.name, current_scope, |id| Symbol::Struct(StructRef { sym: id, internal_scope: Scope::new(None) }))?;
//         let struct_sym = enum_val!(Symbol, sym, Struct);
//         let struct_scope = &struct_sym.internal_scope;
//
//         for field in &mut stmt.fields {
//             self.handle_type(&mut field.ty, current_scope)?;
//             Self::handle_ident(self.second_pass, &mut field.name, struct_scope, |id| Symbol::Variable(VarRef(id)))?;
//         }
//         for method in &mut stmt.methods {
//             self.handle_meth_def(method, current_scope)?;
//             Self::handle_ident(self.second_pass, &mut method.name, struct_scope, |id| Symbol::Function(FnRef(id)))?;
//         }
//
//         Ok(Some(id))
//     }
//
//     fn handle_meth_def(&mut self, meth_decl: &mut MethodDeclStmt, current_scope: &SharedScope) -> NoResult {
//         let second_pass = self.second_pass;
//         let init = Some(|new_scope: SharedScope| {
//             for param in &mut meth_decl.params {
//                 Self::handle_ident(second_pass, &mut param.name, &new_scope, |id| Symbol::Variable(VarRef(id)))?;
//             }
//             Ok(())
//         });
//
//         self.handle_scope_init(iter::once(&mut *meth_decl.block), Some(current_scope.clone()), init)?;
//
//         if let Some(ret_type) = &mut meth_decl.return_type {
//             self.handle_type(ret_type, current_scope)?;
//         }
//         Ok(())
//     }
// }
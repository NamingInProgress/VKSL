use crate::parser;
use crate::parser::ast::{ExprStmt, MethodDeclStmt, StructStmt};
use crate::parser::err::{ParseErr, ParseErrType};
use crate::scope::{ResolvedScope, SharedScope, COUNTER_ID};
use std::collections::hash_map::Entry;

type PAst = parser::ast::Ast;
type PStmt = parser::ast::Stmt;
type PExpr = parser::ast::Expr;
type PType = parser::ast::Type;

type Result<T> = parser::Result<T>;

fn handle_struct(stmt: &mut StructStmt, scope: &SharedScope) -> Result<()> {
    let struct_scope = ResolvedScope::with_parent(scope.clone());
    stmt.fut_scope = Some(struct_scope);

    let name = stmt.name.val.clone();
    let mut guard = scope.borrow_mut();
    let symbol_id = mvutils::utils::next_id(COUNTER_ID);

    stmt.symbol_id = Some(symbol_id);

    let entry = guard.by_name.entry(name.clone());
    match entry {
        Entry::Occupied(_) => {
            let hint = format!("consider removing one of the structs `{name}`");
            let e = ParseErr {
                ty: ParseErrType::DuplicateStruct(name),
                ctx: stmt.name.tkn.clone(),
                hint: Some(hint),
            };
            return Err(e);
        }
        Entry::Vacant(ve) => {
            ve.insert(symbol_id);
        }
    }

    Ok(())
}

fn handle_fn(stmt: &mut MethodDeclStmt, scope: &SharedScope) -> Result<()> {
    let fn_scope = ResolvedScope::with_parent(scope.clone());
    stmt.fut_scope = Some(fn_scope);

    let name = stmt.name.val.clone();
    let mut guard = scope.borrow_mut();
    let symbol_id = mvutils::utils::next_id(COUNTER_ID);

    stmt.symbol_id = Some(symbol_id);

    let entry = guard.by_name.entry(name.clone());
    match entry {
        Entry::Occupied(_) => {
            let hint = format!("consider removing one of the functions `{name}`");
            let e = ParseErr {
                ty: ParseErrType::DuplicateFn(name),
                ctx: stmt.name.tkn.clone(),
                hint: Some(hint),
            };
            return Err(e);
        }
        Entry::Vacant(ve) => {
            ve.insert(symbol_id);
        }
    }

    Ok(())
}

fn handle_type(ty: &mut PType, scope: &SharedScope) -> Result<()> {
    match ty {
        PType::StructDef(sd) => {
            handle_struct(&mut *sd.inner, scope)?;
        }
        PType::ArrayOf(a) => {
            handle_type(&mut a.component, scope)?;
        }
        PType::TupleOf(t) => {
            for (t, _) in &mut t.types {
                handle_type(t, scope)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_ast(ast: &mut PAst) -> Result<SharedScope> {
    let scope = ResolvedScope::global();
    for stmt in ast {
        handle_stmt(stmt, &scope)?;
    }
    Ok(scope)
}

fn handle_stmt(stmt: &mut PStmt, scope: &SharedScope) -> Result<()> {
    match stmt {
        PStmt::For(s) => {
            let for_scope = ResolvedScope::with_parent(scope.clone());
            handle_stmt(&mut s.block, &for_scope)?;
            s.fut_scope = Some(for_scope);
        }
        PStmt::If(s) => {
            handle_stmt(&mut s.branch, scope)?;
            if let Some(else_branch) = &mut s.else_branch {
                handle_stmt(else_branch, scope)?;
            }
        }
        PStmt::While(s) => {
            handle_stmt(&mut s.block, scope)?;
        }
        PStmt::MethodDecl(s) => {
            handle_fn(s, scope)?;
            if let Some(ty) = &mut s.return_type {
                handle_type(ty, scope)?;
            }
            handle_stmt(&mut s.block, scope)?;
        }
        PStmt::VarDecl(s) => {
            if let Some(ty) = &mut s.ty {
                handle_type(ty, scope)?;
            }
        }
        PStmt::Input(s) => {
            handle_type(&mut s.ty, scope)?;
        }
        PStmt::Output(s) => {
            handle_type(&mut s.ty, scope)?;
        }
        PStmt::Provide(s) => {
            handle_type(&mut s.ty, scope)?;
        }
        PStmt::PushConstants(s) => {
            handle_type(&mut s.ty, scope)?;
        }
        PStmt::Uniform(s) => {
            handle_type(&mut s.ty, scope)?;
        }
        PStmt::Struct(s) => {
            handle_struct(s, scope)?;
        }
        PStmt::Block(s) => {
            let new_scope = ResolvedScope::with_parent(scope.clone());
            for s in &mut s.stmts {
                handle_stmt(s, &new_scope)?;
            }
            s.fut_scope = Some(new_scope);
        }
        PStmt::Compound(s) => {
            for s in &mut s.components {
                handle_stmt(s, scope)?;
            }
        }
        PStmt::Expr(e) => {
            handle_expr(&mut e.expr, scope)?;
        }
        PStmt::Return(r) => {
            if let Some(ref mut expr) = r.expr {
                handle_expr(expr, scope)?;
            }
        }
        PStmt::Yield(y) => {
            handle_expr(&mut y.expr, scope)?;
        }
        PStmt::Break(_) | PStmt::Continue(_) | PStmt::Extension(_) | PStmt::Semi(_) => {
            // we can ignore this!!!
        }
        PStmt::Include(_) => todo!(),
    };
    Ok(())
}

fn handle_expr(expr: &mut PExpr, scope: &SharedScope) -> Result<()> {
    match expr {
        PExpr::Unary(e) => {
            handle_expr(&mut e.expr, scope)?;
        }
        PExpr::Binary(e) => {
            handle_expr(&mut e.lhs, scope)?;
            handle_expr(&mut e.rhs, scope)?;
        }
        PExpr::FnCall(e) => {
            for (e, _) in &mut e.args {
                handle_expr(e, scope)?;
            }
        }
        PExpr::Access(e) => {
            handle_expr(&mut e.parent, scope)?;
        }
        PExpr::Index(e) => {
            handle_expr(&mut e.array, scope)?;
            handle_expr(&mut e.index, scope)?;
        }
        PExpr::Ternary(e) => {
            handle_expr(&mut e.yes, scope)?;
            handle_expr(&mut e.no, scope)?;
            handle_expr(&mut e.cond, scope)?;
        }
        PExpr::PreFix(e) => {
            handle_expr(&mut e.expr, scope)?;
        }
        PExpr::PostFix(e) => {
            handle_expr(&mut e.expr, scope)?;
        }
        PExpr::Tuple(e) => {
            for (e, _) in &mut e.args {
                handle_expr(e, scope)?;
            }
        }
        PExpr::Array(e) => {
            for (e, _) in &mut e.args {
                handle_expr(e, scope)?;
            }
        }
        PExpr::Block(e) => {
            let new_scope = ResolvedScope::with_parent(scope.clone());
            for s in &mut e.block {
                handle_stmt(s, &new_scope)?;
            }
            e.fut_scope = Some(new_scope);
        }
        PExpr::Assign(e) => {
            handle_expr(&mut e.lhs, scope)?;
            handle_expr(&mut e.rhs, scope)?;
        }
        PExpr::Nonuniform(e) => {
            handle_expr(&mut e.expr, scope)?;
        }
        PExpr::TupleAccess(e) => {
            handle_expr(&mut e.tuple, scope)?;
        }
        PExpr::AccessFnCall(e) => {
            handle_expr(&mut e.parent, scope)?;
            for (e, _) in &mut e.args {
                handle_expr(e, scope)?;
            }
        }
        PExpr::Construct(e) => {
            for (e, _) in &mut e.fields {
                handle_expr(&mut e.expr, scope)?;
            }
        }
        PExpr::This(_) | PExpr::Literal(_) | PExpr::Variable(_) => {
            //YOOO WE CAN SKIP NO WORK HERE
        }
        PExpr::As(e) => {
            handle_type(&mut e.ty, scope)?;
        }
    }

    Ok(())
}
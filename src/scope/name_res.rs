use crate::{critical_error, parser};
use crate::parser::mods::ResMods;
use crate::scope::ast::expr::{AccessExpr, AccessFnCallExpr, ArrayExpr, AsExpr, AssignExpr, BinExpr, BlockExpr, ConstructExpr, ConstructorExpr, Expr, FieldInit, FnCallExpr, IndexExpr, LitExpr, NonuniformExpr, PostFixExpr, PreFixExpr, TernaryExpr, ThisExpr, TupleAccessExpr, TupleExpr, UnaryExpr, VarExpr};
use crate::scope::ast::stmt::{BlockStmt, BreakStmt, CompoundStmt, ConstDeclStmt, ContinueStmt, ExprStmt, ExtensionStmt, ForStmt, IfStmt, InputStmt, MethodDeclStmt, OutputStmt, ProvideStmt, PushConstantsStmt, ReturnStmt, SemiStmt, Stmt, StructStmt, UniformStmt, VarDeclStmt, WhileStmt, YieldStmt};
use crate::scope::ast::ty::{ArrayType, PrimitiveRef, StructDef, StructRef, TupleDef, Type};
use crate::scope::ast::{Ast, Ident};
use crate::scope::{FieldSym, FnParamSym, FnSym, ResolvedScope, SharedScope, StructSym, Symbol, SymbolId, SymbolSpecies, UniformSym, VarSym, COUNTER_ID};
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::rc::Rc;
use mvutils::enum_val_ref;

pub type Result<T> = parser::Result<T>;
pub type NoResult = parser::Result<()>;

type PStmt = parser::ast::Stmt;
type PExpr = parser::ast::Expr;
type PIdent = parser::ast::Ident;
type PType = parser::ast::Type;

pub fn conv_ast(ast: parser::ast::Ast, global_scope: SharedScope) -> Result<Vec<Stmt>> {
    let mut new_ast = vec![];

    for s in ast {
        let c = conv_stmt(s, &global_scope)?;
        new_ast.push(c);
    }
    Ok(new_ast)
}

fn make_var(ident: PIdent, current_scope: &SharedScope, stmt: parser::ast::VarDeclStmt) -> Result<SymbolId> {
    make_symbol(ident, current_scope, move |id| Ok(Symbol::Variable(VarSym {
        id,
        kw_tkn: stmt.kw_tkn,
        name: Ident::convert(stmt.name, id),
        colon_tkn: stmt.colon_tkn,
        ty: conv_type_opt(stmt.ty, current_scope)?,
        eq_tkn: stmt.eq_tkn,
        init: conv_expr_opt(stmt.init, current_scope)?,
        mods: ResMods::empty(),
        semi_tkn: stmt.semi_tkn,
        cnst: false,
    })))
}

pub fn make_fn(symbol: SymbolId, current_scope: &SharedScope, stmt: parser::ast::MethodDeclStmt) -> Result<(SharedScope, Stmt)> {
    let Some(fn_scope) = stmt.fut_scope else { critical_error!() };

    let mut params = vec![];
    for p in stmt.params {
        let param_id = make_symbol(p.name.clone(), &fn_scope, move |id| {
            Ok(Symbol::FnParam(FnParamSym {
                id,
                name: Ident::convert(p.name, id),
                colon_tkn: p.colon_tkn,
                ty: conv_type(p.ty, current_scope)?,
                comma_tkn: p.comma_tkn,
            }))
        })?;

        params.push(param_id);
    }

    let guard = current_scope.borrow_mut();

    if !guard.by_name.contains_key(&stmt.name.val) {
        critical_error!()
    }

    let block = conv_stmt(*stmt.block, &fn_scope)?;

    let data = Symbol::Function(FnSym {
        id: symbol,
        fn_tkn: stmt.fn_tkn,
        name: Ident::convert(stmt.name, symbol),
        l_paren: stmt.l_paren,
        params,
        r_paren: stmt.r_paren,
        arrow_tkn: stmt.arrow_tkn,
        return_type: conv_type_opt(stmt.return_type, current_scope)?,
        scope: fn_scope.clone(),
    });

    guard.insert_symbol(symbol, Rc::new(RefCell::new(data)));
    Ok((fn_scope, block))
}

fn make_input(ident: PIdent, current_scope: &SharedScope, stmt: parser::ast::InputStmt) -> Result<SymbolId> {
    make_symbol(ident, current_scope, move |id| Ok(Symbol::Input(VarSym {
        id,
        kw_tkn: stmt.input_tkn,
        name: Ident::convert(stmt.name, id),
        colon_tkn: Some(stmt.colon_tkn),
        ty: Some(conv_type(stmt.ty, current_scope)?),
        eq_tkn: None,
        init: None,
        mods: stmt.mods,
        semi_tkn: stmt.semi_tkn,
        cnst: false,
    })))
}

fn make_output(ident: PIdent, current_scope: &SharedScope, stmt: parser::ast::OutputStmt) -> Result<SymbolId> {
    make_symbol(ident, current_scope, move |id| Ok(Symbol::Input(VarSym {
        id,
        kw_tkn: stmt.output_tkn,
        name: Ident::convert(stmt.name, id),
        colon_tkn: Some(stmt.colon_tkn),
        ty: Some(conv_type(stmt.ty, current_scope)?),
        eq_tkn: None,
        init: None,
        mods: ResMods::empty(),
        semi_tkn: stmt.semi_tkn,
        cnst: false,
    })))
}

fn make_provide(ident: PIdent, current_scope: &SharedScope, stmt: parser::ast::ProvideStmt) -> Result<SymbolId> {
    make_symbol(ident, current_scope, move |id| Ok(Symbol::Input(VarSym {
        id,
        kw_tkn: stmt.provide_tkn,
        name: Ident::convert(stmt.name, id),
        colon_tkn: Some(stmt.colon_tkn),
        ty: Some(conv_type(stmt.ty, current_scope)?),
        eq_tkn: None,
        init: None,
        mods: stmt.mods,
        semi_tkn: stmt.semi_tkn,
        cnst: false,
    })))
}

fn make_push_const(ident: PIdent, current_scope: &SharedScope, stmt: parser::ast::PushConstantsStmt) -> Result<SymbolId> {
    make_symbol(ident, current_scope, move |id| Ok(Symbol::PushConstant(VarSym {
        id,
        kw_tkn: stmt.pc_tkn,
        name: Ident::convert(stmt.name, id),
        colon_tkn: Some(stmt.colon_tkn),
        ty: Some(conv_type(stmt.ty, current_scope)?),
        eq_tkn: None,
        init: None,
        mods: ResMods::empty(),
        semi_tkn: stmt.semi_tkn,
        cnst: false,
    })))
}

fn make_uniform(ident: PIdent, current_scope: &SharedScope, stmt: parser::ast::UniformStmt) -> Result<SymbolId> {
    make_symbol(ident, current_scope, move |id| Ok(Symbol::Uniform(UniformSym {
        uniform_tkn: stmt.uniform_tkn,
        name: Ident::convert(stmt.name, id),
        ty: conv_type(stmt.ty, current_scope)?,
        set_tkn: stmt.set_tkn,
        set_eq_tkn: stmt.set_eq_tkn,
        set_lit_tkn: stmt.set_lit_tkn,
        set: stmt.set,
        binding_tkn: stmt.binding_tkn,
        binding_eq_tkn: stmt.binding_eq_tkn,
        binding_lit_tkn: stmt.binding_lit_tkn,
        binding: stmt.binding,
        mods: stmt.mods,
        uniform_type: stmt.uniform_type,
        semi_tkn: stmt.semi_tkn,
    })))
}

fn make_struct(symbol: SymbolId, current_scope: &SharedScope, stmt: parser::ast::StructStmt) -> Result<SharedScope> {
    let Some(struct_scope) = stmt.fut_scope else { critical_error!() };

    let mut fields = vec![];
    for field in stmt.fields {
        let f_id = make_symbol(field.name.clone(), &struct_scope, |id| Ok(Symbol::Field(FieldSym {
            name: Ident::convert(field.name, id),
            colon_tkn: field.colon_tkn,
            ty: conv_type(field.ty, &struct_scope)?,
            semi_tkn: field.semi_tkn,
        })))?;
        fields.push(f_id);
    }

    let mut methods = vec![];
    for method in stmt.methods {
        let Some(f_id) = method.symbol_id else { critical_error!() };
        make_fn(f_id, &struct_scope, method)?;
        methods.push(f_id);
    }

    let guard = current_scope.borrow_mut();

    if !guard.by_name.contains_key(&stmt.name.val) {
        critical_error!()
    }

    let data = Rc::new(RefCell::new(Symbol::Struct(StructSym {
        id: symbol,
        internal_scope: struct_scope.clone(),
        name: Ident::convert(stmt.name, symbol),
        brace1_tkn: stmt.brace1_tkn,
        fields,
        methods,
        brace2_tkn: stmt.brace2_tkn,
    })));

    guard.insert_symbol(symbol, data);
    Ok(struct_scope)
}

pub(crate) fn make_symbol<F: FnOnce(SymbolId) -> Result<Symbol>>(ident: PIdent, current_scope: &SharedScope, generator: F) -> Result<SymbolId> {
    let name = ident.val;
    let symbol_id = mvutils::utils::next_id(COUNTER_ID);
    let symbol = generator(symbol_id)?;
    let mut guard = current_scope.borrow_mut();
    let symbol = Rc::new(RefCell::new(symbol));
    let entry = guard.by_name.entry(name);
    match entry {
        Entry::Occupied(mut oe) => {
            oe.insert(symbol_id);
        }
        Entry::Vacant(ve) => {
            ve.insert(symbol_id);
        }
    }

    guard.insert_symbol(symbol_id, symbol);
    Ok(symbol_id)
}

fn resolve_ident(old: PIdent, scope: &SharedScope, species: SymbolSpecies) -> Result<Ident> {
    let guard = scope.borrow();
    let ident_id = guard.resolve_symbol_id(&old, species)?;
    Ok(Ident::convert(old, ident_id))
}

/* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=- */

pub fn conv_stmt_opt(expr: Option<PStmt>, current_scope: &SharedScope) -> Result<Option<Stmt>> {
    if let Some(stmt) = expr {
        Ok(Some(conv_stmt(stmt, current_scope)?))
    } else {
        Ok(None)
    }
}

pub fn conv_stmt_opt_box(expr: Option<Box<PStmt>>, current_scope: &SharedScope) -> Result<Option<Box<Stmt>>> {
    if let Some(stmt) = expr {
        let b = conv_stmt(*stmt, current_scope)?;
        Ok(Some(Box::new(b)))
    } else {
        Ok(None)
    }
}

pub fn conv_stmt(stmt: PStmt, current_scope: &SharedScope) -> Result<Stmt> {
    let c = match stmt {
        PStmt::If(s) => {
            let cond = conv_expr(s.cond, current_scope)?;
            let branch = conv_stmt(*s.branch, current_scope)?;
            Stmt::If(IfStmt {
                if_tkn: s.if_tkn,
                l_paren: s.l_paren,
                cond,
                r_paren: s.r_paren,
                branch: Box::new(branch),
                else_tkn: s.else_tkn,
                else_branch: conv_stmt_opt_box(s.else_branch, current_scope)?,
            })
        }
        PStmt::For(s) => {
            let for_scope = s.fut_scope.expect("first pass must have already added all scopes");
            let block = conv_stmt(*s.block, &for_scope)?;

            Stmt::For(ForStmt {
                for_tkn: s.for_tkn,
                l_paren: s.l_paren,
                start_cond: conv_expr_opt(s.start_cond, &for_scope)?,
                semi1_tkn: s.semi1_tkn,
                cond: conv_expr_opt(s.cond, &for_scope)?,
                semi2_tkn: s.semi2_tkn,
                after_run: conv_expr_opt(s.after_run, &for_scope)?,
                r_paren: s.r_paren,
                block: Box::new(block),
                scope: for_scope,
            })
        }
        PStmt::While(s) => {
            let cond =  conv_expr(s.cond, current_scope)?;
            let block = conv_stmt(*s.block, current_scope)?;

            Stmt::While(WhileStmt {
                while_tkn: s.while_tkn,
                l_paren: s.l_paren,
                cond,
                r_paren: s.r_paren,
                block: Box::new(block),
            })
        }
        PStmt::MethodDecl(s) => {
            let Some(symbol) = s.symbol_id else { critical_error!() };
            let (scope, block) = make_fn(symbol, current_scope, s)?;
            Stmt::MethodDecl(MethodDeclStmt {
                symbol,
                scope,
                block: Box::new(block)
            })
        }
        PStmt::VarDecl(s) => {
            let cnst = s.cnst;
            let symbol = make_var(s.name.clone(), current_scope, s)?;
            if cnst {
                Stmt::ConstDecl(ConstDeclStmt {
                    symbol,
                })
            } else {
                Stmt::VarDecl(VarDeclStmt {
                    symbol,
                })
            }
        }
        PStmt::Return(s) => {
            Stmt::Return(ReturnStmt {
                return_tkn: s.return_tkn,
                expr: conv_expr_opt(s.expr, current_scope)?,
                semi_tkn: s.semi_tkn,
            })
        }
        PStmt::Yield(s) => {
            Stmt::Yield(YieldStmt {
                yield_tkn: s.yield_tkn,
                expr: conv_expr(s.expr, current_scope)?,
                semi_tkn: s.semi_tkn,
            })
        }
        PStmt::Break(s) => {
            Stmt::Break(BreakStmt {
                break_tkn: s.break_tkn,
            })
        }
        PStmt::Continue(s) => {
            Stmt::Continue(ContinueStmt {
                continue_tkn: s.continue_tkn,
            })
        }
        PStmt::Include(s) => {
            todo!()
        }
        PStmt::Extension(s) => {
            Stmt::Extension(ExtensionStmt {
                mods: s.mods,
                extension: Ident::plain(s.extension),
                semi_tkn: s.semi_tkn,
            })
        }
        PStmt::Input(s) => {
            let symbol = make_input(s.name.clone(), current_scope, s)?;
            Stmt::Input(InputStmt {
                symbol,
            })
        }
        PStmt::Output(s) => {
            let symbol = make_output(s.name.clone(), current_scope, s)?;
            Stmt::Output(OutputStmt {
                symbol,
            })
        }
        PStmt::Provide(s) => {
            let symbol = make_provide(s.name.clone(), current_scope, s)?;
            Stmt::Provide(ProvideStmt {
                symbol,
            })
        }
        PStmt::PushConstants(s) => {
            let symbol = make_push_const(s.name.clone(), current_scope, s)?;
            Stmt::PushConstants(PushConstantsStmt {
                symbol,
            })
        }
        PStmt::Uniform(s) => {
            let symbol = make_uniform(s.name.clone(), current_scope, s)?;
            Stmt::Uniform(UniformStmt {
                symbol,
            })
        }
        PStmt::Struct(s) => {
            let symbol = s.symbol_id.expect("first pass must have already added all struct symbol ids");
            let scope = make_struct(symbol, current_scope, s)?;
            Stmt::Struct(StructStmt {
                symbol,
                scope
            })
        }
        PStmt::Block(s) => {
            let new_scope = s.fut_scope.expect("first pass must have already added all scopes");
            let mut stmts = vec![];
            for stmt in s.stmts {
                stmts.push(conv_stmt(stmt, &new_scope)?);
            }

            Stmt::Block(BlockStmt {
                l_brace: s.l_brace,
                stmts,
                r_brace: s.r_brace,
                scope: new_scope,
            })
        }
        PStmt::Compound(s) => {
            let mut out = vec![];
            for s in s.components {
                let c = conv_stmt(s, current_scope)?;
                out.push(c);
            }
            Stmt::Compound(CompoundStmt {
                components: out,
            })
        }
        PStmt::Expr(s) => {
            Stmt::Expr(ExprStmt {
                expr: conv_expr(s.expr, current_scope)?,
                semi_tkn: s.semi_tkn,
            })
        }
        PStmt::Semi(s) => {
            Stmt::Semi(SemiStmt {
                semi_tkn: s.semi_tkn,
            })
        }
    };

    Ok(c)
}

pub fn conv_expr_opt_box(expr: Option<Box<PExpr>>, current_scope: &SharedScope) -> Result<Option<Box<Expr>>> {
    if let Some(expr) = expr {
        let b = conv_expr(*expr, current_scope)?;
        Ok(Some(Box::new(b)))
    } else {
        Ok(None)
    }
}

pub fn conv_expr_opt(expr: Option<PExpr>, current_scope: &SharedScope) -> Result<Option<Expr>> {
    if let Some(expr) = expr {
        Ok(Some(conv_expr(expr, current_scope)?))
    } else {
        Ok(None)
    }
}

pub fn conv_expr(expr: PExpr, current_scope: &SharedScope) -> Result<Expr> {
    Ok(match expr {
        PExpr::Unary(e) => {
            Expr::Unary(UnaryExpr {
                op: e.op,
                op_tkn: e.op_tkn,
                expr: Box::new(conv_expr(*e.expr, current_scope)?),
                ty: None,
            })
        }
        PExpr::Binary(e) => {
            Expr::Binary(BinExpr {
                op: e.op,
                op_tkn: e.op_tkn,
                lhs: Box::new(conv_expr(*e.lhs, current_scope)?),
                rhs: Box::new(conv_expr(*e.rhs, current_scope)?),
                ty: None,
            })
        }
        PExpr::FnCall(e) => {
            let mut args = Vec::new();
            for (expr, tkn) in e.args {
                args.push((conv_expr(expr, current_scope)?, tkn))
            }
            if let Ok(ident) = resolve_ident(e.ident.clone(), current_scope, SymbolSpecies::Type) {
                Expr::Constructor(ConstructorExpr {
                    ident,
                    open_tkn: e.open_tkn,
                    args,
                    close_tkn: e.close_tkn,
                    ty: None,
                })
            } else {
                Expr::FnCall(FnCallExpr {
                    ident: resolve_ident(e.ident, current_scope, SymbolSpecies::Fn)?,
                    open_tkn: e.open_tkn,
                    args,
                    close_tkn: e.close_tkn,
                    ty: None,
                })
            }
        }
        PExpr::Access(e) => {
            Expr::Access(AccessExpr {
                parent: Box::new(conv_expr(*e.parent, current_scope)?),
                dot_tkn: e.dot_tkn,
                child: resolve_ident(e.child, current_scope, SymbolSpecies::Ident)?,
                ty: None,
            })
        }
        PExpr::Variable(e) => {
            Expr::Variable(VarExpr {
                ident: resolve_ident(e.ident, current_scope, SymbolSpecies::Ident)?,
                ty: None,
            })
        }
        PExpr::Literal(e) => {
            Expr::Literal(LitExpr {
                lit: e.lit,
                lit_tkn: e.lit_tkn,
                ty: None,
            })
        }
        PExpr::Index(e) => {
            Expr::Index(IndexExpr {
                array: Box::new(conv_expr(*e.array, current_scope)?),
                open_tkn: e.open_tkn,
                index: Box::new(conv_expr(*e.index, current_scope)?),
                close_tkn: e.close_tkn,
                ty: None,
            })
        }
        PExpr::Ternary(e) => {
            Expr::Ternary(TernaryExpr {
                cond: Box::new(conv_expr(*e.cond, current_scope)?),
                question_tkn: e.question_tkn,
                yes: Box::new(conv_expr(*e.yes, current_scope)?),
                colon_tkn: e.colon_tkn,
                no: Box::new(conv_expr(*e.no, current_scope)?),
                ty: None,
            })
        }
        PExpr::PreFix(e) => {
            Expr::PreFix(PreFixExpr {
                op: e.op,
                op_tkn: e.op_tkn,
                expr: Box::new(conv_expr(*e.expr, current_scope)?),
                ty: None,
            })
        }
        PExpr::PostFix(e) => {
            Expr::PostFix(PostFixExpr {
                op: e.op,
                op_tkn: e.op_tkn,
                expr: Box::new(conv_expr(*e.expr, current_scope)?),
                ty: None,
            })
        }
        PExpr::Tuple(e) => {
            let mut args = Vec::new();
            for (arg, tkn) in e.args {
                args.push((conv_expr(arg, current_scope)?, tkn))
            }
            Expr::Tuple(TupleExpr {
                open_tkn: e.open_tkn,
                args,
                close_tkn: e.close_tkn,
                ty: None,
            })
        }
        PExpr::Array(e) => {
            let mut args = Vec::new();
            for (expr, tkn) in e.args {
                args.push((conv_expr(expr, current_scope)?, tkn))
            }

            Expr::Array(ArrayExpr {
                open_tkn: e.open_tkn,
                args,
                close_tkn: e.close_tkn,
                ty: None,
            })
        }
        PExpr::Block(e) => {
            let scope = e.fut_scope.expect("first pass must have already added all scopes");

            let mut stmts = vec![];
            for stmt in e.block {
                let s = conv_stmt(stmt, &scope)?;
                stmts.push(s);
            }

            Expr::Block(BlockExpr {
                open_tkn: e.open_tkn,
                scope,
                block: stmts,
                close_tkn: e.close_tkn,
                ty: None,
            })
        }
        PExpr::Assign(e) => {
            Expr::Assign(AssignExpr {
                lhs: Box::new(conv_expr(*e.lhs, current_scope)?),
                eq_tkn: e.eq_tkn,
                rhs: Box::new(conv_expr(*e.rhs, current_scope)?),
                ty: None,
            })
        }
        PExpr::Nonuniform(e) => {
            Expr::Nonuniform(NonuniformExpr {
                nonuniform_tkn: e.nonuniform_tkn,
                expr: Box::new(conv_expr(*e.expr, current_scope)?),
                ty: None,
            })
        }
        PExpr::TupleAccess(e) => {
            Expr::TupleAccess(TupleAccessExpr {
                tuple: Box::new(conv_expr(*e.tuple, current_scope)?),
                component: e.component,
                ty: None,
            })
        }
        PExpr::AccessFnCall(e) => {
            let mut args = Vec::new();
            for (expr, tkn) in e.args {
                args.push((conv_expr(expr, current_scope)?, tkn))
            }

            Expr::AccessFnCall(AccessFnCallExpr {
                parent: Box::new(conv_expr(*e.parent, current_scope)?),
                ident: resolve_ident(e.ident, current_scope, SymbolSpecies::Fn)?,
                open_tkn: e.open_tkn,
                args,
                close_tkn: e.close_tkn,
                ty: None,
            })
        }
        PExpr::This(e) => {
            Expr::This(ThisExpr {
                this_tkn: e.this_tkn,
                ty: None
            })
        }
        PExpr::As(s) => {
            Expr::As(AsExpr {
                expr: Box::new(conv_expr(*s.expr, current_scope)?),
                as_tkn: s.as_tkn,
                target_ty: conv_type(s.ty, current_scope)?,
                ty: None
            })
        }
        PExpr::Construct(s) => {
            let guard = current_scope.borrow();
            let struct_sym = guard.resolve_symbol(&s.name, SymbolSpecies::Type)?;
            let guard = &*struct_sym.borrow();
            let sym = enum_val_ref!(Symbol, guard, Struct);
            let struct_scope = sym.internal_scope.clone();

            let mut fields = vec![];
            for (field, tok) in s.fields {
                let f = FieldInit {
                    name: resolve_ident(field.name, &struct_scope, SymbolSpecies::Ident)?,
                    colon_tkn: field.colon_tkn,
                    expr: Box::new(conv_expr(*field.expr, current_scope)?),
                };
                fields.push((f, tok));
            }

            Expr::Construct(ConstructExpr {
                name: resolve_ident(s.name, current_scope, SymbolSpecies::Type)?,
                brace1_tkn: s.brace1_tkn,
                fields,
                brace2_tkn: s.brace2_tkn,
                ty: None,
            })
        }
    })
}

pub fn conv_type_opt(ty: Option<PType>, current_scope: &SharedScope) -> Result<Option<Type>> {
    if let Some(expr) = ty {
        Ok(Some(conv_type(expr, current_scope)?))
    } else {
        Ok(None)
    }
}

pub fn conv_type(ty: PType, current_scope: &SharedScope) -> Result<Type> {
    Ok(match ty {
        PType::Primitive(ty, tkn) => Type::Primitive(PrimitiveRef { ty, tkn }),
        PType::SingleType(ty) => {
            Type::StructRef(StructRef {
                sym: current_scope.borrow().resolve_symbol_id(&ty, SymbolSpecies::Type)?,
                name_tkn: ty.tkn
            })
        },
        PType::PathType(_path) => todo!(),
        PType::ArrayOf(arr) => {
            Type::ArrayOf(ArrayType {
                component: Box::new(conv_type(*arr.component, current_scope)?),
                brack1_tkn: arr.brack1_tkn,
                dimension: conv_expr_opt_box(arr.dimension, current_scope)?,
                evaluated: None,
                brack2_tkn: arr.brack2_tkn,
            })
        }
        PType::TupleOf(tup) => {
            let mut types = Vec::with_capacity(tup.types.len());
            for (ty, tok) in tup.types {
                types.push((conv_type(ty, current_scope)?, tok));
            }
            Type::Tuple(TupleDef {
                paren1_tok: tup.paren1_tok,
                types,
                paren2_tok: tup.paren2_tok,
            })
        }
        PType::StructDef(def) => {
            Type::StructDef(StructDef {
                sym: def.inner.symbol_id.expect("first pass must have already added all struct symbol ids"),
                struct_tkn: def.inner.struct_tkn,
                name_tkn: def.inner.name.tkn,
                brace1_tkn: def.inner.brace1_tkn,
                brace2_tkn: def.inner.brace2_tkn
            })
        }
    })
}
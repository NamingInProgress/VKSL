use mvutils::enum_val_ref_mut;
use crate::{parser, symbol};
use crate::parser::err::{ParseErr, ParseErrType};
use crate::scope::ast::Ast;
use crate::scope::ast::expr::{Expr, LitExpr};
use crate::scope::ast::stmt::{ExprStmt, MethodDeclStmt, Stmt};
use crate::scope::ast::ty::Type;
use crate::scope::{SharedScope, Symbol};
use crate::token::{Literal, Operator, TokCtx};
use crate::utils::ReplaceIter;

pub fn collapse_constants(mut ast: Ast, scope: &SharedScope) -> parser::Result<Ast> {

    Ok(ast)
}

fn handle_stmt(mut stmt: Stmt, scope: &SharedScope) -> parser::Result<Stmt> {
    macro_rules! optional_stmt {
        ($s:ident,$f:ident) => {
            if let Some(stmt) = $s.$f {
                $s.$f = Some(Box::new(handle_stmt(*stmt, scope)?))
            }
        };
    }
    macro_rules! optional_expr {
        ($s:ident,$f:ident) => {
            if let Some(expr) = $s.$f {
                $s.$f = Some(handle_non_const_expr(expr, scope)?)
            }
        };
    }
    macro_rules! optional_ty {
        ($s:ident,$f:ident,$scope:ident) => {
            if let Some(ty) = $s.$f {
                $s.$f = Some(handle_type(ty, $scope)?);
            }
        };
    }
    Ok(match stmt {
        Stmt::If(mut s) => {
            s.cond = handle_non_const_expr(s.cond, scope)?;
            s.branch = Box::new(handle_stmt(*s.branch, scope)?);
            if let Some(branch) = s.else_branch {
                s.else_branch = Some(Box::new(handle_stmt(*branch, scope)?))
            }
            optional_stmt!(s, else_branch);
            Stmt::If(s)
        }
        Stmt::For(mut s) => {
            optional_expr!(s, start_cond);
            optional_expr!(s, cond);
            optional_expr!(s, after_run);
            s.block = Box::new(handle_stmt(*s.block, &s.scope)?);
            Stmt::For(s)
        }
        Stmt::While(mut s) => {
            s.cond = handle_non_const_expr(s.cond, scope)?;
            s.block = Box::new(handle_stmt(*s.block, scope)?);
            Stmt::While(s)
        }
        Stmt::MethodDecl(mut s) => {
            let sub_scope = &s.scope;
            let sym = symbol!((scope => s.symbol) as mut Function);
            for field in &sym.params {
                let field_sym = symbol!((sub_scope => *field) as mut Variable);
                optional_ty!(field_sym, ty, sub_scope);
            }
            optional_ty!(sym, return_type, sub_scope);

            s.block = Box::new(handle_stmt(*s.block, scope)?);
            Stmt::MethodDecl(s)
        }
        Stmt::VarDecl(mut s) => {
            let sym = symbol!((scope => s.symbol) as mut Variable);
            optional_expr!(sym, init);
            optional_ty!(sym, ty, scope);
            Stmt::VarDecl(s)
        }
        Stmt::ConstDecl(s) => {
            let sym = symbol!((scope => s.symbol) as mut Variable);
            optional_ty!(sym, ty, scope);
            if let Some(init) = sym.init {
                sym.init = Some(handle_const_expr(init, scope)?);
            }
            Stmt::ConstDecl(s)
        }
        Stmt::Return(_) => {}
        Stmt::Yield(_) => {}
        Stmt::Input(_) => {}
        Stmt::Output(_) => {}
        Stmt::Provide(_) => {}
        Stmt::PushConstants(_) => {}
        Stmt::Uniform(_) => {}
        Stmt::Struct(mut s) => {
            let struct_scope = &s.scope;
            let sym = symbol!((scope => s.symbol) as Struct);
            for field in sym.fields {
                let field_sym = symbol!((struct_scope => field) as mut Variable);

            }

            let mut new_methods = Vec::with_capacity(s.methods.len());            
            for mut meth_body in s.methods.drain(..) {
                let sub_scope = &meth_body.scope;
                let sym = symbol!((scope => meth_body.symbol) as mut Function);
                for field in &sym.params {
                    let field_sym = symbol!((sub_scope => *field) as mut Variable);
                    optional_ty!(field_sym, ty, sub_scope);
                }
                optional_ty!(sym, return_type, sub_scope);

                meth_body.block = Box::new(handle_stmt(*meth_body.block, scope)?);
                new_methods.push(meth_body);
            }

            s.methods = new_methods;

            Stmt::Struct(s)
        }
        Stmt::Block(mut b) => {
            b.stmts = handle_block(b.stmts, &b.scope)?;
            Stmt::Block(b)
        }
        Stmt::Compound(mut c) => {
            c.components = handle_block(c.components, scope)?;
            Stmt::Compound(c)
        }
        Stmt::Expr(ExprStmt { expr: Expr::Block(mut b), semi_tkn }) => {
            b.block = handle_block(b.block, &b.scope)?;
            Stmt::Expr(ExprStmt { expr: Expr::Block(b), semi_tkn })
        }
        Stmt::Expr(_) => {}
        s => s,
    })
}

fn handle_block<I: IntoIterator<Item=Stmt>>(block: I, scope: &SharedScope) -> parser::Result<Vec<Stmt>> {
    let mut vec = vec![];
    for item in block {
        vec.push(handle_stmt(item, scope)?);
    }
    Ok(vec)
}

fn handle_method_decl(mut s: MethodDeclStmt, scope: &SharedScope) -> parser::Result<MethodDeclStmt> {

    Ok(s)
}

fn handle_type(ty: Type, scope: &SharedScope) -> parser::Result<Type> {

}

fn handle_non_const_expr(expr: Expr, scope: &SharedScope) -> parser::Result<Expr> {

}

fn handle_const_expr(expr: Expr, scope: &SharedScope) -> parser::Result<Expr> {
    match expr {
        Expr::Unary(e) => {
            let literal = match handle_const_expr(*e.expr, scope)? {
                Expr::Literal(lit) => {
                    lit
                },
                Expr::Variable(var) => {
                    todo!()
                }
                other => return Ok(other),
            };

            let tkn = literal.lit_tkn.clone();
            let literal = apply_op(literal, e.op, None)?;
            Ok(Expr::Literal(LitExpr {
                lit: literal,
                lit_tkn: tkn,
                ty: None,
            }))
        }
        Expr::Binary(e) => {}
        Expr::FnCall(e) => {}
        Expr::Access(e) => {}
        Expr::Variable(e) => {}
        Expr::Literal(e) => {}
        Expr::Index(e) => {}
        Expr::Ternary(e) => {}
        Expr::PreFix(e) => {}
        Expr::PostFix(e) => {}
        Expr::Tuple(e) => {}
        Expr::Array(e) => {}
        Expr::Block(e) => {}
        Expr::Assign(e) => {}
        Expr::Nonuniform(e) => {}
        Expr::TupleAccess(e) => {}
        Expr::AccessFnCall(e) => {}
        Expr::This(e) => {}
        Expr::As(e) => {}
        Expr::Construct(e) => {}
        Expr::Constructor(e) => {}
    }
}

trait Lit {
    fn lit(&self) -> &Literal;
    fn token(&self) -> TokCtx;
    fn ty(&self) -> parser::Result<&Type>;
}

impl Lit for LitExpr {
    fn lit(&self) -> &Literal {
        &self.lit
    }

    fn token(&self) -> TokCtx {
        self.lit_tkn.clone()
    }

    fn ty(&self) -> parser::Result<&Type> {
        if let Some(ty) = &self.ty {
            Ok(ty)
        } else {
            Err(ParseErr {
                ty: ParseErrType::MissingType,
                ctx: self.token(),
                hint: None,
            })
        }
    }
}

fn apply_op(lhs: impl Lit, op: Operator, rhs: Option<impl Lit>) -> parser::Result<Literal> {
    match op {
        Operator::Assign => {}
        Operator::Plus => {}
        Operator::Minus => {}
        Operator::Mul => {}
        Operator::Div => {}
        Operator::Dot => {}
        Operator::Modulo => {}
        Operator::PlusPlus => {}
        Operator::MinusMinus => {}
        Operator::BitOr => {}
        Operator::BitAnd => {}
        Operator::BitXor => {}
        Operator::BitNegate => {}
        Operator::Greater => {}
        Operator::GreaterEq => {}
        Operator::Less => {}
        Operator::LessEq => {}
        Operator::And => {}
        Operator::Or => {}
        Operator::EqEq => {}
        Operator::Neq => {}
        Operator::Not => {}
        Operator::Lsh => {}
        Operator::Rsh => {}
        Operator::LogicalRsh => {}
        Operator::Merge => {}
    }
}
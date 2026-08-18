use crate::token::{TokCtx, Literal, Operator};
use enum_dispatch::enum_dispatch;
use crate::ast::Ident;

#[enum_dispatch(Exprs)]
#[allow(unused)]
trait Exprs {}

#[derive(Clone, Debug)]
#[enum_dispatch(Exprs)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinExpr),
    FnCall(FnCallExpr),
    Access(AccessExpr),
    Variable(VarExpr),
    Literal(LitExpr),
    Index(IndexExpr),
    Ternary(TernaryExpr),
    PreFix(PreFixExpr),
    PostFix(PostFixExpr),
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub op: Operator,
    pub op_tkn: TokCtx,
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct BinExpr {
    pub op: Operator,
    pub op_tkn: TokCtx,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct FnCallExpr {
    pub ident: Ident,
    pub paren1_tkn: TokCtx,
    pub args: Vec<(Expr, Option<TokCtx>)>,
    pub paren2_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct AccessExpr {
    pub parent: Box<Expr>,
    pub dot_tkn: TokCtx,
    pub child: Ident,
}

#[derive(Clone, Debug)]
pub struct VarExpr {
    pub ident: Ident,
}

#[derive(Clone, Debug)]
pub struct LitExpr {
    pub lit: Literal,
    pub lit_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct IndexExpr {
    pub array: Box<Expr>,
    pub open_tkn: TokCtx,
    pub index: Box<Expr>,
    pub close_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct TernaryExpr {
    pub cond: Box<Expr>,
    pub question_tkn: TokCtx,
    pub yes: Box<Expr>,
    pub colon_tkn: TokCtx,
    pub no: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct PreFixExpr {
    pub op: Operator,
    pub op_tkn: TokCtx,
    pub expr: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct PostFixExpr {
    pub op: Operator,
    pub op_tkn: TokCtx,
    pub expr: Box<Expr>
}
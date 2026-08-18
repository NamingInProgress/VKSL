use enum_dispatch::enum_dispatch;
use crate::token::{Literal, Operator};

#[enum_dispatch(Exprs)]
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
    PushFront(PushFrontExpr),
    PushBack(PushBackExpr),
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub op: Operator,
    pub expr: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct BinExpr {
    pub op: Operator,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct FnCallExpr {
    pub name: String,
    pub args: Vec<Expr>
}

#[derive(Clone, Debug)]
pub struct AccessExpr {
    pub parent: Box<Expr>,
    pub child: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct VarExpr {
    pub name: String
}

#[derive(Clone, Debug)]
pub struct LitExpr {
    pub lit: Literal
}

#[derive(Clone, Debug)]
pub struct IndexExpr {
    pub array: Box<Expr>,
    pub index: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct TernaryExpr {
    pub cond: Box<Expr>,
    pub yes: Box<Expr>,
    pub no: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct PushFrontExpr {
    pub target: Box<Expr>,
    pub addition: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct PushBackExpr {
    pub target: Box<Expr>,
    pub addition: Box<Expr>
}
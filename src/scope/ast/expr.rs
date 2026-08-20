use enum_dispatch::enum_dispatch;
use crate::scope::ast::ty::Type;
use crate::scope::ast::Ident;
use crate::scope::ast::stmt::Stmt;
use crate::scope::SharedScope;
use crate::token::{Literal, Operator, TokCtx};

#[enum_dispatch(Exprs2)]
#[allow(unused)]
trait Exprs2 {}

#[derive(Clone, Debug)]
#[enum_dispatch(Exprs2)]
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
    Tuple(TupleExpr),
    Array(ArrayExpr),
    Block(BlockExpr),
    Assign(AssignExpr),
    Nonuniform(NonuniformExpr),
    TupleAccess(TupleAccessExpr),
    AccessFnCall(AccessFnCallExpr),
    This(ThisExpr),
    As(AsExpr),
    Construct(ConstructExpr),
    Constructor(ConstructorExpr),
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub op: Operator,
    pub op_tkn: TokCtx,
    pub expr: Box<Expr>,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct BinExpr {
    pub op: Operator,
    pub op_tkn: TokCtx,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct FnCallExpr {
    pub ident: Ident,
    pub open_tkn: TokCtx,
    pub args: Vec<(Expr, Option<TokCtx>)>,
    pub close_tkn: TokCtx,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct AccessExpr {
    pub parent: Box<Expr>,
    pub dot_tkn: TokCtx,
    pub child: Ident,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct VarExpr {
    pub ident: Ident,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct LitExpr {
    pub lit: Literal,
    pub lit_tkn: TokCtx,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct IndexExpr {
    pub array: Box<Expr>,
    pub open_tkn: TokCtx,
    pub index: Box<Expr>,
    pub close_tkn: TokCtx,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct TernaryExpr {
    pub cond: Box<Expr>,
    pub question_tkn: TokCtx,
    pub yes: Box<Expr>,
    pub colon_tkn: TokCtx,
    pub no: Box<Expr>,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct PreFixExpr {
    pub op: Operator,
    pub op_tkn: TokCtx,
    pub expr: Box<Expr>,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct PostFixExpr {
    pub op: Operator,
    pub op_tkn: TokCtx,
    pub expr: Box<Expr>,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct TupleExpr {
    pub open_tkn: TokCtx,
    pub args: Vec<(Expr, Option<TokCtx>)>,
    pub close_tkn: TokCtx,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct ArrayExpr {
    pub open_tkn: TokCtx,
    pub args: Vec<(Expr, Option<TokCtx>)>,
    pub close_tkn: TokCtx,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct BlockExpr {
    pub open_tkn: TokCtx,
    pub scope: SharedScope,
    pub block: Vec<Stmt>,
    pub close_tkn: TokCtx,
    pub ty: Option<Type>,
}

#[derive(Clone, Debug)]
pub struct AssignExpr {
    pub lhs: Box<Expr>,
    pub eq_tkn: TokCtx,
    pub rhs: Box<Expr>,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct NonuniformExpr {
    pub nonuniform_tkn: TokCtx,
    pub expr: Box<Expr>,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct TupleAccessExpr {
    pub tuple: Box<Expr>,
    pub component: u32,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct AccessFnCallExpr {
    pub parent: Box<Expr>,
    pub ident: Ident,
    pub open_tkn: TokCtx,
    pub args: Vec<(Expr, Option<TokCtx>)>,
    pub close_tkn: TokCtx,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct ThisExpr {
    pub this_tkn: TokCtx,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct AsExpr {
    pub expr: Box<Expr>,
    pub as_tkn: TokCtx,
    pub target_ty: Type,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct ConstructExpr {
    pub name: Ident,
    pub brace1_tkn: TokCtx,
    pub fields: Vec<(FieldInit, Option<TokCtx>)>,
    pub brace2_tkn: TokCtx,
    pub ty: Option<Type>
}

#[derive(Clone, Debug)]
pub struct FieldInit {
    pub name: Ident,
    pub colon_tkn: TokCtx,
    pub expr: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct ConstructorExpr {
    pub ident: Ident,
    pub open_tkn: TokCtx,
    pub args: Vec<(Expr, Option<TokCtx>)>,
    pub close_tkn: TokCtx,
    pub ty: Option<Type>
}
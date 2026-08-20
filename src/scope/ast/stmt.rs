use crate::parser::mods::ResMods;
use crate::scope::ast::expr::Expr;
use crate::scope::ast::ty::Type;
use crate::scope::ast::Ident;
use crate::token::TokCtx;
use enum_dispatch::enum_dispatch;
use crate::parser;
use crate::scope::{SharedScope, SymbolId};

#[enum_dispatch(Stmts2)]
#[allow(unused)]
trait Stmts2 {}

#[derive(Clone, Debug)]
#[enum_dispatch(Stmts2)]
pub enum Stmt {
    If(IfStmt),
    For(ForStmt),
    While(WhileStmt),
    MethodDecl(MethodDeclStmt),
    VarDecl(VarDeclStmt),
    ConstDecl(ConstDeclStmt),
    Return(ReturnStmt),
    Yield(YieldStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Include(IncludeStmt),
    Extension(ExtensionStmt),
    Input(InputStmt),
    Output(OutputStmt),
    Provide(ProvideStmt),
    PushConstants(PushConstantsStmt),
    Uniform(UniformStmt),
    Struct(StructStmt),
    Block(BlockStmt),
    Compound(CompoundStmt),
    Expr(ExprStmt),
    Semi(SemiStmt),
}

#[derive(Clone, Debug)]
pub struct BreakStmt {
    pub break_tkn: TokCtx
}

#[derive(Clone, Debug)]
pub struct ContinueStmt {
    pub continue_tkn: TokCtx
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub if_tkn: TokCtx,
    pub l_paren: TokCtx,
    pub cond: Expr,
    pub r_paren: TokCtx,
    pub branch: Box<Stmt>,
    pub else_tkn: Option<TokCtx>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Clone, Debug)]
pub struct ForStmt {
    pub for_tkn: TokCtx,
    pub l_paren: TokCtx,
    pub start_cond: Option<Expr>,
    pub semi1_tkn: TokCtx,
    pub cond: Option<Expr>,
    pub semi2_tkn: TokCtx,
    pub after_run: Option<Expr>,
    pub r_paren: TokCtx,
    pub block: Box<Stmt>,
    pub scope: SharedScope
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub while_tkn: TokCtx,
    pub l_paren: TokCtx,
    pub cond: Expr,
    pub r_paren: TokCtx,
    pub block: Box<Stmt>,
}

#[derive(Clone, Debug)]
pub struct MethodDeclStmt {
    pub symbol: SymbolId,
    pub scope: SharedScope,
    pub block: Box<Stmt>,
    
    
}

#[derive(Clone, Debug)]
pub struct VarDeclStmt {
    pub symbol: SymbolId,
}

#[derive(Clone, Debug)]
pub struct ConstDeclStmt {
    pub symbol: SymbolId,
}

#[derive(Clone, Debug)]
pub struct ReturnStmt {
    pub return_tkn: TokCtx,
    pub expr: Option<Expr>,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct YieldStmt {
    pub yield_tkn: TokCtx,
    pub expr: Expr,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct IncludeStmt {
    pub include_tkn: TokCtx,
    pub include: parser::ast::Type,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct ExtensionStmt {
    pub mods: ResMods,
    pub extension: Ident,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct InputStmt {
    pub symbol: SymbolId,
}

#[derive(Clone, Debug)]
pub struct OutputStmt {
    pub symbol: SymbolId,
}

#[derive(Clone, Debug)]
pub struct ProvideStmt {
    pub symbol: SymbolId,
}

#[derive(Clone, Debug)]
pub struct PushConstantsStmt {
    pub symbol: SymbolId,
}

#[derive(Clone, Debug)]
pub struct UniformStmt {
    pub symbol: SymbolId
}

#[derive(Clone, Debug)]
pub struct StructStmt {
    pub symbol: SymbolId,
    pub scope: SharedScope,
}

#[derive(Clone, Debug)]
pub struct StructField {
    pub name: Ident,
    pub colon_tkn: TokCtx,
    pub ty: Type,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct BlockStmt {
    pub l_brace: TokCtx,
    pub stmts: Vec<Stmt>,
    pub r_brace: TokCtx,
    pub scope: SharedScope
}

#[derive(Clone, Debug)]
pub struct CompoundStmt {
    pub components: Vec<Stmt>
}

#[derive(Clone, Debug)]
pub struct ExprStmt {
    pub expr: Expr,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct SemiStmt {
    pub semi_tkn: TokCtx
}
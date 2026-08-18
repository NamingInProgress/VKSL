use crate::ast::expr::Expr;
use crate::ast::ty::Type;
use enum_dispatch::enum_dispatch;
use crate::ast::Ident;
use crate::parser::mods::{ResMods};
use crate::token::TokCtx;

#[enum_dispatch(Stmts)]
#[allow(unused)]
trait Stmts {}

#[derive(Clone, Debug)]
#[enum_dispatch(Stmts)]
pub enum Stmt {
    If(IfStmt),
    For(ForStmt),
    While(WhileStmt),
    MethodDecl(MethodDeclStmt),
    VarDecl(VarDeclStmt),
    Return(ReturnStmt),
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
    Compound(CompoundStmt)
}

#[derive(Clone, Debug)]
pub enum UniformType {
    Uniform,
    SSBO,
    UBO
}

#[derive(Clone, Debug)]
pub struct BreakStmt {}

#[derive(Clone, Debug)]
pub struct ContinueStmt {}

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
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub while_tkn: TokCtx,
    pub l_paren: TokCtx,
    pub cond: Option<Expr>,
    pub r_paren: TokCtx,
    pub block: Box<Stmt>,
}

#[derive(Clone, Debug)]
pub struct MethodDeclStmt {
    pub fn_tkn: TokCtx,
    pub name: Ident,
    pub l_paren: TokCtx,
    pub params: Vec<MethodParamDecl>,
    pub r_paren: TokCtx,
    pub arrow_tkn: Option<TokCtx>,
    pub return_type: Option<Type>,
    pub block: BlockStmt,
}

#[derive(Clone, Debug)]
pub struct MethodParamDecl {
    pub name: Ident,
    pub colon_tkn: TokCtx,
    pub ty: Type,
    pub comma_tkn: Option<TokCtx>,
}

#[derive(Clone, Debug)]
pub struct VarDeclStmt {
    pub kw_tkn: TokCtx, //either let or const
    pub name: Ident,
    pub colon_tkn: Option<TokCtx>,
    pub ty: Option<Type>,
    pub eq_tkn: Option<TokCtx>,
    pub init: Option<Expr>,
    pub semi_tkn: TokCtx,
    pub cnst: bool,
}

#[derive(Clone, Debug)]
pub struct ReturnStmt {
    pub return_tkn: TokCtx,
    pub expr: Option<Expr>,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct IncludeStmt {
    pub include_tkn: TokCtx,
    pub include: Type,
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
    pub input_tkn: TokCtx,
    pub ty: Type,
    pub name: Ident,
    pub mods: ResMods,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct OutputStmt {
    pub output_tkn: TokCtx,
    pub ty: Type,
    pub name: Ident,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct ProvideStmt {
    pub provide_tkn: TokCtx,
    pub ty: Type,
    pub name: Ident,
    pub mods: ResMods,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct PushConstantsStmt {
    pub pc_tkn: TokCtx,
    pub name: Ident,
    pub ty: Type,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct UniformStmt {
    pub uniform_tkn: TokCtx,
    pub name: Ident,
    pub ty: Type,
    
    pub set_tkn: TokCtx,
    pub set_eq_tkn: TokCtx,
    pub set_lit_tkn: TokCtx,
    pub set: u32,

    pub binding_tkn: TokCtx,
    pub binding_eq_tkn: TokCtx,
    pub binding_lit_tkn: TokCtx,
    pub binding: u32,
    pub mods: ResMods,
    pub uniform_type: UniformType,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct StructStmt {
    pub name: Ident,
    pub brace1_tkn: TokCtx,
    pub fields: Vec<StructField>,
    pub methods: Vec<MethodDeclStmt>,
    pub brace2_tkn: TokCtx,
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
}

#[derive(Clone, Debug)]
pub struct CompoundStmt {
    pub components: Vec<Stmt>
}

use enum_dispatch::enum_dispatch;
use crate::ast::expr::Expr;
use crate::ast::ty::Type;

#[derive(Clone, Debug)]
pub enum ExtensionBehavior {
    Enable,
    Require,
    Warn,
    Disable
}

#[derive(Clone, Debug)]
pub enum InputInterpolation {
    Flat,
    Smooth,
    Noperspective
}

#[derive(Clone, Debug)]
pub enum UniformModifier {
    Readonly,
    PackingType(PackingType)
}

#[derive(Clone, Debug)]
pub enum PackingType {
    STD140,
    STD430
}

#[enum_dispatch(Stmts)]
trait Stmts {}

#[derive(Clone, Debug)]
#[enum_dispatch(Stmts)]
pub enum Stmt {
    If(IfStmt),
    For(ForStmt),
    While(WhileStmt),
    MethodDecl(MethodDeclStmt),
    VarDef(VarDefStmt),
    Return(ReturnStmt),
    Include(IncludeStmt),
    Extension(ExtensionStmt),
    Input(InputStmt),
    Output(OutputStmt),
    Provide(ProvideStmt),
    PushConstants(PushConstantsStmt),
    Uniform(UniformStmt),
    Struct(StructStmt),
    Block(BlockStmt),
    Const(ConstStmt)
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub cond: Expr,
    pub branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Clone, Debug)]
pub struct ForStmt {
    pub start_cond: Option<Expr>,
    pub cond: Option<Expr>,
    pub after_run: Option<Expr>,
    pub block: Box<Stmt>
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub cond: Expr,
    pub block: Box<Stmt>
}

#[derive(Clone, Debug)]
pub struct MethodDeclStmt {
    pub name: String,
    pub params: Vec<MethodParamDef>,
    pub return_type: Option<Type>,
    pub block: BlockStmt
}

#[derive(Clone, Debug)]
pub struct MethodParamDef {
    pub name: String,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct VarDefStmt {
    pub name: String,
    pub ty: Option<Type>,
    pub init: Option<Expr>
}

#[derive(Clone, Debug)]
pub struct ConstStmt {
    pub name: String,
    pub ty: Option<Type>,
    pub init: Option<Expr>
}

#[derive(Clone, Debug)]
pub struct ReturnStmt {
    pub expr: Option<Expr>
}

#[derive(Clone, Debug)]
pub struct IncludeStmt {
    pub include: Type
}

#[derive(Clone, Debug)]
pub struct ExtensionStmt {
    pub behavior: ExtensionBehavior,
    pub extension: String
}

#[derive(Clone, Debug)]
pub struct InputStmt {
    pub ty: Type,
    pub name: String,
    pub interpolation: InputInterpolation
}

#[derive(Clone, Debug)]
pub struct OutputStmt {
    pub ty: Type,
    pub name: String
}

#[derive(Clone, Debug)]
pub struct ProvideStmt {
    pub ty: Type,
    pub name: String,
    pub interpolation: InputInterpolation
}

#[derive(Clone, Debug)]
pub struct PushConstantsStmt {
    pub name: String,
    pub fields: Vec<StructField>
}

#[derive(Clone, Debug)]
pub struct UniformStmt {
    pub name: String,
    pub ty: Type,
    pub mods: Vec<UniformModifier>,
    pub fields: Vec<StructField>
}

#[derive(Clone, Debug)]
pub struct StructStmt {
    pub name: String,
    pub fields: Vec<StructField>
}

#[derive(Clone, Debug)]
pub struct StructField {
    pub name: String,
    pub ty: Type
}

#[derive(Clone, Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>
}
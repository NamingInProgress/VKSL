use crate::ast::expr::Expr;
use crate::ast::stmt::{*};
use crate::ast::ty::Type;

#[allow(non_snake_case)]
pub fn If(cond: Expr, branch: Stmt) -> Stmt {
    IfStmt {
        cond,
        branch: branch.into(),
        else_branch: None
    }.into()
}

#[allow(non_snake_case)]
pub fn IfElse(cond: Expr, branch: Stmt, else_branch: Stmt) -> Stmt {
    IfStmt {
        cond,
        branch: branch.into(),
        else_branch: Some(else_branch.into())
    }.into()
}

#[allow(non_snake_case)]
pub fn For(start: Option<Expr>, cond: Option<Expr>, after_run: Option<Expr>, block: Stmt) -> Stmt {
    ForStmt {
        start_cond: start,
        cond,
        after_run,
        block: block.into()
    }.into()
}

#[allow(non_snake_case)]
pub fn While(cond: Expr, block: Stmt) -> Stmt {
    WhileStmt {
        cond,
        block: block.into()
    }.into()
}

#[allow(non_snake_case)]
pub fn MethodDef(name: String, params: Vec<MethodParamDef>, return_type: Option<Type>, block: BlockStmt) -> Stmt {
    MethodDeclStmt {
        name,
        params,
        return_type,
        block
    }.into()
}

#[allow(non_snake_case)]
pub fn MethodParam(name: String, ty: Type) -> MethodParamDef {
    MethodParamDef {
        name,
        ty
    }
}

#[allow(non_snake_case)]
pub fn LetIT(name: String, ty: Type, init: Expr) -> Stmt {
    VarDefStmt {
        name,
        ty: Some(ty),
        init: Some(init)
    }.into()
}

#[allow(non_snake_case)]
pub fn LetT(name: String, ty: Type) -> Stmt {
    VarDefStmt {
        name,
        ty: Some(ty),
        init: None
    }.into()
}

#[allow(non_snake_case)]
pub fn LetI(name: String, init: Expr) -> Stmt {
    VarDefStmt {
        name,
        ty: None,
        init: Some(init)
    }.into()
}

#[allow(non_snake_case)]
pub fn ConstIT(name: String, ty: Type, init: Expr) -> Stmt {
    ConstStmt {
        name,
        ty: Some(ty),
        init: Some(init)
    }.into()
}

#[allow(non_snake_case)]
pub fn ConstT(name: String, ty: Type) -> Stmt {
    ConstStmt {
        name,
        ty: Some(ty),
        init: None
    }.into()
}

#[allow(non_snake_case)]
pub fn ConstI(name: String, init: Expr) -> Stmt {
    ConstStmt {
        name,
        ty: None,
        init: Some(init)
    }.into()
}

#[allow(non_snake_case)]
pub fn RetV() -> Stmt {
    ReturnStmt {
        expr: None
    }.into()
}

#[allow(non_snake_case)]
pub fn Ret(expr: Expr) -> Stmt {
    ReturnStmt {
        expr: Some(expr)
    }.into()
}

#[allow(non_snake_case)]
pub fn Include(include: Type) -> Stmt {
    IncludeStmt {
        include
    }.into()
}

#[allow(non_snake_case)]
pub fn Extension(behavior: ExtensionBehavior, extension: String) -> Stmt {
    ExtensionStmt {
        behavior,
        extension
    }.into()
}

#[allow(non_snake_case)]
pub fn InputInterp(name: String, ty: Type, interpolation: InputInterpolation) -> Stmt {
    InputStmt {
        ty,
        name,
        interpolation
    }.into()
}

#[allow(non_snake_case)]
pub fn Input(name: String, ty: Type) -> Stmt {
    InputStmt {
        ty,
        name,
        interpolation: InputInterpolation::Smooth
    }.into()
}

#[allow(non_snake_case)]
pub fn Output(name: String, ty: Type) -> Stmt {
    OutputStmt {
        ty,
        name
    }.into()
}

#[allow(non_snake_case)]
pub fn Provide(name: String, ty: Type) -> Stmt {
    ProvideStmt {
        ty,
        name,
        interpolation: InputInterpolation::Smooth
    }.into()
}

#[allow(non_snake_case)]
pub fn ProvideInterp(name: String, ty: Type, interpolation: InputInterpolation) -> Stmt {
    ProvideStmt {
        ty,
        name,
        interpolation
    }.into()
}

#[allow(non_snake_case)]
pub fn PushConstants(name: String, fields: Vec<StructField>) -> Stmt {
    PushConstantsStmt {
        name,
        fields
    }.into()
}

#[allow(non_snake_case)]
pub fn Uniform(name: String, ty: Type) -> Stmt {
    UniformStmt {
        name,
        ty,
        mods: vec!(),
        fields: vec!()
    }.into()
}

#[allow(non_snake_case)]
pub fn UBO(name: String, fields: Vec<StructField>) -> Stmt {
    UniformStmt {
        name,
        ty: Type::SingleType("UNIFORM_BUFFER".to_string()),
        mods: vec!(UniformModifier::PackingType(PackingType::STD140)),
        fields
    }.into()
}

// This method is considered unsafe, because generally a UBO might not permit the STD430 layout.
#[allow(non_snake_case)]
pub fn UBO_packing(name: String, fields: Vec<StructField>, packing_type: PackingType) -> Stmt {
    UniformStmt {
        name,
        ty: Type::SingleType("UNIFORM_BUFFER".to_string()),
        mods: vec!(UniformModifier::PackingType(packing_type)),
        fields
    }.into()
}

#[allow(non_snake_case)]
pub fn SSBO(name: String, fields: Vec<StructField>) -> Stmt {
    UniformStmt {
        name,
        ty: Type::SingleType("SHADER_STORAGE_BUFFER".to_string()),
        mods: vec!(UniformModifier::PackingType(PackingType::STD430)),
        fields
    }.into()
}

#[allow(non_snake_case)]
pub fn SSBO_packing(name: String, fields: Vec<StructField>, packing_type: PackingType) -> Stmt {
    UniformStmt {
        name,
        ty: Type::SingleType("SHADER_STORAGE_BUFFER".to_string()),
        mods: vec!(UniformModifier::PackingType(packing_type)),
        fields
    }.into()
}

#[allow(non_snake_case)]
pub fn Struct(name: String, fields: Vec<StructField>) -> Stmt {
    StructStmt {
        name,
        fields
    }.into()
}

#[allow(non_snake_case)]
pub fn Block(statements: Vec<Stmt>) -> Stmt {
    BlockStmt {
        stmts: statements
    }.into()
}

#[allow(non_snake_case)]
pub fn StructField(name: String, ty: Type) -> StructField {
    StructField {
        name,
        ty
    }.into()
}
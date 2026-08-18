use crate::ast::stmt::StructStmt;
use mvutils_proc_macro::TryFromString;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::ast::Ident;
use crate::token::TokCtx;

#[derive(Clone, Debug)]
pub enum Type {
    Primitive(PrimitiveType, TokCtx),
    SingleType(String, TokCtx),
    PathType(PathType),
    ArrayOf(ArrayType),
    StructDef(Box<StructStmt>),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Type::Primitive(ty, _) => ty.to_string(),
            Type::SingleType(ty, _) => ty.clone(),
            Type::PathType(ty) => ty.to_string(),
            Type::ArrayOf(ty) => ty.to_string(),
            Type::StructDef(_) => "}".to_string(),
        };

        f.write_str(&data)
    }
}

#[derive(Clone, Debug)]
pub struct PathType {
    pub path_parts: Vec<Ident>,
    pub dot_tkns: Vec<TokCtx>,
    pub name: Ident,
}

impl Display for PathType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut data = String::new();
        for part in &self.path_parts {
            data.push_str(part);
            data.push('.');
        }
        data.push_str(&self.name);
        f.write_str(&data)
    }
}

#[derive(Clone, Debug)]
pub struct ArrayType {
    pub component: Box<Type>,
    pub brack1_tkn: TokCtx,
    pub dimension: Option<u32>,
    pub brack2_tkn: TokCtx,
}

impl Display for ArrayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut data = self.component.to_string();
        data.push('[');
        if let Some(dim) = self.dimension {
            data.push_str(&format!("{dim}"));
        }
        data.push(']');
        f.write_str(&data)
    }
}

#[derive(Copy, Clone, Debug, TryFromString)]
pub enum PrimitiveType {
    #[casing(Lower)]
    F32,
    #[casing(Lower)]
    F64,
    #[casing(Lower)]
    Int,
    #[casing(Lower)]
    Uint,
    #[casing(Lower)]
    Bool,
    #[casing(Lower)]
    Vec2,
    #[casing(Lower)]
    Vec3,
    #[casing(Lower)]
    Vec4,
    #[casing(Lower)]
    IVec2,
    #[casing(Lower)]
    IVec3,
    #[casing(Lower)]
    IVec4,
    #[casing(Lower)]
    DVec2,
    #[casing(Lower)]
    DVec3,
    #[casing(Lower)]
    DVec4,
    #[casing(Lower)]
    BVec2,
    #[casing(Lower)]
    BVec3,
    #[casing(Lower)]
    BVec4,
    #[casing(Lower)]
    UVec2,
    #[casing(Lower)]
    UVec3,
    #[casing(Lower)]
    UVec4,
    #[casing(Lower)]
    Mat2,
    #[casing(Lower)]
    Mat3,
    #[casing(Lower)]
    Mat4,
    #[casing(Lower)]
    Mat2x3,
    #[casing(Lower)]
    Mat2x4,
    #[casing(Lower)]
    Mat3x2,
    #[casing(Lower)]
    Mat3x4,
    #[casing(Lower)]
    Mat4x2,
    #[casing(Lower)]
    Mat4x3,
    #[casing(Lower)]
    DMat2,
    #[casing(Lower)]
    DMat3,
    #[casing(Lower)]
    DMat4,
    #[casing(Lower)]
    DMat2x3,
    #[casing(Lower)]
    DMat2x4,
    #[casing(Lower)]
    DMat3x2,
    #[casing(Lower)]
    DMat3x4,
    #[casing(Lower)]
    DMat4x2,
    #[casing(Lower)]
    DMat4x3,
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::Int => "int",
            Self::Uint => "uint",
            Self::Bool => "bool",
            Self::Vec2 => "vec2",
            Self::Vec3 => "vec3",
            Self::Vec4 => "vec4",
            Self::IVec2 => "ivec2",
            Self::IVec3 => "ivec3",
            Self::IVec4 => "ivec4",
            Self::DVec2 => "dvec2",
            Self::DVec3 => "dvec3",
            Self::DVec4 => "dvec4",
            Self::BVec2 => "bvec2",
            Self::BVec3 => "bvec3",
            Self::BVec4 => "bvec4",
            Self::UVec2 => "uvec2",
            Self::UVec3 => "uvec3",
            Self::UVec4 => "uvec4",
            Self::Mat2 => "mat2",
            Self::Mat3 => "mat3",
            Self::Mat4 => "mat4",
            Self::Mat2x3 => "mat2x3",
            Self::Mat2x4 => "mat2x4",
            Self::Mat3x2 => "mat3x2",
            Self::Mat3x4 => "mat3x4",
            Self::Mat4x2 => "mat4x2",
            Self::Mat4x3 => "mat4x3",
            Self::DMat2 => "dmat2",
            Self::DMat3 => "dmat3",
            Self::DMat4 => "dmat4",
            Self::DMat2x3 => "dmat2x3",
            Self::DMat2x4 => "dmat2x4",
            Self::DMat3x2 => "dmat3x2",
            Self::DMat3x4 => "dmat3x4",
            Self::DMat4x2 => "dmat4x2",
            Self::DMat4x3 => "dmat4x3",
        };
        f.write_str(data)
    }
}

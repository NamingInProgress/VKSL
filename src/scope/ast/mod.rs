use crate::{parser, token};
use crate::parser::ast::PrimitiveType;
use crate::scope::ast::stmt::Stmt;
use crate::scope::ast::ty::{ArrayType, PrimitiveRef, StructRef, Type};
use crate::scope::SymbolId;
use crate::token::TokCtx;

pub mod ty;
pub mod expr;
pub mod stmt;

pub type Ast = Vec<Stmt>;

#[derive(Clone, Debug)]
pub struct Ident {
    pub val: String,
    pub tkn: TokCtx,
    pub symbol: SymbolId
}

impl Ident {
    pub fn convert(old: parser::ast::Ident, symbol: SymbolId) -> Self {
        Self {
            val: old.val,
            tkn: old.tkn,
            symbol,
        }
    }
    
    pub fn plain(old: parser::ast::Ident) -> Self {
        Self {
            val: old.val,
            tkn: old.tkn,
            symbol: SymbolId::MAX,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    BoolLit(bool),
    IntLit(i32),
    UIntLit(u32),
    FloatLit(f32),
    DoubleLit(f64),

    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    IVec2([i32; 2]),
    IVec3([i32; 3]),
    IVec4([i32; 4]),
    BVec2([bool; 2]),
    BVec3([bool; 3]),
    BVec4([bool; 4]),
    DVec2([f64; 2]),
    DVec3([f64; 3]),
    DVec4([f64; 4]),
    UVec2([u32; 2]),
    UVec3([u32; 3]),
    UVec4([u32; 4]),
    Mat2([[f32; 2]; 2]),
    Mat3([[f32; 3]; 3]),
    Mat4([[f32; 4]; 4]),
    Mat2x3([[f32; 2]; 3]),
    Mat2x4([[f32; 2]; 4]),
    Mat3x2([[f32; 3]; 2]),
    Mat3x4([[f32; 3]; 4]),
    Mat4x2([[f32; 4]; 2]),
    Mat4x3([[f32; 4]; 3]),
    DMat2([[f64; 2]; 2]),
    DMat3([[f64; 3]; 3]),
    DMat4([[f64; 4]; 4]),
    DMat2x3([[f64; 2]; 3]),
    DMat2x4([[f64; 2]; 4]),
    DMat3x2([[f64; 3]; 2]),
    DMat3x4([[f64; 3]; 4]),
    DMat4x2([[f64; 4]; 2]),
    DMat4x3([[f64; 4]; 3]),

    ArrayLit {
        items: Vec<Literal>,
        len: u32,
        ty: Box<Type>,
    },
    TupleLit {
        items: Vec<Literal>,
        ty: Box<Type>,
    },
    StructLit {
        items: Vec<(String, Literal)>,
        ty: Box<Type>,
    },

}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::BoolLit(a), Literal::BoolLit(b)) => a == b,
            (Literal::IntLit(a), Literal::IntLit(b)) => a == b,
            (Literal::UIntLit(a), Literal::UIntLit(b)) => a == b,
            (Literal::FloatLit(a), Literal::FloatLit(b)) => a == b,
            (Literal::DoubleLit(a), Literal::DoubleLit(b)) => a == b,
            (Literal::ArrayLit { items, len, .. }, Literal::ArrayLit { items: items2, len: len2, .. }) => len == len2 && items == items2,
            (Literal::TupleLit { items, .. }, Literal::TupleLit { items: items2, .. }) => items == items2,
            (Literal::StructLit { ty, items }, Literal::StructLit { ty: ty2, items: items2 }) => ty == ty2 && items == items2,
            _ => false,
        }
    }
}

impl From<token::Literal> for Literal {
    fn from(lit: token::Literal) -> Self {
        match lit {
            token::Literal::BoolLit(val) => Literal::BoolLit(val),
            token::Literal::IntLit(val) => Literal::IntLit(val),
            token::Literal::UIntLit(val) => Literal::UIntLit(val),
            token::Literal::FloatLit(val) => Literal::FloatLit(val),
            token::Literal::DoubleLit(val) => Literal::DoubleLit(val),
        }
    }
}

impl Literal {
    pub fn ty(&self) -> Type {
        match self {
            Literal::BoolLit(_) => PrimitiveType::Bool.into(),
            Literal::IntLit(_) => PrimitiveType::Int.into(),
            Literal::UIntLit(_) => PrimitiveType::Uint.into(),
            Literal::FloatLit(_) => PrimitiveType::F32.into(),
            Literal::DoubleLit(_) => PrimitiveType::F64.into(),

            Literal::Vec2(_) => PrimitiveType::Vec2.into(),
            Literal::Vec3(_) => PrimitiveType::Vec3.into(),
            Literal::Vec4(_) => PrimitiveType::Vec4.into(),
            Literal::IVec2(_) => PrimitiveType::IVec2.into(),
            Literal::IVec3(_) => PrimitiveType::IVec3.into(),
            Literal::IVec4(_) => PrimitiveType::IVec4.into(),
            Literal::BVec2(_) => PrimitiveType::BVec2.into(),
            Literal::BVec3(_) => PrimitiveType::BVec3.into(),
            Literal::BVec4(_) => PrimitiveType::BVec4.into(),
            Literal::DVec2(_) => PrimitiveType::DVec2.into(),
            Literal::DVec3(_) => PrimitiveType::DVec3.into(),
            Literal::DVec4(_) => PrimitiveType::DVec4.into(),
            Literal::UVec2(_) => PrimitiveType::UVec2.into(),
            Literal::UVec3(_) => PrimitiveType::UVec3.into(),
            Literal::UVec4(_) => PrimitiveType::UVec4.into(),
            Literal::Mat2(_) => PrimitiveType::Mat2.into(),
            Literal::Mat3(_) => PrimitiveType::Mat3.into(),
            Literal::Mat4(_) => PrimitiveType::Mat4.into(),
            Literal::Mat2x3(_) => PrimitiveType::Mat2x3.into(),
            Literal::Mat2x4(_) => PrimitiveType::Mat2x4.into(),
            Literal::Mat3x2(_) => PrimitiveType::Mat3x2.into(),
            Literal::Mat3x4(_) => PrimitiveType::Mat3x4.into(),
            Literal::Mat4x2(_) => PrimitiveType::Mat4x2.into(),
            Literal::Mat4x3(_) => PrimitiveType::Mat4x3.into(),
            Literal::DMat2(_) => PrimitiveType::DMat2.into(),
            Literal::DMat3(_) => PrimitiveType::DMat3.into(),
            Literal::DMat4(_) => PrimitiveType::DMat4.into(),
            Literal::DMat2x3(_) => PrimitiveType::DMat2x3.into(),
            Literal::DMat2x4(_) => PrimitiveType::DMat2x4.into(),
            Literal::DMat3x2(_) => PrimitiveType::DMat3x2.into(),
            Literal::DMat3x4(_) => PrimitiveType::DMat3x4.into(),
            Literal::DMat4x2(_) => PrimitiveType::DMat4x2.into(),
            Literal::DMat4x3(_) => PrimitiveType::DMat4x3.into(),

            Literal::ArrayLit { ty, .. } => *ty.clone(),
            Literal::TupleLit { ty, .. } => *ty.clone(),
            Literal::StructLit { ty, .. } => *ty.clone(),
        }
    }

    pub fn is_swizzleable(&self) -> bool {
        matches!(self,
            Literal::Vec2(_) | Literal::Vec3(_) | Literal::Vec4(_) |
            Literal::IVec2(_) | Literal::IVec3(_) | Literal::IVec4(_) |
            Literal::BVec2(_) | Literal::BVec3(_) | Literal::BVec4(_) |
            Literal::DVec2(_) | Literal::DVec3(_) | Literal::DVec4(_) |
            Literal::UVec2(_) | Literal::UVec3(_) | Literal::UVec4(_) |
            Literal::Mat2(_) | Literal::Mat3(_) | Literal::Mat4(_) |
            Literal::Mat2x3(_) | Literal::Mat2x4(_) | Literal::Mat3x2(_) |
            Literal::Mat3x4(_) | Literal::Mat4x2(_) | Literal::Mat4x3(_) |
            Literal::DMat2(_) | Literal::DMat3(_) | Literal::DMat4(_) |
            Literal::DMat2x3(_) | Literal::DMat2x4(_) | Literal::DMat3x2(_) |
            Literal::DMat3x4(_) | Literal::DMat4x2(_) | Literal::DMat4x3(_)
        )
    }

    pub fn swizzle(&self, parts: Vec<usize>) -> Result<Literal, usize> {
        macro_rules! swizzle {
            ($value:expr, $n:expr, $v1:ident, $v2:ident, $v3:ident, $v4:ident) => {{
                let value = $value;

                for (i, &part) in parts.iter().enumerate() {
                    if part as usize >= $n {
                        return Err(i);
                    }
                }

                Ok(match parts.len() {
                    1 => Literal::$v1(value[parts[0]]),
                    2 => Literal::$v2([value[parts[0]], value[parts[1]]]),
                    3 => Literal::$v3([value[parts[0]], value[parts[1]], value[parts[2]]]),
                    4 => Literal::$v4([value[parts[0]], value[parts[1]], value[parts[2]], value[parts[3]]]),
                    _ => unreachable!(),
                })
            }};
        }

        match self {
            Literal::Vec2(v)  => swizzle!(*v, 2, FloatLit, Vec2, Vec3, Vec4),
            Literal::Vec3(v)  => swizzle!(*v, 3, FloatLit, Vec2, Vec3, Vec4),
            Literal::Vec4(v)  => swizzle!(*v, 4, FloatLit, Vec2, Vec3, Vec4),

            Literal::IVec2(v) => swizzle!(*v, 2, IntLit, IVec2, IVec3, IVec4),
            Literal::IVec3(v) => swizzle!(*v, 3, IntLit, IVec2, IVec3, IVec4),
            Literal::IVec4(v) => swizzle!(*v, 4, IntLit, IVec2, IVec3, IVec4),

            Literal::BVec2(v) => swizzle!(*v, 2, BoolLit, BVec2, BVec3, BVec4),
            Literal::BVec3(v) => swizzle!(*v, 3, BoolLit, BVec2, BVec3, BVec4),
            Literal::BVec4(v) => swizzle!(*v, 4, BoolLit, BVec2, BVec3, BVec4),

            Literal::DVec2(v) => swizzle!(*v, 2, DoubleLit, DVec2, DVec3, DVec4),
            Literal::DVec3(v) => swizzle!(*v, 3, DoubleLit, DVec2, DVec3, DVec4),
            Literal::DVec4(v) => swizzle!(*v, 4, DoubleLit, DVec2, DVec3, DVec4),

            Literal::UVec2(v) => swizzle!(*v, 2, UIntLit, UVec2, UVec3, UVec4),
            Literal::UVec3(v) => swizzle!(*v, 3, UIntLit, UVec2, UVec3, UVec4),
            Literal::UVec4(v) => swizzle!(*v, 4, UIntLit, UVec2, UVec3, UVec4),

            Literal::Mat2(v) => swizzle!(*v, 2, Vec2, Mat2, Mat2x3, Mat2x4),
            Literal::Mat3(v) => swizzle!(*v, 3, Vec3, Mat3x2, Mat3, Mat3x4),
            Literal::Mat4(v) => swizzle!(*v, 4, Vec4, Mat4x2, Mat4x3, Mat4),
            Literal::Mat2x3(v) => swizzle!(*v, 3, Vec2, Mat2, Mat2x3, Mat2x4),
            Literal::Mat2x4(v) => swizzle!(*v, 4, Vec2, Mat2, Mat2x3, Mat2x4),
            Literal::Mat3x2(v) => swizzle!(*v, 2, Vec3, Mat3x2, Mat3, Mat3x4),
            Literal::Mat3x4(v) => swizzle!(*v, 4, Vec3, Mat3x2, Mat3, Mat3x4),
            Literal::Mat4x2(v) => swizzle!(*v, 2, Vec4, Mat4x2, Mat4x3, Mat4),
            Literal::Mat4x3(v) => swizzle!(*v, 3, Vec4, Mat4x2, Mat4x3, Mat4),

            Literal::DMat2(v) => swizzle!(*v, 2, DVec2, DMat2, DMat2x3, DMat2x4),
            Literal::DMat3(v) => swizzle!(*v, 3, DVec3, DMat3x2, DMat3, DMat3x4),
            Literal::DMat4(v) => swizzle!(*v, 4, DVec4, DMat4x2, DMat4x3, DMat4),
            Literal::DMat2x3(v) => swizzle!(*v, 3, DVec2, DMat2, DMat2x3, DMat2x4),
            Literal::DMat2x4(v) => swizzle!(*v, 4, DVec2, DMat2, DMat2x3, DMat2x4),
            Literal::DMat3x2(v) => swizzle!(*v, 2, DVec3, DMat3x2, DMat3, DMat3x4),
            Literal::DMat3x4(v) => swizzle!(*v, 4, DVec3, DMat3x2, DMat3, DMat3x4),
            Literal::DMat4x2(v) => swizzle!(*v, 2, DVec4, DMat4x2, DMat4x3, DMat4),
            Literal::DMat4x3(v) => swizzle!(*v, 3, DVec4, DMat4x2, DMat4x3, DMat4),

            _ => unreachable!()
        }
    }

    pub fn max_swizzle_size(&self) -> usize {
        match self {
            Literal::Vec2(_) |
            Literal::IVec2(_) |
            Literal::BVec2(_) |
            Literal::DVec2(_) |
            Literal::UVec2(_) |
            Literal::Mat2(_) |
            Literal::Mat3x2(_) |
            Literal::Mat4x2(_) |
            Literal::DMat2(_) |
            Literal::DMat3x2(_) |
            Literal::DMat4x2(_) => 2,

            Literal::Vec3(_) |
            Literal::IVec3(_) |
            Literal::BVec3(_) |
            Literal::DVec3(_) |
            Literal::UVec3(_) |
            Literal::Mat3(_) |
            Literal::Mat2x3(_) |
            Literal::Mat4x3(_) |
            Literal::DMat3(_) |
            Literal::DMat2x3(_) |
            Literal::DMat4x3(_) => 3,

            Literal::Vec4(_) |
            Literal::IVec4(_) |
            Literal::BVec4(_) |
            Literal::DVec4(_) |
            Literal::UVec4(_) |
            Literal::Mat4(_) |
            Literal::Mat2x4(_) |
            Literal::Mat3x4(_) |
            Literal::DMat4(_) |
            Literal::DMat2x4(_) |
            Literal::DMat3x4(_) => 4,

            _ => unreachable!()
        }
    }
}

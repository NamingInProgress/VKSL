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
    Mat4x2([[f32; 4]; 4]),
    Mat4x3([[f32; 4]; 4]),
    DMat2([[f32; 2]; 2]),
    DMat3([[f32; 3]; 3]),
    DMat4([[f32; 4]; 4]),
    DMat2x3([[f32; 2]; 3]),
    DMat2x4([[f32; 2]; 4]),
    DMat3x2([[f32; 3]; 2]),
    DMat3x4([[f32; 3]; 4]),
    DMat4x2([[f32; 4]; 4]),
    DMat4x3([[f32; 4]; 4]),

    ArrayLit {
        items: Vec<Literal>,
        len: u32,
        // will always be some if len > 0
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
            Literal::BoolLit(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Bool,
                tkn: TokCtx::empty(),
            }),
            Literal::IntLit(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Int,
                tkn: TokCtx::empty(),
            }),
            Literal::UIntLit(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Uint,
                tkn: TokCtx::empty(),
            }),
            Literal::FloatLit(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::F32,
                tkn: TokCtx::empty(),
            }),
            Literal::DoubleLit(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::F64,
                tkn: TokCtx::empty(),
            }),

            Literal::Vec2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Vec2,
                tkn: TokCtx::empty(),
            }),
            Literal::Vec3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Vec3,
                tkn: TokCtx::empty(),
            }),
            Literal::Vec4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Vec4,
                tkn: TokCtx::empty(),
            }),
            Literal::IVec2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::IVec2,
                tkn: TokCtx::empty(),
            }),
            Literal::IVec3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::IVec3,
                tkn: TokCtx::empty(),
            }),
            Literal::IVec4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::IVec4,
                tkn: TokCtx::empty(),
            }),
            Literal::BVec2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::BVec2,
                tkn: TokCtx::empty(),
            }),
            Literal::BVec3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::BVec3,
                tkn: TokCtx::empty(),
            }),
            Literal::BVec4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::BVec4,
                tkn: TokCtx::empty(),
            }),
            Literal::DVec2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DVec2,
                tkn: TokCtx::empty(),
            }),
            Literal::DVec3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DVec3,
                tkn: TokCtx::empty(),
            }),
            Literal::DVec4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DVec4,
                tkn: TokCtx::empty(),
            }),
            Literal::UVec2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::UVec2,
                tkn: TokCtx::empty(),
            }),
            Literal::UVec3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::UVec3,
                tkn: TokCtx::empty(),
            }),
            Literal::UVec4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::UVec4,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat2,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat3,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat4,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat2x3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat2x3,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat2x4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat2x4,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat3x2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat3x2,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat3x4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat3x4,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat4x2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat4x2,
                tkn: TokCtx::empty(),
            }),
            Literal::Mat4x3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::Mat4x3,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat2,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat3,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat4,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat2x3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat2x3,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat2x4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat2x4,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat3x2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat3x2,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat3x4(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat3x4,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat4x2(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat4x2,
                tkn: TokCtx::empty(),
            }),
            Literal::DMat4x3(_) => Type::Primitive(PrimitiveRef {
                ty: PrimitiveType::DMat4x3,
                tkn: TokCtx::empty(),
            }),


            Literal::ArrayLit { ty, .. } => *ty.clone(),
            Literal::TupleLit { ty, .. } => *ty.clone(),
            Literal::StructLit { ty, .. } => *ty.clone(),
        }
    }
}

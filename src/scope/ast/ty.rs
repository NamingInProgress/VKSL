use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::parser::ast::ty::PrimitiveType;
use crate::scope::ast::expr::Expr;
use crate::scope::ast::stmt::MethodDeclStmt;
use crate::scope::{SharedScope, SymbolId};
use crate::token::TokCtx;

pub type SharedType = Rc<RefCell<Type>>;

pub enum CastTarget {
    F32,
    F64,
    Int,
    Uint,
    Bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    // ONLY used for 0 length array at initialization
    Unknown,
    Primitive(PrimitiveRef),
    ArrayOf(ArrayType),
    StructRef(StructRef),
    StructDef(StructDef),
    Tuple(TupleDef),
}

impl Type {
    pub fn as_raw(&self) -> String {
        match self {
            Type::Unknown => "()".to_string(),
            Type::Primitive(ty) => ty.to_string(),
            Type::StructRef(ty) => ty.to_string(),
            Type::ArrayOf(ty) => ty.as_raw(),
            Type::Tuple(ty) => ty.as_raw(),
            Type::StructDef(st) => st.to_string(),
        }
    }

    pub fn as_cast(&self) -> Option<CastTarget> {
        let Type::Primitive(ty) = self else { return None; };
        match ty.ty {
            PrimitiveType::F32 => Some(CastTarget::F32),
            PrimitiveType::F64 => Some(CastTarget::F64),
            PrimitiveType::Int => Some(CastTarget::Int),
            PrimitiveType::Uint => Some(CastTarget::Uint),
            PrimitiveType::Bool => Some(CastTarget::Bool),
            _ => None
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Type::Unknown => "()".to_string(),
            Type::Primitive(ty) => ty.to_string(),
            Type::StructRef(ty) => ty.to_string(),
            Type::ArrayOf(ty) => ty.to_string(),
            Type::Tuple(ty) => ty.to_string(),
            Type::StructDef(st) => st.to_string(),
        };

        f.write_str(&data)
    }
}

#[derive(Clone, Debug)]
pub struct PrimitiveRef {
    pub ty: PrimitiveType,
    pub tkn: TokCtx
}

impl PartialEq for PrimitiveRef {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}

impl Eq for PrimitiveRef {}

impl Display for PrimitiveRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.ty, f)
    }
}

#[derive(Clone, Debug)]
pub struct ArrayType {
    pub component: Box<Type>,
    pub brack1_tkn: TokCtx,
    pub dimension: Option<Box<Expr>>,
    pub evaluated: Option<u32>,
    pub evaluated_tok: Option<TokCtx>,
    pub brack2_tkn: TokCtx,
    pub default_size: Option<u32>,
    pub default_size_tkn: Option<TokCtx>,
    pub default_kw_tkn: Option<TokCtx>,
}

impl PartialEq for ArrayType {
    fn eq(&self, other: &Self) -> bool {
        if let Some(this_size) = self.evaluated && let Some(other_size) = other.evaluated {
            if this_size == other_size {
                if this_size == 0 {
                    &*self.component == &Type::Unknown || &*other.component == &Type::Unknown || self.component == other.component
                } else {
                    self.component == other.component
                }
            } else { false }
        } else {
            panic!("Hello bear, you must not use type comparisons before constants are evaluated!")
        }
    }
}

impl Eq for ArrayType {}

impl Display for ArrayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut data = self.component.to_string();
        data.push('[');
        if let Some(evaluated) = self.evaluated {
            data.push_str(&*evaluated.to_string());
        } else if let Some(dim) = &self.dimension {
            data.push_str(&format!("{dim:?}"));
        } else if let Some(dim) = &self.default_size {
            data.push_str(&format!("default {}", *dim));
        }
        data.push(']');
        f.write_str(&data)
    }
}

impl ArrayType {
    pub(crate) fn as_raw(&self) -> String {
        let mut data = self.component.to_string();
        data.push('[');
        if let Some(evaluated) = &self.evaluated_tok {
            data.push_str(&evaluated.get_parts().2);
        } else if let Some(evaluated) = self.evaluated {
            data.push_str(&*evaluated.to_string());
        } else if let Some(dim) = &self.dimension {
            data.push_str(&format!("{dim:?}"));
        } else if let Some(dim) = &self.default_size {
            data.push_str(&format!("default {}", *dim));
        }
        data.push(']');
        data
    }
}

#[derive(Clone, Debug)]
pub struct StructRef {
    pub sym: SymbolId,
    pub name_tkn: TokCtx,

    pub debug_name: String,
}

impl PartialEq for StructRef {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym
    }
}

impl Eq for StructRef {}

impl Display for StructRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.debug_name)
    }
}

#[derive(Clone, Debug)]
pub struct StructDef {
    pub sym: SymbolId,
    pub scope: SharedScope,
    pub methods: Vec<MethodDeclStmt>,
    pub struct_tkn: TokCtx,
    pub name_tkn: TokCtx,
    pub brace1_tkn: TokCtx,
    pub brace2_tkn: TokCtx,

    pub debug_name: String,
}

impl PartialEq for StructDef {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym
    }
}

impl Eq for StructDef {}

impl Display for StructDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.debug_name)?;
        f.write_str(" { .. }")
    }
}

#[derive(Clone, Debug)]
pub struct TupleDef {
    pub paren1_tok: TokCtx,
    pub types: Vec<(Type, Option<TokCtx>)>,
    pub paren2_tok: TokCtx,
}

impl PartialEq for TupleDef {
    fn eq(&self, other: &Self) -> bool {
        if self.types.len() == other.types.len() {
            for i in 0..self.types.len() {
                if self.types[i].0 != other.types[i].0 {
                    return false;
                }
            }
            true
        } else { false }
    }
}

impl Eq for TupleDef {}

impl Display for TupleDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        for (ty, tok) in &self.types {
            Display::fmt(ty, f)?;
            if tok.is_some() {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")
    }
}

impl TupleDef {
    pub(crate) fn as_raw(&self) -> String {
        let mut data = "(".to_string();
        for (ty, tok) in &self.types {
            data.push_str(&ty.as_raw());
            if tok.is_some() {
                data.push_str(", ");
            }
        }
        data.push(')');
        data
    }
}
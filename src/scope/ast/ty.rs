use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::ast::{MethodDeclStmt, StructField};
use crate::parser::ast::ty::PrimitiveType;
use crate::scope::ast::expr::Expr;
use crate::scope::SymbolId;
use crate::token::TokCtx;

pub type SharedType = Rc<RefCell<Type>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Primitive(PrimitiveRef),
    ArrayOf(ArrayType),
    StructRef(StructRef),
    StructDef(StructDef),
    Tuple(TupleDef),
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

#[derive(Clone, Debug)]
pub struct ArrayType {
    pub component: Box<Type>,
    pub brack1_tkn: TokCtx,
    pub dimension: Option<Box<Expr>>,
    pub evaluated: Option<u32>,
    pub brack2_tkn: TokCtx,
}

impl PartialEq for ArrayType {
    fn eq(&self, other: &Self) -> bool {
        if let Some(this_size) = self.evaluated && let Some(other_size) = other.evaluated {
            this_size == other_size && self.component == other.component
        } else {
            panic!("Hello bear, you must not use type comparisons before constants are evaluated!")
        }
    }
}

impl Eq for ArrayType {}

#[derive(Clone, Debug)]
pub struct StructRef {
    pub sym: SymbolId,
    pub name_tkn: TokCtx
}

impl PartialEq for StructRef {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym
    }
}

impl Eq for StructRef {}

#[derive(Clone, Debug)]
pub struct StructDef {
    pub sym: SymbolId,
    pub struct_tkn: TokCtx,
    pub name_tkn: TokCtx,
    pub brace1_tkn: TokCtx,
    pub brace2_tkn: TokCtx,
}

impl PartialEq for StructDef {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym
    }
}

impl Eq for StructDef {}

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
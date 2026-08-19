use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::ast::stmt::Stmt;
use crate::scope::SymbolId;
use crate::token::TokCtx;

pub mod expr;
pub mod stmt;
pub mod ty;

pub type Ast = Vec<Stmt>;

#[derive(Clone, Debug)]
pub struct Ident {
    pub val: String,
    pub tkn: TokCtx,
    pub resolved_ident: Option<SymbolId>,
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.val)
    }
}

impl Deref for Ident {
    type Target = String;
    
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl From<(String, TokCtx)> for Ident {
    fn from(ty: (String, TokCtx)) -> Self {
        Self {
            val: ty.0,
            tkn: ty.1,
            resolved_ident: None
        }
    }
}
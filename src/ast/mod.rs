use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::token::TokCtx;

pub mod expr;
pub mod stmt;
pub mod ty;

#[derive(Clone, Debug)]
pub struct Ident {
    pub val: String,
    pub tkn: TokCtx
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
        }
    }
}
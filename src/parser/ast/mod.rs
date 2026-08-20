use crate::token::TokCtx;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub mod expr;
pub mod stmt;
pub mod ty;

pub use expr::*;
pub use stmt::*;
pub use ty::*;
use crate::scope::SymbolName;

pub type Ast = Vec<Stmt>;

#[derive(Clone, Debug)]
pub struct Ident {
    pub val: String,
    pub tkn: TokCtx,
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

impl SymbolName for Ident {
    fn get_name(&self) -> &String {
        &self.val
    }

    fn get_error_token(&self) -> TokCtx {
        self.tkn.clone()
    }
}
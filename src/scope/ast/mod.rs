use crate::parser;
use crate::parser::ast::Stmt;
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
use crate::token;
use crate::token::{Token, TokenType};
use crate::token::tokenizer::TokenErr;

pub mod expr;

#[derive(Debug)]
pub enum ParseErr {
    TokenizerErr(TokenErr),
    UnexpectedEof,
    UnexpectedToken(Token, Vec<TokenExpectation>),
    InternalErr
}

#[derive(Debug)]
pub enum TokenExpectation {
    Exact(TokenType),
    AnyIdentifier,
    AnyType
}

pub type Result<T> = core::result::Result<T, ParseErr>;
pub type TokenRes = token::tokenizer::Result;
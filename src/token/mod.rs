pub mod tokenizer;
pub mod utils;

use std::fmt::{Debug, Formatter};
use crate::parser;
use crate::parser::err::{ParseErr, ParseErrType};
use crate::token::tokenizer::History;
use mvutils_proc_macro::TryFromString;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub struct TokCtx {
    pub line: u32,
    pub start_pos: u32,
    pub end_pos: u32,
    pub file: Option<PathBuf>,
    pub history: History,
}

impl Debug for TokCtx {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("<tctx>")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub ctx: TokCtx,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    //symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semi,
    Colon,
    Question,

    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    OperatorAssign(Operator),

    Ident(String),
}

#[derive(Copy, Clone, Debug, PartialEq, TryFromString)]
#[repr(u8)]
pub enum Keyword {
    //MODIFIERS DO NOT REORDER
    #[casing(Lower)]
    Flat,
    #[casing(Lower)]
    Smooth,
    #[casing(Lower)]
    Noperspective,
    #[casing(Lower)]
    Enable,
    #[casing(Lower)]
    Require,
    #[casing(Lower)]
    Warn,
    #[casing(Lower)]
    Disable,
    #[casing(Lower)]
    Readonly,
    #[casing(Lower)]
    Writeonly,
    #[custom("std140")]
    STD140,
    #[custom("std430")]
    STD430,

    #[casing(Lower)]
    Fn,
    #[casing(Lower)]
    Struct,
    #[casing(Lower)]
    If,
    #[casing(Lower)]
    Else,
    #[casing(Lower)]
    While,
    #[casing(Lower)]
    For,
    #[casing(Lower)]
    Return,
    #[casing(Lower)]
    Let,
    #[casing(Lower)]
    Const,
    #[casing(Lower)]
    Include,
    #[casing(Lower)]
    Extension,
    #[casing(Lower)]
    Input,
    #[casing(Lower)]
    Output,
    #[casing(Lower)]
    Provide,
    #[custom("push_constants")]
    PushConstants,
    #[casing(Lower)]
    Uniform,
    #[casing(Lower)]
    Buffer,
    #[casing(Lower)]
    Break,
    #[casing(Lower)]
    Continue,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Literal {
    BoolLit(bool),
    IntLit(i32),
    UIntLit(u32),
    FloatLit(f32),
    DoubleLit(f64),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operator {
    Assign,
    Plus,
    Minus,
    Mul,
    Div,
    Dot,
    Modulo,
    PlusPlus,
    MinusMinus,
    BitOr,
    BitAnd,
    BitXor,
    BitNegate,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    And,
    Or,
    EqEq,
    Neq,
    Not,
    Lsh,
    Rsh,
    LogicalRsh,
    Merge,
}

impl Operator {
    pub fn precedence(
        &self,
        ctx: &TokCtx,
        tail_gen: impl FnOnce() -> String,
    ) -> parser::Result<u8> {
        match self {
            Operator::Not | Operator::PlusPlus | Operator::MinusMinus => Ok(8),
            Operator::Mul | Operator::Div | Operator::Modulo => Ok(7),
            Operator::Plus | Operator::Minus => Ok(6),
            Operator::Lsh | Operator::LogicalRsh | Operator::Rsh => Ok(5),
            Operator::Less | Operator::Greater | Operator::LessEq | Operator::GreaterEq => Ok(4),
            Operator::EqEq | Operator::Neq => Ok(3),
            Operator::BitAnd | Operator::BitOr | Operator::BitXor | Operator::BitNegate => Ok(2),
            Operator::And | Operator::Or => Ok(1),
            Operator::Merge => Ok(0),
            op => Err(ParseErr {
                ty: ParseErrType::NonBinaryCompatibleOperator(*self),
                ctx: ctx.clone(),
                tail: tail_gen(),
                hint: (*op == Operator::Assign).then_some(
                    "Consider moving variable assignment to it's own statement".to_string(),
                ),
            }),
        }
    }
}

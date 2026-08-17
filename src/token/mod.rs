pub mod tokenizer;

use mvutils_proc_macro::TryFromString;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub line: u32,
    pub start_pos: u32,
    pub end_pos: u32,
    pub file: Option<PathBuf>
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
    LArrow,
    RArrow,

    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    OperatorAssign(Operator),

    Ident(String)
}

#[derive(Clone, Debug, PartialEq, TryFromString)]
pub enum Keyword {
    #[casing(Lower)] Fn,
    #[casing(Lower)] Struct,
    #[casing(Lower)] If,
    #[casing(Lower)] Else,
    #[casing(Lower)] While,
    #[casing(Lower)] For,
    #[casing(Lower)] Return,
    #[casing(Lower)] Let,
    #[casing(Lower)] Include,
    #[casing(Lower)] Extension,
    #[casing(Lower)] Enable,
    #[casing(Lower)] Require,
    #[casing(Lower)] Warn,
    #[casing(Lower)] Disable,
    #[casing(Lower)] Input,
    #[casing(Lower)] Output,
    #[casing(Lower)] Provide,
    #[custom("push_constants")] PushConstants,
    #[casing(Lower)] Uniform,
    #[casing(Lower)] Buffer,
    #[custom("std430")] STD430,
    #[custom("std140")] STD140,
    #[casing(Lower)] Readonly,
    #[casing(Lower)] Writeonly,
    #[casing(Lower)] Break,
    #[casing(Lower)] Continue,
    #[casing(Lower)] Flat,
    #[casing(Lower)] Smooth,
    #[casing(Lower)] Noperspective,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    BoolLit(bool),
    IntLit(i32),
    UIntLit(u32),
    FloatLit(f32),
    DoubleLit(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
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
    Less,
    And,
    Or,
    Eq,
    Not,
    Lsh,
    Rsh,
    LogicalRsh
}
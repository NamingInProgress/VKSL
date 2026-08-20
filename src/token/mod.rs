pub mod tokenizer;
pub mod utils;

use std::fmt::{Debug, Display, Formatter};
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
    Yield,
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
    #[casing(Lower)]
    Nonuniform,
    #[casing(Lower)]
    This,
    #[casing(Lower)]
    As,
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

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LParen => f.write_str("("),
            TokenType::RParen => f.write_str(")"),
            TokenType::LBrace => f.write_str("{"),
            TokenType::RBrace => f.write_str("}"),
            TokenType::LBracket => f.write_str("["),
            TokenType::RBracket => f.write_str("]"),
            TokenType::Comma => f.write_str(","),
            TokenType::Semi => f.write_str(";"),
            TokenType::Colon => f.write_str(":"),
            TokenType::Question => f.write_str("?"),
            TokenType::Keyword(kw) => write!(f, "{kw}"),
            TokenType::Literal(lit) => write!(f, "{lit}"),
            TokenType::Operator(op) => write!(f, "{op}"),
            TokenType::OperatorAssign(oa) => write!(f, "{oa}="),
            TokenType::Ident(ident) => write!(f, "{ident}")
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Keyword::Flat => "flat",
            Keyword::Smooth => "smooth",
            Keyword::Noperspective => "noperspective",
            Keyword::Enable => "enable",
            Keyword::Require => "require",
            Keyword::Warn => "warn",
            Keyword::Disable => "disable",
            Keyword::Readonly => "readonly",
            Keyword::Writeonly => "writeonly",
            Keyword::STD140 => "std140",
            Keyword::STD430 => "std430",
            Keyword::Fn => "fn",
            Keyword::Struct => "struct",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::For => "for",
            Keyword::Return => "return",
            Keyword::Yield => "yield",
            Keyword::Let => "let",
            Keyword::Const => "const",
            Keyword::Include => "include",
            Keyword::Extension => "extension",
            Keyword::Input => "input",
            Keyword::Output => "output",
            Keyword::Provide => "provide",
            Keyword::PushConstants => "push_constants",
            Keyword::Uniform => "uniform",
            Keyword::Buffer => "buffer",
            Keyword::Break => "break",
            Keyword::Continue => "continue",
            Keyword::Nonuniform => "nonuniform",
            Keyword::This => "this",
            Keyword::As => "as",
        })
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::BoolLit(b) => Display::fmt(b, f),
            Literal::IntLit(i) => Display::fmt(i, f),
            Literal::UIntLit(u) => Display::fmt(u, f),
            Literal::FloatLit(fl) => Display::fmt(fl, f),
            Literal::DoubleLit(d) => Display::fmt(d, f),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Operator::Assign => "=",
            Operator::Plus => "+",
            Operator::Minus => "_",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Dot => ".",
            Operator::Modulo => "%",
            Operator::PlusPlus => "++",
            Operator::MinusMinus => "--",
            Operator::BitOr => "|",
            Operator::BitAnd => "&",
            Operator::BitXor => "^",
            Operator::BitNegate => "~",
            Operator::Greater => ">",
            Operator::GreaterEq => ">=",
            Operator::Less => "<",
            Operator::LessEq => ">=",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::EqEq => "==",
            Operator::Neq => "!=",
            Operator::Not => "!",
            Operator::Lsh => "<<",
            Operator::Rsh => ">>",
            Operator::LogicalRsh => ">>>",
            Operator::Merge => "->",
        })
    }
}

pub mod tokenizer;
pub mod utils;

use std::fmt::{Debug, Display, Formatter};
use crate::parser;
use crate::parser::err::{create_pad, ParseErr, ParseErrType};
use crate::token::tokenizer::SlidingString;
use mvutils_proc_macro::TryFromString;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub struct TokCtx {
    pub line: u32,
    pub line_pos: u32,
    pub start_pos: u32,
    pub end_pos: u32,
    pub file: Option<PathBuf>,
    pub history: SlidingString,
    pub future: String,

    pub merged_pre_hist: Option<String>,
    pub merged_hist: Option<String>,
    pub merged_start: Option<String>,
    pub merged_end: Option<String>,
}

impl TokCtx {
    pub fn empty() -> Self {
        TokCtx {
            line: 0,
            line_pos: 0,
            start_pos: 0,
            end_pos: 0,
            file: None,
            history: SlidingString::new(0),
            future: String::new(),
            merged_pre_hist: None,
            merged_hist: None,
            merged_start: None,
            merged_end: None,
        }
    }

    pub fn merge(start: TokCtx, end: TokCtx) -> Self {
        const HALF_LIMIT: usize = 24;
        const FULL_LIMIT: usize = HALF_LIMIT * 2 + 2;

        assert_eq!(start.file, end.file);

        fn chars(s: &str) -> Vec<char> {
            s.chars().collect()
        }

        fn limit_string(s: &str, max_len: usize, from_end: bool) -> String {
            let chars = chars(s);

            if chars.len() <= max_len {
                return s.to_string();
            }

            if from_end {
                chars[chars.len() - max_len..].iter().collect()
            } else {
                chars[..max_len].iter().collect()
            }
        }

        fn reconstructed_token(ctx: &TokCtx) -> Option<String> {
            let history = ctx.history.reconstruct();
            let token_len = (ctx.end_pos - ctx.start_pos + 1) as usize;
            let history_len = history.chars().count();

            if history_len < token_len {
                return None;
            }

            Some(
                history
                    .chars()
                    .skip(history_len - token_len)
                    .collect(),
            )
        }

        fn reconstructed_hist(ctx: &TokCtx) -> (Option<String>, Option<String>) {
            let history = ctx.history.reconstruct();

            let token_len = (ctx.end_pos - ctx.start_pos + 1) as usize;
            let history_chars: Vec<char> = history.chars().collect();

            if history_chars.len() < token_len {
                return (None, None);
            }

            let before_token = &history_chars[..history_chars.len() - token_len];

            let last_newline = before_token.iter().rposition(|&c| c == '\n');

            match last_newline {
                Some(nl) => {
                    let current_line: String = before_token[nl + 1..].iter().collect();

                    let pre_hist = before_token[..nl]
                        .iter()
                        .rposition(|&c| c == '\n')
                        .map(|prev_nl| {
                            before_token[prev_nl + 1..nl]
                                .iter()
                                .collect::<String>()
                        })
                        .or_else(|| {
                            if nl > 0 {
                                Some(before_token[..nl].iter().collect())
                            } else {
                                None
                            }
                        });

                    (pre_hist, Some(current_line))
                }
                None => (
                    None,
                    Some(before_token.iter().collect()),
                ),
            }
        }

        fn make_token_parts(token: &str) -> (String, Option<String>) {
            let lines: Vec<&str> = token.split('\n').collect();

            if lines.len() == 1 {
                if token.chars().count() <= FULL_LIMIT {
                    return (token.to_string(), None);
                }

                return (
                    limit_string(token, HALF_LIMIT, false),
                    Some(limit_string(token, HALF_LIMIT, true)),
                );
            }

            let first = limit_string(lines[0], HALF_LIMIT, false);
            let last = limit_string(lines[lines.len() - 1], HALF_LIMIT, true);

            (first, Some(last))
        }

        fn complete_token(ctx: &TokCtx) -> Option<String> {
            if ctx.merged_start.is_some() && ctx.merged_end.is_none() {
                let start = ctx.merged_start.as_ref().unwrap();

                let expected_len = (ctx.end_pos - ctx.start_pos + 1) as usize;

                if start.chars().count() == expected_len {
                    return Some(start.clone());
                }
            }

            reconstructed_token(ctx)
        }

        fn get_hist(ctx: &TokCtx) -> (Option<String>, Option<String>) {
            if ctx.merged_hist.is_some() {
                return (
                    ctx.merged_pre_hist.clone(),
                    ctx.merged_hist.clone(),
                );
            }

            reconstructed_hist(ctx)
        }

        let (start, end) = if start.start_pos <= end.start_pos {
            (start, end)
        } else {
            (end, start)
        };

        let (merged_pre_hist, merged_hist) = get_hist(&start);

        let start_token = complete_token(&start);
        let end_token = complete_token(&end);

        let combined_token = match (start_token, end_token) {
            (Some(a), Some(b)) => {
                if start.end_pos >= end.start_pos {
                    let overlap_start = start.end_pos.saturating_sub(end.start_pos);
                    let overlap = overlap_start as usize;

                    let b_chars: Vec<char> = b.chars().collect();

                    let suffix = if overlap < b_chars.len() {
                        b_chars[overlap..].iter().collect::<String>()
                    } else {
                        String::new()
                    };

                    Some(format!("{a}{suffix}"))
                } else {
                    let gap = (end.start_pos - start.end_pos - 1) as usize;
                    let future_chars: Vec<char> = start.future.chars().collect();

                    if future_chars.len() >= gap {
                        let middle: String = future_chars[..gap].iter().collect();
                        Some(format!("{a}{middle}{b}"))
                    } else {
                        None
                    }
                }
            }
            (Some(a), None) => {
                if end.start_pos <= start.end_pos {
                    Some(a)
                } else {
                    None
                }
            }
            (None, Some(b)) => {
                if start.end_pos >= end.start_pos {
                    Some(b)
                } else {
                    None
                }
            }

            (None, None) => None,
        };

        if let Some(token) = combined_token {
            let (merged_start, merged_end) = make_token_parts(&token);

            return TokCtx {
                line: start.line,
                line_pos: start.line_pos,
                start_pos: start.start_pos,
                end_pos: end.end_pos,
                file: start.file,
                history: start.history,
                future: end.future,
                merged_pre_hist,
                merged_hist,
                merged_start: Some(merged_start),
                merged_end,
            };
        }

        let start_edge = if let Some(s) = start.merged_start.clone() {
            s
        } else if let Some(token) = reconstructed_token(&start) {
            make_token_parts(&token).0
        } else {
            String::new()
        };

        let end_edge = if let Some(e) = end.merged_end.clone() {
            Some(e)
        } else if let Some(token) = reconstructed_token(&end) {
            Some(make_token_parts(&token).0)
        } else {
            None
        };

        let (merged_start, merged_end) = if end_edge.is_none() {
            if start_edge.chars().count() <= FULL_LIMIT {
                (start_edge, None)
            } else {
                (
                    limit_string(&start_edge, HALF_LIMIT, false),
                    Some(limit_string(&start_edge, HALF_LIMIT, true)),
                )
            }
        } else {
            (
                limit_string(&start_edge, HALF_LIMIT, false),
                Some(limit_string(
                    &end_edge.unwrap(),
                    HALF_LIMIT,
                    true,
                )),
            )
        };

        TokCtx {
            line: start.line,
            line_pos: start.line_pos,
            start_pos: start.start_pos,
            end_pos: end.end_pos,
            file: start.file,
            history: start.history,
            future: end.future,
            merged_pre_hist,
            merged_hist: Some(merged_hist.unwrap_or_default()),
            merged_start: Some(merged_start),
            merged_end,
        }
    }

    pub fn get_parts(&self) -> (Option<String>, String, String, String) {
        if let Some(mut start) = self.merged_start.clone() {
            let Some(hist) = self.merged_hist.clone() else { panic!("oh no the chatgpt code failed :(") };

            let padding = create_pad(hist.len());

            if let Some(end) = self.merged_end.clone() {
                start = format!("{start} .. {end}");
            }

            (self.merged_pre_hist.clone(), hist, start, padding)
        } else {
            let hist = self.history.reconstruct();
            let hist2 = hist.clone();
            let mut hist_iter = hist2.lines().rev().map(|s| s.to_string());
            let hist_main = hist_iter
                .next()
                .unwrap_or(hist);

            let pre_hist = hist_iter.next();

            let tok_len = self.end_pos - self.start_pos + 1;
            let pad_len = hist_main.len() - (tok_len as usize).min(hist_main.len());
            let padding = create_pad(pad_len);

            let hist_part_normal = hist_main[0..pad_len].to_string();
            let hist_part_error = hist_main[pad_len..].to_string();

            (pre_hist, hist_part_normal, hist_part_error, padding)
        }
    }

    pub fn get_future_line(&self) -> String {
        self.future.split('\n').next().map(ToString::to_string).unwrap_or(self.future.clone())
    }
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
    Static,
    
    // END OF MODIFIERS
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
            Keyword::Static => "static",
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

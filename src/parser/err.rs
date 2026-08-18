use crate::token::tokenizer::TokenErr;
use crate::token::{Operator, Token, TokCtx, TokenType};
use mvutils::print::{Col, Fmt};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use crate::parser::mods::Modifier;

#[derive(Clone, Debug)]
pub struct ParseErr {
    pub ty: ParseErrType,
    pub ctx: TokCtx,
    pub tail: String,
    pub hint: Option<String>,
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = self.ty.get_message();
        let summary = self.ty.get_summary();

        fmt_fancy(
            f,
            &message,
            &summary,
            &self.ctx,
            &self.tail,
            self.hint.as_ref(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum ParseErrType {
    TokenizerErr(TokenErr),
    UnexpectedToken(Token, Vec<TokenExpectation>),
    NonBinaryCompatibleOperator(Operator),
    InternalErr,
    IllegalArrayDim,
    MissingType,
    NoMultipleNamesPerType,
    MismatchedLiteral(String, String),
    NonPositiveNumber,
    IllegalModifier(Modifier),
    MissingModifier,
    InvalidExpression,
}

impl ParseErrType {
    pub fn get_message(&self) -> String {
        match self {
            ParseErrType::TokenizerErr(_) => {
                "invalid syntax".to_string()
            }
            ParseErrType::UnexpectedToken(found, expected) => {
                format!("\texpected any of {expected:?}, but found {:?}", found.ty)
            }
            ParseErrType::NonBinaryCompatibleOperator(o) => {
                format!("operator {o:?} cannot be used in binary expressions!")
            }
            ParseErrType::InternalErr => {
                "internal error happened. This is usually unreachable. Please contact the vke team ASAP".to_owned()
            },
            ParseErrType::IllegalArrayDim => {
                "illegal literal used to specify static array length".to_string()
            },
            ParseErrType::MissingType => {
                "type annotation is required".to_string()
            },
            ParseErrType::NoMultipleNamesPerType => {
                "multiple names per type not allowed".to_string()
            },
            ParseErrType::MismatchedLiteral(s, e) => {
                format!("expected literal {s} but got {e}")
            },
            ParseErrType::NonPositiveNumber => {
                "a non positive integer is not allowed here!".to_string()
            },
            ParseErrType::IllegalModifier(md) => {
                format!("illegal modifier found `{md}`")
            }
            ParseErrType::MissingModifier => {
                "missing a required modifier".to_string()
            }
            ParseErrType::InvalidExpression => {
                "not a valid expression".to_string()
            }
        }
    }

    pub fn get_summary(&self) -> String {
        match self {
            ParseErrType::TokenizerErr(e) => {
                format!("{e:?}")
            }
            ParseErrType::UnexpectedToken(_, _) => "illegal token".to_string(),
            ParseErrType::NonBinaryCompatibleOperator(o) => {
                format!("invalid use of {o:?}")
            }
            ParseErrType::InternalErr => "unrecoverable".to_string(),
            ParseErrType::IllegalArrayDim => "illegal array length".to_string(),
            ParseErrType::MissingType => "missing type annotation".to_string(),
            ParseErrType::NoMultipleNamesPerType => { "too many names for type".to_string() },
            ParseErrType::MismatchedLiteral(s, e) => { format!("expected exact literal {s} but got {e}") },
            ParseErrType::NonPositiveNumber => "negative or fractional number not allowed".to_string(),
            ParseErrType::IllegalModifier(md) => format!("illegal modifier `{md}`"),
            ParseErrType::MissingModifier => "missing required modifier".to_string(),
            ParseErrType::InvalidExpression => "invalid expression".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenExpectation {
    Exact(TokenType),
    Ident,
    Lit
}

pub fn print_nice_error(error: ParseErr) {
    eprintln!("{error}");
}

fn fmt_fancy(
    f: &mut Formatter<'_>,
    message: &str,
    summary: &str,
    ctx: &TokCtx,
    tail: &str,
    hint: Option<&String>,
) -> fmt::Result {
    writeln!(f)?;
    let filename = ctx
        .file
        .as_ref()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "<not given>".to_string());
    let line = ctx.line;
    let pos = ctx.start_pos;

    let hist = ctx.history.reconstruct();
    let hist = hist
        .lines()
        .rev()
        .next()
        .map(|s| s.to_string())
        .unwrap_or(hist);

    let tok_len = ctx.end_pos - ctx.start_pos + 1;
    let pad_len = hist.len() - (tok_len as usize).min(hist.len());
    let padding = create_pad(pad_len);

    let hist_part_normal: &str = &hist[0..pad_len];
    let hist_part_error: &str = &hist[pad_len..];
    let squiggly = "~".repeat(hist_part_error.len());

    let mut printer = mvutils::print::Printer::start()
        .fmt(Fmt::Bold)
        .col(Col::Red)
        .text("error")
        .def()
        .fmt(Fmt::Bold)
        .col(Col::White)
        .text(": ")
        .text_ln(&message)
        .def()
        .fmt(Fmt::Bold)
        .col(Col::Cyan)
        .text("--> ")
        .fmt(Fmt::Bold)
        .col(Col::Blue)
        .text(&filename)
        .def()
        .text(" [")
        .col(Col::Yellow)
        .text(&format!("{line}:{pos}"))
        .def()
        .text_ln("]")
        .def()
        .fmt(Fmt::Bold)
        .col(Col::Cyan)
        .text_ln(" | ")
        .def()
        .fmt(Fmt::Bold)
        .col(Col::Cyan)
        .text(" | ")
        .def()
        .text(hist_part_normal)
        .bg(Col::Red)
        .text(hist_part_error)
        .def()
        .text_ln(tail)
        .def()
        .fmt(Fmt::Bold)
        .col(Col::Cyan)
        .text(" | ")
        .text(&padding)
        .def()
        .col(Col::Red)
        .text(&squiggly)
        .def()
        .text(" ")
        .col(Col::Grey)
        .text_ln(&summary)
        .def();

    if let Some(hint) = hint {
        let mut lines = hint.lines();
        let first = lines.next().unwrap_or("");
        printer = printer
            .fmt(Fmt::Bold)
            .col(Col::Cyan)
            .text_ln(" | ")
            .def()
            .fmt(Fmt::Bold)
            .col(Col::Cyan)
            .text(" = ")
            .col(Col::White)
            .text("hint: ")
            .def()
            .text_ln(first)
            .def();

        for remaining in lines {
            printer = printer
                .fmt(Fmt::Bold)
                .col(Col::Cyan)
                .text(" | ")
                .def()
                .text_ln(remaining)
                .def();
        }
    }

    let printed = printer.to_string();

    f.write_str(&printed)
}

fn create_pad(amt: usize) -> String {
    " ".repeat(amt)
}

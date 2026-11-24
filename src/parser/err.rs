use crate::token::tokenizer::TokenErr;
use crate::token::{Operator, Token, TokCtx, TokenType};
use mvutils::print::{Col, Fmt};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use itertools::Itertools;
use crate::parser::ast::ty::PrimitiveType;
use crate::parser::mods::Modifier;
use crate::scope::ast::ty::Type;
use crate::scope::builtins::BuiltinFn;

#[derive(Clone, Debug)]
pub struct ParseErr {
    pub ty: ParseErrType,
    pub ctx: TokCtx,
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
            self.hint.as_ref(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum ParseErrType {
    TokenizerErr(TokenErr),
    UnexpectedToken(Token, Vec<TokenExpectation>),
    UnconstrainedUnexpectedToken(TokenType),
    NonBinaryCompatibleOperator(Operator),
    NonUnaryCompatibleOperator(Operator),
    InternalErr,
    IllegalArrayDim(Type),
    MissingType,
    NoMultipleNamesPerType,
    MismatchedLiteral(String, String),
    NonPositiveNumber,
    IllegalModifier(Modifier),
    MissingModifier,
    InvalidExpression,
    AssignOpNotAllowedAtExpressionHead,
    IllegalTrailingSeparator,
    UnknownType(String),
    NotATypeSymbol(String),
    UnknownIdent(String),
    IllegalTupleAccessLiteral(PrimitiveType),
    DuplicateStruct(String),
    DuplicateVar(String),
    DuplicateFn(String),
    ConstantNotAConstant,
    ArraySizeRequired(bool),
    DefaultNotAllowed,
    CyclicalConstant,
    NotAConstant,
    TypeError(Type, Type),
    FunctionInConst,
    BlockInConst,
    ThisInConst,
    ModificationInConst,
    NotAllowed,
    ArrayTypeMismatch,
    CyclicalConstantInStruct,
    NoSuchField(String, String),
    MissingFields(String, Vec<String>),
    DuplicateField(String, String),
    CannotCastToType(Type),
    NotATuple,
    TupleIndexOutOfBounds(u32, u32),
    NotAnArray,
    ArrayIndexOutOfBounds(u32, u32),
    NegativeArraySize(i32),
    IllegalSwizzleComponent(char),
    SwizzleTooLong,
    AccessNotAllowed(Type),
    SwizzleComponentOutOfBounds(char, Type),
    NotConstFunction(BuiltinFn),
    IllegalParamCount(BuiltinFn),
    IllegalParams(BuiltinFn),
    IllegalUnaryType(Type, &'static str),
    CompilerVersionAlpha,
}

impl ParseErrType {
    pub fn get_message(&self) -> String {
        match self {
            ParseErrType::TokenizerErr(_) => {
                "invalid syntax".to_string()
            }
            ParseErrType::UnexpectedToken(found, expected) => {
                let available = expected.iter()
                    .map(|te| {
                        match te {
                            TokenExpectation::Exact(e) => format!("`{e}`"),
                            TokenExpectation::Ident => "identifier".to_string(),
                            TokenExpectation::Lit => "number".to_string(),
                        }
                    })
                    .join(", ");

                format!("\texpected any of {available}, but found `{}`", found.ty)
            }
            ParseErrType::UnconstrainedUnexpectedToken(found) => {
                format!("\tunexpected `{}`", found)
            }
            ParseErrType::NonBinaryCompatibleOperator(o) => {
                format!("operator {o:?} cannot be used in binary expressions!")
            }
            ParseErrType::NonUnaryCompatibleOperator(o) => {
                format!("operator {o:?} cannot be used in unary expressions!")
            }
            ParseErrType::InternalErr => {
                "internal error happened. This is usually unreachable. Please contact the vke team ASAP".to_owned()
            },
            ParseErrType::IllegalArrayDim(ty) => {
                format!("type `{}` cannot be used to define an array length", ty)
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
            ParseErrType::AssignOpNotAllowedAtExpressionHead => {
                "assignment operators cannot be used inside expressions".to_string()
            }
            ParseErrType::IllegalTrailingSeparator => {
                "a trailing separator is not allowed here!".to_string()
            }
            ParseErrType::UnknownType(st) => {
                format!("unknown type name {st}")
            }
            ParseErrType::NotATypeSymbol(st) => {
                format!("{st} is not a valid type symbol!")
            }
            ParseErrType::UnknownIdent(st) => {
                format!("unknown identifier {st}")
            }
            ParseErrType::IllegalTupleAccessLiteral(ty) => {
                format!("{ty} cannot be used to index literals")
            }
            ParseErrType::DuplicateStruct(ty) => {
                format!("the struct `{ty}` has been encountered at multiple times!")
            }
            ParseErrType::DuplicateVar(ty) => {
                format!("the variable `{ty}` has been encountered at multiple times!")
            }
            ParseErrType::DuplicateFn(f) => {
                format!("the function `{f}` has been encountered at multiple times!")
            }
            ParseErrType::ConstantNotAConstant => {
                "constant value is not constantly evaluatable".to_string()
            }
            ParseErrType::ArraySizeRequired(allow_default) => {
                if *allow_default {
                    "arrays within an SSBO or primitive uniform must either have a default size or definite size".to_string()
                } else {
                    "arrays within a UBO must have a definite size".to_string()
                }
            }
            ParseErrType::DefaultNotAllowed => {
                "arrays within a UBO do not support default size, and must instead have a definite size".to_string()
            }
            ParseErrType::CyclicalConstant => {
                "unable to evaluate constant whose definition is cyclical (points to itself)".to_string()
            }
            ParseErrType::NotAConstant => {
                "identifier references a value which isn't a constant inside a constant expression".to_string()
            }
            ParseErrType::TypeError(expected, found) => {
                format!("expected type `{expected}` but found `{found}`")
            }
            ParseErrType::FunctionInConst => {
                "function calls and constructors are not allowed in constant statements".to_string()
            }
            ParseErrType::BlockInConst => {
                "code blocks are currently not supported in constant statements".to_string()
            }
            ParseErrType::ThisInConst => {
                "`this` is not allowed in const statements".to_string()
            }
            ParseErrType::ModificationInConst => {
                "expressions that cause side effects are not allowed in constants".to_string()
            }
            ParseErrType::NotAllowed => {
                "the following code is not allowed for use".to_string()
            }
            ParseErrType::ArrayTypeMismatch => {
                "not all the values in the array are the same type".to_string()
            }
            ParseErrType::CyclicalConstantInStruct => {
                "struct type definition references constant which attempts to initialize the same struct".to_string()
            }
            ParseErrType::NoSuchField(s, f) => {
                format!("field `{f}` does not exist on struct `{s}`")
            }
            ParseErrType::MissingFields(name, fields) => {
                let fields = fields.iter().map(|s| format!("`{s}`")).join(", ");
                format!("missing field(s) {fields} for constructor of `{name}`")
            }
            ParseErrType::DuplicateField(s, f) => {
                format!("field `{f}` defined multiple times in constructor of `{s}`")
            }
            ParseErrType::CannotCastToType(t) => {
                format!("type `{t}` not supported for casting!")
            }
            ParseErrType::NotATuple => {
                "provided expression is not a tuple".to_string()
            }
            ParseErrType::TupleIndexOutOfBounds(i, l) => {
                format!("tuple index `{i}` out of bounds for tuple length {l}")
            }
            ParseErrType::NotAnArray => {
                "provided expression is not an array".to_string()
            }
            ParseErrType::ArrayIndexOutOfBounds(i, l) => {
                format!("array index `{i}` out of bounds for array length {l}")
            }
            ParseErrType::NegativeArraySize(i) => {
                format!("`{i}` is negative and thus cannot be used as an array size")
            }
            ParseErrType::IllegalSwizzleComponent(e) => {
                format!("`{e}` is not a valid swizzle component")
            }
            ParseErrType::SwizzleTooLong => {
                "swizzle is too long, swizzles can only be up to 4 components".to_string()
            }
            ParseErrType::AccessNotAllowed(t) => {
                format!("cannot access fields on type `{t}` as it is not a struct, vec or matrix")
            }
            ParseErrType::SwizzleComponentOutOfBounds(e, t) => {
                format!("swizzle component `{e}` is out of bounds for type `{t}`")
            }
            ParseErrType::NotConstFunction(f) => {
                format!("function `{f}` is not callable from a constant expression")
            }
            ParseErrType::IllegalParamCount(f) => {
                format!("illegal parameter count for constant function `{f}`")
            }
            ParseErrType::IllegalParams(f) => {
                format!("illegal parameters provided for constant function `{f}`")
            }
            ParseErrType::IllegalUnaryType(ty, unary) => {
                format!("{unary} operator cannot be applied to type {ty}")
            }
            ParseErrType::CompilerVersionAlpha => {
                "sad as it may be, the compiler is still in alpha and we do not support everything we should yet".to_string()
            }
        }
    }

    pub fn get_summary(&self) -> String {
        match self {
            ParseErrType::TokenizerErr(e) => {
                format!("{e:?}")
            }
            ParseErrType::UnexpectedToken(_, _) => "illegal token".to_string(),
            ParseErrType::UnconstrainedUnexpectedToken(_) => "illegal token".to_string(),
            ParseErrType::NonBinaryCompatibleOperator(o) => { format!("invalid use of {o}") }
            ParseErrType::NonUnaryCompatibleOperator(o) => { format!("invalid use of {o}") }
            ParseErrType::InternalErr => "unrecoverable".to_string(),
            ParseErrType::IllegalArrayDim(_) => "illegal array length".to_string(),
            ParseErrType::MissingType => "missing type annotation".to_string(),
            ParseErrType::NoMultipleNamesPerType => { "too many names for type".to_string() },
            ParseErrType::MismatchedLiteral(s, e) => { format!("expected exact literal {s} but got {e}") },
            ParseErrType::NonPositiveNumber => "negative or fractional number not allowed".to_string(),
            ParseErrType::IllegalModifier(md) => format!("illegal modifier `{md}`"),
            ParseErrType::MissingModifier => "missing required modifier".to_string(),
            ParseErrType::InvalidExpression => "invalid expression".to_string(),
            ParseErrType::AssignOpNotAllowedAtExpressionHead => "illegal assignment operator at start of expression".to_string(),
            ParseErrType::IllegalTrailingSeparator => "disallowed trailing separator".to_string(),
            ParseErrType::UnknownType(st) => format!("unknown type {st}"),
            ParseErrType::NotATypeSymbol(st) => format!("invalid type symbol {st}"),
            ParseErrType::UnknownIdent(st) => format!("unknown identifier {st}"),
            ParseErrType::IllegalTupleAccessLiteral(_) => "illegal tuple access".to_string(),
            ParseErrType::DuplicateStruct(_) => "duplicate struct".to_string(),
            ParseErrType::DuplicateVar(_) => "duplicate var".to_string(),
            ParseErrType::DuplicateFn(_) => "duplicate function".to_string(),
            ParseErrType::ConstantNotAConstant => "inevaluatable constant".to_string(),
            ParseErrType::ArraySizeRequired(_) => "array without size definition".to_string(),
            ParseErrType::DefaultNotAllowed => "illegal use of default".to_string(),
            ParseErrType::CyclicalConstant => "constant used in definition of itself".to_string(),
            ParseErrType::NotAConstant => "identifier pointing to non-constant".to_string(),
            ParseErrType::TypeError(_, _) => "invalid type".to_string(),
            ParseErrType::FunctionInConst => "function call in constant expression".to_string(),
            ParseErrType::BlockInConst => "code block in constant expression".to_string(),
            ParseErrType::ThisInConst => "illegal `this` token".to_string(),
            ParseErrType::ModificationInConst => "side effect causing expression".to_string(),
            ParseErrType::NotAllowed => "what made you think this would work?".to_string(),
            ParseErrType::ArrayTypeMismatch => "mismatched array type".to_string(),
            ParseErrType::CyclicalConstantInStruct => "cyclical struct reference type definition idk how you achieved receiving this error".to_string(),
            ParseErrType::NoSuchField(_, _) => "reference of non existent field".to_string(),
            ParseErrType::MissingFields(_, _) => "missing fields not present".to_string(),
            ParseErrType::DuplicateField(_, _) => "duplicate field definition".to_string(),
            ParseErrType::CannotCastToType(_) => "invalid use of type for cast".to_string(),
            ParseErrType::NotATuple => "non tuple expression".to_string(),
            ParseErrType::TupleIndexOutOfBounds(_, _) => "out of bound tuple index".to_string(),
            ParseErrType::NotAnArray => "non array expression".to_string(),
            ParseErrType::ArrayIndexOutOfBounds(_, _) => "out of bound array index".to_string(),
            ParseErrType::NegativeArraySize(_) => "negative array size".to_string(),
            ParseErrType::IllegalSwizzleComponent(e) => format!("swizzle with `{e}` component"),
            ParseErrType::SwizzleTooLong => "illegaly long swizzle".to_string(),
            ParseErrType::AccessNotAllowed(_) => "illegal field access".to_string(),
            ParseErrType::SwizzleComponentOutOfBounds(e, _) => format!("swizzle with `{e}` component"),
            ParseErrType::NotConstFunction(_) => "illegal function call".to_string(),
            ParseErrType::IllegalParamCount(_) => "illegal function call".to_string(),
            ParseErrType::IllegalParams(_) => "illegal function call".to_string(),
            ParseErrType::IllegalUnaryType(_, _) => "illegal type into unary operator".to_string(),
            ParseErrType::CompilerVersionAlpha => "be excited for the update!".to_string(),
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
    hint: Option<&String>,
) -> fmt::Result {
    writeln!(f)?;
    let filename = ctx
        .file
        .as_ref()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "<not given>".to_string());
    let line = ctx.line;
    let pos = ctx.line_pos;

    let (pre_hist, hist_part_normal, hist_part_error, padding) = ctx.get_parts();

    let future = ctx.get_future_line();
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
        .def();

    if let Some(pre_history) = pre_hist {
        printer = printer.fmt(Fmt::Bold)
            .col(Col::Cyan)
            .text(" | ")
            .def()
            .text_ln(&pre_history)
            .def();
    }

    printer = printer
        .fmt(Fmt::Bold)
        .col(Col::Cyan)
        .text(" | ")
        .def()
        .text(&hist_part_normal)
        .bg(Col::Red)
        .text(&hist_part_error)
        .def()
        .text_ln(&future)
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

pub(crate) fn create_pad(amt: usize) -> String {
    " ".repeat(amt)
}

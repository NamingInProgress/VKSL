use crate::token;
use crate::token::Operator::*;
use crate::token::Token;
use crate::token::TokenType::*;
use itertools::PeekNth;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;
use std::vec::IntoIter;

pub type Result = core::result::Result<Token, TokenErr>;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenErr {
    file: Option<PathBuf>,
    line: u32,
    pos: u32,
    error: TokenErrType,
}

#[derive(Clone, PartialEq)]
pub enum TokenErrType {
    FloatError(ParseFloatError),
    IntError(ParseIntError),
}

impl Debug for TokenErrType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenErrType::FloatError(e) => Display::fmt(&e, f),
            TokenErrType::IntError(e) => Display::fmt(&e, f)
        }
    }
}

pub struct Tokenizer<I: Iterator<Item = char>> {
    src: PeekNth<I>,
    file: Option<PathBuf>,
    line: u32,
    pos: u32,
    buffer: VecDeque<Token>,
}

impl Tokenizer<IntoIter<char>> {
    pub fn from_str(source: String) -> Self {
        let source = source.chars().collect::<Vec<_>>().into_iter();
        Self::new(source, None)
    }
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    pub fn new(source: I, file: Option<PathBuf>) -> Self {
        Self {
            src: itertools::peek_nth(source),
            file,
            line: 1,
            pos: 0,
            buffer: VecDeque::new(),
        }
    }

    pub fn next_token(&mut self) -> Option<Result> {
        if !self.buffer.is_empty() {
            return Some(Ok(self.buffer.pop_front().unwrap()));
        }

        loop {
            let mut found_any = self.skip_whitespace();
            found_any |= self.skip_comments();
            if !found_any {
                break;
            }
        }

        let c = self.inc()?;
        let start_pos = self.pos;

        loop {
            let token_type = match c {
                '(' => LParen,
                ')' => RParen,
                '{' => LBrace,
                '}' => RBrace,
                '[' => LBracket,
                ']' => RBracket,
                ',' => Comma,
                ';' => Semi,
                ':' => Colon,
                '?' => Question,
                '.' => Operator(Dot),

                '+' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Plus)
                    } else if Some('+') == self.peek() {
                        self.inc();
                        Operator(PlusPlus)
                    } else {
                        Operator(Plus)
                    }
                }

                '-' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Minus)
                    } else if Some('-') == self.peek() {
                        self.inc();
                        Operator(MinusMinus)
                    } else if Some('>') == self.peek() {
                        self.inc();
                        RArrow
                    } else {
                        Operator(Minus)
                    }
                }

                '*' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Mul)
                    } else {
                        Operator(Mul)
                    }
                }

                '/' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Div)
                    } else {
                        Operator(Div)
                    }
                }

                '%' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Modulo)
                    } else {
                        Operator(Modulo)
                    }
                }

                '<' => {
                    // < <= <- << <<=
                    if Some('-') == self.peek() {
                        self.inc();
                        LArrow
                    } else if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Less)
                    } else if Some('<') == self.peek() {
                        self.inc();
                        if Some('=') == self.peek() {
                            self.inc();
                            OperatorAssign(Lsh)
                        } else {
                            Operator(Lsh)
                        }
                    } else {
                        Operator(Less)
                    }
                }

                '>' => {
                    // > >= >> >>= >>> >>>=
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Greater)
                    } else if Some('>') == self.peek() {
                        self.inc();
                        if Some('=') == self.peek() {
                            self.inc();
                            OperatorAssign(Rsh)
                        } else if Some('>') == self.peek() {
                            self.inc();
                            if Some('=') == self.peek() {
                                self.inc();
                                OperatorAssign(LogicalRsh)
                            } else {
                                Operator(LogicalRsh)
                            }
                        } else {
                            Operator(Rsh)
                        }
                    } else {
                        Operator(Greater)
                    }
                }

                '!' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Not)
                    } else {
                        Operator(Not)
                    }
                }

                '=' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(Eq)
                    } else {
                        Operator(Eq)
                    }
                }

                '|' => {
                    if Some('|') == self.peek() {
                        self.inc();
                        Operator(Or)
                    } else if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(BitOr)
                    } else {
                        Operator(BitOr)
                    }
                }

                '&' => {
                    if Some('&') == self.peek() {
                        self.inc();
                        Operator(And)
                    } else if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(BitAnd)
                    } else {
                        Operator(BitAnd)
                    }
                }

                '^' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(BitXor)
                    } else {
                        Operator(BitXor)
                    }
                }

                '~' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        OperatorAssign(BitNegate)
                    } else {
                        Operator(BitNegate)
                    }
                }

                _ => {
                    let mut buf = String::new();
                    buf.push(c);
                    let is_num = c.is_numeric();
                    let mut is_float = false;
                    let mut is_hex = false;
                    let mut is_bin = false;
                    while let Some(c) = self.peek() && Self::is_lit_part(c, is_num, is_hex) {
                        if is_num && c == 'x' {
                            is_hex = true;
                        }
                        if is_num && c == 'b' {
                            is_bin = true;
                        }
                        buf.push(self.inc()?);
                    }
                    while let Some(c) = self.peek() && c == '.' && is_num {
                        buf.push(self.inc()?);
                        is_float = true;
                    }
                    while let Some(c) = self.peek() && Self::is_lit_part(c, is_num, is_hex) {
                        buf.push(self.inc()?);
                    }

                    let is_uint = Some('u') == self.peek().map(|c| c.to_ascii_lowercase());
                    if is_uint {
                        self.inc();
                    }
                    let is_f64 = self.peequal(['f', '6', '4']);
                    is_float |= self.peequal(['f', '3', '2']);

                    if is_num {
                        if is_float || is_f64 {
                            if is_f64 {
                                match buf.parse::<f64>() {
                                    Ok(f) => Literal(token::Literal::DoubleLit(f)),
                                    Err(e) => {
                                        return self.create_error(TokenErrType::FloatError(e));
                                    }
                                }
                            } else {
                                match buf.parse::<f32>() {
                                    Ok(f) => Literal(token::Literal::FloatLit(f)),
                                    Err(e) => {
                                        return self.create_error(TokenErrType::FloatError(e));
                                    }
                                }
                            }
                        } else {
                            if is_uint {
                                let res = if is_hex {
                                    u32::from_str_radix(&buf[2..], 16)
                                } else if is_bin {
                                    u32::from_str_radix(&buf[2..], 2)
                                } else {
                                    buf.parse::<u32>()
                                };
                                match res {
                                    Ok(f) => Literal(token::Literal::UIntLit(f)),
                                    Err(e) => return self.create_error(TokenErrType::IntError(e)),
                                }
                            } else {
                                let res = if is_hex {
                                    i32::from_str_radix(&buf[2..], 16)
                                } else if is_bin {
                                    i32::from_str_radix(&buf[2..], 2)
                                } else {
                                    buf.parse::<i32>()
                                };
                                match res {
                                    Ok(f) => Literal(token::Literal::IntLit(f)),
                                    Err(e) => return self.create_error(TokenErrType::IntError(e)),
                                }
                            }
                        }
                    } else {
                        if let Ok(kw) = buf.parse::<token::Keyword>() {
                            Keyword(kw)
                        } else {
                            Ident(buf)
                        }
                    }
                }
            };

            return Some(Ok(Token {
                ty: token_type,
                line: self.line,
                start_pos,
                end_pos: self.pos,
                file: self.file.clone(),
            }));
        }
    }

    fn create_error(&self, inner: TokenErrType) -> Option<Result> {
        Some(Err(TokenErr {
            file: self.file.clone(),
            line: self.line,
            pos: self.pos,
            error: inner,
        }))
    }

    fn is_lit_part(n: char, is_num: bool, is_hex: bool) -> bool {
        if is_num {
            matches!(n, 'e' | '-' | 'b' | 'x') || if is_hex { n.is_alphanumeric() } else { n.is_numeric() }
        } else {
            matches!(n, '_' | '$' | 'e') || n.is_alphanumeric()
        }
    }

    pub fn putback(&mut self, token: Token) {
        self.buffer.push_back(token);
    }

    fn inc(&mut self) -> Option<char> {
        self.pos += 1;
        let n = self.src.next()?;
        if n == '\n' {
            self.line += 1;
            self.pos = 0;
        }
        Some(n)
    }

    fn peek(&mut self) -> Option<char> {
        self.peek_n(0)
    }

    fn peek_n(&mut self, num: usize) -> Option<char> {
        self.src.peek_nth(num).copied()
    }

    fn peequal<const N: usize>(&mut self, arr: [char; N]) -> bool {
        for i in 0..N {
            if self.peek_n(i) != Some(arr[i]) {
                return false;
            }
        }
        for _ in 0..N {
            self.inc();
        }
        true
    }

    fn skip_whitespace(&mut self) -> bool {
        let mut f = false;
        while let Some(c) = self.src.peek() && c.is_whitespace() {
            self.inc();
            f = true;
        }
        f
    }

    fn skip_comments(&mut self) -> bool {
        let mut f = false;
        if let Some(c) = self.src.peek_nth(0).copied() {
            if let Some(n) = self.src.peek_nth(1).copied() {
                match (c, n) {
                    ('/', '/') => {
                        self.inc();
                        self.inc();
                        f = true;
                        while let Some(nl) = self.peek() {
                            self.inc();
                            if nl == '\n' {
                                break;
                            }
                        }
                    }
                    ('/', '*') => {
                        self.inc();
                        self.inc();
                        f = true;
                        while let Some(c) = self.src.peek_nth(0).copied() {
                            if c == '*' && self.src.peek_nth(1).copied() == Some('/') {
                                self.inc();
                                self.inc();
                                break;
                            }
                            self.inc();
                        }
                    }
                    _ => {}
                }
            }
        }
        f
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

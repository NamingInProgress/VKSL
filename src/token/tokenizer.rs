use crate::token;
use crate::token::Operator::*;
use crate::token::{Token, TokenContext};
use crate::token::TokenType::*;
use itertools::PeekNth;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;
use std::vec::IntoIter;

pub type Result = core::result::Result<Token, TokenErr>;

pub const HIST_CAP: usize = 60;
pub const TAIL_CAP: usize = 10;

#[derive(Clone, PartialEq)]
pub enum TokenErr {
    FloatError(ParseFloatError),
    IntError(ParseIntError),
    NegativeUInt(u32),
    EOF
}

impl Debug for TokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenErr::FloatError(e) => Display::fmt(&e, f),
            TokenErr::IntError(e) => Display::fmt(&e, f),
            TokenErr::EOF => f.write_str("end of file"),
            TokenErr::NegativeUInt(uint) => {
                write!(f, "-{uint}u is not a valid unsigned number declaration")
            }
        }
    }
}

pub struct Tokenizer<I: Iterator<Item = char>> {
    src: PeekNth<I>,
    pub file: Option<PathBuf>,
    pub line: u32,
    pub pos: u32,
    buffer: VecDeque<Token>,
    pub history: History,
    pub start_pos: u32
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
            history: History::new(HIST_CAP),
            start_pos: 0,
        }
    }

    pub fn peek_token(&mut self) -> Option<Result> {
        self.peek_token_n(0)
    }

    pub fn peek_token_n(&mut self, n: usize) -> Option<Result> {
        let missing = n as isize - self.buffer.len() as isize + 1;
        if missing <= 0 {
            self.buffer.get(n).cloned().map(Ok)
        } else {
            for _ in 0..missing {
                let t = self.next_token()?;
                match t {
                    Ok(t) => {
                        self.buffer.push_back(t);
                    }
                    Err(e) => return Some(Err(e))
                }
            }
            self.peek_token_n(n)
        }
    }

    pub fn next_token(&mut self) -> Option<Result> {
        if !self.buffer.is_empty() {
            let mut t = self.buffer.pop_front().unwrap();
            t.ctx.history = self.history.clone();
            return Some(Ok(t));
        }

        loop {
            let mut found_any = self.skip_whitespace();
            found_any |= self.skip_comments();
            if !found_any {
                break;
            }
        }

        let mut c = self.inc()?;
        let start_pos = self.pos;
        self.start_pos = start_pos;

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
                        Operator(Merge)
                    } else if Some('=') == self.peek() {
                        self.inc();
                        Operator(LessEq)
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
                        Operator(GreaterEq)
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
                        Operator(Neq)
                    } else {
                        Operator(Not)
                    }
                }

                '=' => {
                    if Some('=') == self.peek() {
                        self.inc();
                        Operator(EqEq)
                    } else {
                        Operator(Assign)
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
                    let mut is_neg = false;
                    if c == '-' {
                        let raw = if Some('=') == self.peek() {
                            self.inc();
                            OperatorAssign(Minus)
                        } else if Some('-') == self.peek() {
                            self.inc();
                            Operator(MinusMinus)
                        } else if Some('>') == self.peek() {
                            self.inc();
                            Operator(Merge)
                        } else if !self.peek().is_some_and(|c| c.is_numeric()) {
                            Operator(Minus)
                        } else {
                            is_neg = true;
                            Operator(Minus)
                        };
                        if !is_neg {
                            return Some(Ok(Token {
                                ty: raw,
                                ctx: TokenContext {
                                    line: self.line,
                                    start_pos,
                                    end_pos: self.pos,
                                    file: self.file.clone(),
                                    history: self.history.clone()
                                }
                            }));
                        }
                        c = self.inc()?;
                    }

                    let sign = if is_neg { -1.0 } else { 1.0 };

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
                                    Ok(f) => Literal(token::Literal::DoubleLit(f * sign)),
                                    Err(e) => {
                                        return self.create_error(TokenErr::FloatError(e));
                                    }
                                }
                            } else {
                                match buf.parse::<f32>() {
                                    Ok(f) => Literal(token::Literal::FloatLit(f * sign as f32)),
                                    Err(e) => {
                                        return self.create_error(TokenErr::FloatError(e));
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
                                    Ok(f) if !is_neg => Literal(token::Literal::UIntLit(f)),
                                    Err(e) => return self.create_error(TokenErr::IntError(e)),
                                    Ok(f) => return self.create_error(TokenErr::NegativeUInt(f))
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
                                    Ok(f) => Literal(token::Literal::IntLit(f * sign as i32)),
                                    Err(e) => return self.create_error(TokenErr::IntError(e)),
                                }
                            }
                        }
                    } else {
                        if let Ok(kw) = buf.parse::<token::Keyword>() {
                            Keyword(kw)
                        } else if "false" == buf {
                            Literal(token::Literal::BoolLit(false))
                        } else if "true" == buf {
                            Literal(token::Literal::BoolLit(true))
                        } else {
                            Ident(buf)
                        }
                    }
                }
            };

            return Some(Ok(Token {
                ty: token_type,
                ctx: TokenContext {
                    line: self.line,
                    start_pos,
                    end_pos: self.pos,
                    file: self.file.clone(),
                    history: self.history.clone()
                }
            }));
        }
    }

    fn create_error(&self, inner: TokenErr) -> Option<Result> {
        Some(Err(inner))
    }

    pub fn create_context(&self) -> TokenContext {
        TokenContext {
            line: self.line,
            start_pos: self.start_pos,
            end_pos: self.pos,
            file: self.file.clone(),
            history: self.history.clone(),
        }
    }

    fn is_lit_part(n: char, is_num: bool, is_hex: bool) -> bool {
        if is_num {
            matches!(n, 'e' | '-' | 'b' | 'x') || if is_hex { n.is_alphanumeric() } else { n.is_numeric() }
        } else {
            matches!(n, '_' | '$' | 'e') || n.is_alphanumeric()
        }
    }

    fn inc(&mut self) -> Option<char> {
        self.pos += 1;
        let n = self.src.next()?;
        if n == '\n' {
            self.line += 1;
            self.pos = 0;
        }
        self.history.push(n);
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

    pub fn expect_any(&mut self) -> Result {
        if let Some(n) = self.next_token() {
            n
        } else {
            self.create_error(TokenErr::EOF).expect("will be some look at method bruh")
        }
    }

    pub fn expect_any_peeked(&mut self) -> Result {
        if let Some(n) = self.peek_token() {
            n
        } else {
            self.create_error(TokenErr::EOF).expect("will be some look at method bruh")
        }
    }

    pub fn next_raw_chars(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        for _ in 0..n {
            if let Some(c) = self.src.next() {
                s.push(c);
            } else {
                break;
            }
        }
        s
    }

    pub fn tail(&mut self, n: usize) -> String {
        self.next_raw_chars(n)
            .chars()
            .take_while(|c| *c != '\n')
            .collect::<String>()
    }

    pub(crate) fn tail_default(&mut self) -> String {
        self.tail(TAIL_CAP)
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct History {
    contents: Vec<char>,
    seam: usize,
    pub(crate) len: usize
}

impl History {
    pub fn new(cap: usize) -> Self {
        Self {
            contents: vec!['\0'; cap],
            seam: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, c: char) {
        let cap = self.contents.len();
        let new_index = (self.seam + 1) % cap;
        self.len += 1;
        if self.len > cap {
            self.len = cap;
        }
        self.contents[new_index] = c;
        self.seam = new_index;
    }

    pub fn reconstruct(&self) -> String {
        let cap = self.contents.len();
        let mut s = String::with_capacity(cap);
        if self.len < cap {
            for i in 0..self.len {
                s.push(self.contents[i + 1]);
            }
        } else {
            let mut idx = (self.seam + 1) % cap;
            let mut count = 0;
            while count < cap {
                s.push(self.contents[idx]);
                idx = (idx + 1) % cap;
                count += 1;
            }
        }
        s
    }
}
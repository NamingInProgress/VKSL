use crate::ast::stmt::Stmt;
use crate::ast::ty::Type;
use err::TokenExpectation::*;
use crate::parser::err::{ParseErr, ParseErrType, TokenExpectation};
use crate::token;
use crate::token::tokenizer::Tokenizer;
use crate::token::{Token, TokenType};

pub mod expr;
pub mod stmt;
pub mod err;
pub mod types;

pub type Result<T> = core::result::Result<T, ParseErr>;
pub type TokenRes = token::tokenizer::Result;

pub struct Parser<I: Iterator<Item=char>> {
    tokens: Tokenizer<I>,
    defs: Vec<Stmt>
}

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn new(tokens: Tokenizer<I>) -> Self {
        Self {
            tokens,
            defs: vec!()
        }
    }

    pub fn parse(mut self) -> Result<Vec<Stmt>> {
        while let Some(_) = self.peek_token()? {
            let s = self.parse_stmt()?;
            self.defs.push(s);
        }

        Ok(self.defs)
    }

    fn expect_next(&mut self, expected: Vec<TokenExpectation>) -> Result<Token> {
        let next = self.unwrap_next()?;

        let matches = expected.iter().any(|expectation| {
            match expectation {
                Exact(ty) => *ty == next.ty,
                Ident => {
                    matches!(next.ty, TokenType::Ident(_))
                }
            }
        });

        if !matches {
            let ctx = next.ctx.clone();
            let ty = ParseErrType::UnexpectedToken(next, expected);

            let tail = self.tokens.tail_default();

            return Err(ParseErr {
                ty,
                ctx,
                tail,
                hint: Some("Consider using one of the listed tokens instead.".to_string()),
            });
        }

        Ok(next)
    }

    fn expect_ident(&mut self) -> Result<String> {
        let token = self.expect_next(vec!(Ident))?;

        match token.ty {
            TokenType::Ident(name) => Ok(name),
            _ => unreachable!(),
        }
    }

    fn expect_semi(&mut self) -> Result<Token> {
        let token = self.expect_next(vec!(Exact(TokenType::Semi)))?;

        match token.ty {
            TokenType::Semi => Ok(token),
            _ => unreachable!(),
        }
    }

    fn skip(&mut self) {
        self.tokens.next();
    }

    fn unwrap_next(&mut self) -> Result<Token> {
        match self.tokens.expect_any() {
            Ok(t) => Ok(t),
            Err(e) => {
                let err = self.tokenizer_err(ParseErrType::TokenizerErr(e));
                Err(err)
            }
        }
    }

    pub fn peek_token(&mut self) -> Result<Option<Token>> {
        match self.tokens.peek_token() {
            None => Ok(None),
            Some(res) => {
                match res {
                    Ok(t) => Ok(Some(t)),
                    Err(e) => {
                        let err = self.tokenizer_err(ParseErrType::TokenizerErr(e));
                        Err(err)
                    }
                }
            }
        }
    }

    fn unwrap_peek(&mut self) -> Result<Token> {
        match self.tokens.expect_any_peeked() {
            Ok(t) => Ok(t),
            Err(e) => {
                let err = self.tokenizer_err(ParseErrType::TokenizerErr(e));
                Err(err)
            }
        }
    }

    pub fn token_err(&mut self, inner: ParseErrType, tok: Token) -> ParseErr {
        let tail = self.tokens.tail_default();

        ParseErr {
            ty: inner,
            ctx: tok.ctx,
            tail,
            hint: None,
        }
    }

    pub fn token_err_with_hint(&mut self, inner: ParseErrType, tok: Token, hint: String) -> ParseErr {
        let tail = self.tokens.tail_default();

        ParseErr {
            ty: inner,
            ctx: tok.ctx,
            tail,
            hint: Some(hint),
        }
    }

    pub fn tokenizer_err(&mut self, inner: ParseErrType) -> ParseErr {
        let tail = self.tokens.tail_default();

        ParseErr {
            ty: inner,
            ctx: self.tokens.create_context(),
            tail,
            hint: None,
        }
    }
}
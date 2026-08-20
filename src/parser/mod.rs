use ast::stmt::Stmt;
use crate::parser::err::{ParseErr, ParseErrType, TokenExpectation};
use crate::token::tokenizer::Tokenizer;
use crate::token::{Literal, TokCtx, Token, TokenType};
use crate::token;
use err::TokenExpectation::*;
use ast::Ast;

pub mod err;
pub mod expr;
pub mod stmt;
pub mod types;
pub mod mods;
pub mod punct;
pub mod ast;

pub type Result<T> = core::result::Result<T, ParseErr>;
pub type TokenRes = token::tokenizer::Result;

pub struct Parser<I: Iterator<Item = char>> {
    tokens: Tokenizer<I>,
    defs: Vec<Stmt>,
}

impl<I: Iterator<Item = char>> Parser<I> {
    pub fn new(tokens: Tokenizer<I>) -> Self {
        Self {
            tokens,
            defs: vec![],
        }
    }

    pub fn parse(mut self) -> Result<Ast> {
        while let Some(_) = self.peek_token()? {
            let s = self.parse_stmt()?;
            self.defs.push(s);
        }

        Ok(self.defs)
    }

    fn expect_next(&mut self, expected: Vec<TokenExpectation>) -> Result<Token> {
        let next = self.unwrap_next()?;

        let matches = expected.iter().any(|expectation| match expectation {
            Exact(ty) => *ty == next.ty,
            Ident => {
                matches!(next.ty, TokenType::Ident(_))
            }
            Lit => {
                matches!(next.ty, TokenType::Literal(_))
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

    fn expect_non_negative_int(&mut self) -> Result<(u32, TokCtx)> {
        let tok = self.expect_next(vec![Lit])?;
        if let TokenType::Literal(lit) = tok.ty {
            match lit {
                Literal::IntLit(a) => {
                    if a >= 0 {
                        return Ok((a as u32, tok.ctx));
                    }
                }
                Literal::UIntLit(a) => return Ok((a, tok.ctx)),
                _ => {},
            }
        }

        Err(self.token_err(ParseErrType::NonPositiveNumber, tok))
    }

    fn expect_ident_exact(&mut self, expected: &str) -> Result<ast::Ident> {
        let token = self.expect_next(vec![Ident])?;

        let TokenType::Ident(name) = &token.ty else {
            unreachable!();
        };

        if name == expected {
            return Ok(ast::Ident { val: name.clone(), tkn: token.ctx });
        }

        Err(self.token_err_with_hint(
            ParseErrType::MismatchedLiteral(expected.to_string(), name.clone()),
            token.ctx,
            format!("Consider replacing {name} with {expected}"),
        ))
    }

    fn expect_ident(&mut self) -> Result<ast::Ident> {
        let token = self.expect_next(vec![Ident])?;

        match token.ty {
            TokenType::Ident(name) => Ok(ast::Ident { val: name, tkn: token.ctx }),
            _ => unreachable!(),
        }
    }

    fn expect_semi(&mut self) -> Result<Token> {
        let token = self.expect_next(vec![Exact(TokenType::Semi)])?;
        Ok(token)
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
            Some(res) => match res {
                Ok(t) => Ok(Some(t)),
                Err(e) => {
                    let err = self.tokenizer_err(ParseErrType::TokenizerErr(e));
                    Err(err)
                }
            },
        }
    }

    fn check_next(&mut self, looking_for: Vec<TokenExpectation>) -> Result<bool> {
        if let Some(tt) = self.peek_token()? {
            return Ok(looking_for.iter().any(|expectation| match expectation {
                Exact(ty) => *ty == tt.ty,
                Ident => {
                    matches!(tt.ty, TokenType::Ident(_))
                }
                Lit => {
                    matches!(tt.ty, TokenType::Literal(_))
                }
            }))
        }

        Ok(false)
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

    pub fn token_err_with_hint(
        &mut self,
        inner: ParseErrType,
        tok: TokCtx,
        hint: String,
    ) -> ParseErr {
        let tail = self.tokens.tail_default();

        ParseErr {
            ty: inner,
            ctx: tok,
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

use itertools::PeekNth;
use crate::ast::expr::Expr;
use crate::{parser, token};
use crate::ast::input_interpolation::InputInterpolation;
use crate::ast::stmt::{BlockStmt, Stmt};
use crate::ast::tree_maker;
use crate::ast::ty::Type;
use crate::parser::{ParseErr, TokenExpectation};
use crate::parser::TokenExpectation::{AnyIdentifier, AnyType, Exact};
use crate::token::{Token, TokenType};
use crate::token::tokenizer::{TokenErr, Tokenizer};

pub struct Parser<I: Iterator<Item=char>> {
    tokens: PeekNth<Tokenizer<I>>,
    defs: Vec<Stmt>
}

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn new(tokens: Tokenizer<I>) -> Self {
        Self {
            tokens: itertools::peek_nth(tokens),
            defs: vec!()
        }
    }

    pub fn parse(mut self) -> parser::Result<Vec<Stmt>> {
        while let Some(token) = self.tokens.next() {
            let token = Self::unwrap_token(token)?;

            match &token.ty {
                TokenType::Keyword(kw) => {
                    let opt = self.parse_keyword(token.clone(), kw.clone())?;
                    self.defs.push(opt);
                }
                _ => {}
            }
        }

        Ok(self.defs)
    }

    pub fn parse_keyword(&mut self, token: Token, keyword: token::Keyword) -> parser::Result<Stmt> {
        match keyword {
            token::Keyword::Input => self.parse_shader_input(false),
            token::Keyword::Provide => self.parse_shader_input(true),
            token::Keyword::Output => self.parse_shader_output(),
            token::Keyword::Fn => self.parse_method_decl(),
            _ => { Ok(tree_maker::ConstT("asd".to_string(), Type::from_str("asd".to_string()))) }
        }
    }


    pub fn parse_expr(&mut self) -> parser::Result<Expr> {
        todo!()
    }

    pub fn parse_method_decl(&mut self) -> parser::Result<Stmt> {
        let name = self.expect_ident()?;
        let mut params = vec!();

        self.expect_next(vec!(Exact(TokenType::LParen)))?;

        let mut next = self.unwrap_next()?;
        while next.ty != TokenType::RParen {
            let param_name = self.expect_ident()?;
            self.expect_next(vec!(Exact(TokenType::Colon)))?;
            let param_type = self.expect_type()?;

            params.push(tree_maker::MethodParam(param_name, param_type));
            next = self.unwrap_next()?;
        }

        let mut ret_type = None;

        if let TokenType::RArrow = self.unwrap_peek()?.ty {
            self.skip();
            ret_type = Some(self.expect_type()?);
        }

        if let Stmt::Block(stmt) = self.parse_block()? {
            Ok(tree_maker::MethodDef(name, params, ret_type, stmt))
        } else {
            Err(ParseErr::InternalErr)
        }
    }

    pub fn parse_block(&mut self) -> parser::Result<Stmt> {
        self.expect_next(vec!(Exact(TokenType::LBrace)))?;

        // TODO:

        self.expect_next(vec!(Exact(TokenType::RBrace)))?;

        Ok(tree_maker::Block(vec!()))
    }

    pub fn parse_shader_output(&mut self) -> parser::Result<Stmt> {
        let ty = self.expect_type()?;
        let name = self.expect_ident()?;

        self.expect_semi()?;

        Ok(tree_maker::Output(name, ty))
    }

    pub fn parse_shader_input(&mut self, provide: bool) -> parser::Result<Stmt> {
        let mut interpolation_modifier: Option<InputInterpolation> = None;
        let mut ty: Option<Type> = None;

        let next = self.expect_next(vec!(
            AnyType,
            Exact(TokenType::Keyword(token::Keyword::Flat)),
            Exact(TokenType::Keyword(token::Keyword::Smooth)),
            Exact(TokenType::Keyword(token::Keyword::Noperspective))
        ))?;

        match &next.ty {
            TokenType::Keyword(kw) => {
                match kw {
                    token::Keyword::Flat => interpolation_modifier = Some(InputInterpolation::Flat),
                    token::Keyword::Smooth => interpolation_modifier = Some(InputInterpolation::Smooth),
                    token::Keyword::Noperspective => interpolation_modifier = Some(InputInterpolation::Noperspective),
                    _ => unreachable!()
                }
            }
            TokenType::Ident(name) => {
                ty = Some(Type::from_str(name.to_string()))
            }
            _ => unreachable!()
        }

        if ty.is_none() {
            ty = Some(self.expect_type()?);
        }

        let name = self.expect_ident()?;
        self.expect_semi()?;

        if provide {
            if interpolation_modifier.is_none() {
                Ok(tree_maker::Provide(name, ty.unwrap()))
            } else {
                Ok(tree_maker::ProvideInterp(name, ty.unwrap(), interpolation_modifier.unwrap()))
            }
        } else {
            if interpolation_modifier.is_none() {
                Ok(tree_maker::Input(name, ty.unwrap()))
            } else {
                Ok(tree_maker::InputInterp(name, ty.unwrap(), interpolation_modifier.unwrap()))
            }
        }
    }

    fn expect_next(&mut self, expected: Vec<TokenExpectation>) -> parser::Result<Token> {
        let next = self.unwrap_next()?;

        let matches = expected.iter().any(|expectation| {
            match expectation {
                Exact(ty) => *ty == next.ty,
                AnyIdentifier | AnyType => {
                    matches!(next.ty, TokenType::Ident(_))
                }
            }
        });

        if !matches {
            return Err(ParseErr::UnexpectedToken(next, expected));
        }

        Ok(next)
    }

    fn expect_ident(&mut self) -> parser::Result<String> {
        let token = self.expect_next(vec!(AnyIdentifier))?;

        match token.ty {
            TokenType::Ident(name) => Ok(name),
            _ => unreachable!(),
        }
    }

    fn expect_type(&mut self) -> parser::Result<Type> {
        let token = self.expect_next(vec!(AnyType))?;

        match token.ty {
            TokenType::Ident(name) => Ok(Type::from_str(name)),
            _ => unreachable!(),
        }
    }

    fn expect_semi(&mut self) -> parser::Result<Token> {
        let token = self.expect_next(vec!(Exact(TokenType::Semi)))?;

        match token.ty {
            TokenType::Semi => Ok(token),
            _ => unreachable!(),
        }
    }

    fn skip(&mut self) {
        self.tokens.next();
    }

    fn unwrap_next(&mut self) -> parser::Result<Token> {
        self.tokens.next()
            .ok_or(ParseErr::UnexpectedEof)?
            .map_err(ParseErr::TokenizerErr)
    }

    fn unwrap_peek(&mut self) -> parser::Result<Token> {
        self.tokens.peek()
            .cloned()
            .ok_or(ParseErr::UnexpectedEof)?
            .map_err(ParseErr::TokenizerErr)
    }

    fn unwrap_token(token: Result<Token, TokenErr>) -> parser::Result<Token> {
        token.map_err(ParseErr::TokenizerErr)
    }
}
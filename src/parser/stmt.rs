use crate::ast::stmt::{InputInterpolation, MethodDeclStmt, Stmt};
use crate::ast::tree_maker;
use crate::ast::ty::Type;
use crate::parser::err::TokenExpectation::*;
use crate::parser::Parser;
use crate::token::{Operator, Token, TokenType};
use crate::{parser, token, Te};
use crate::parser::err::{ParseErr, ParseErrType};

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn parse_stmt(&mut self) -> parser::Result<Stmt> {
        let token = self.unwrap_next()?;

        match &token.ty {
            TokenType::Keyword(kw) => {
                let opt = self.parse_keyword(token.clone(), kw.clone())?;
                Ok(opt)
            }
            _ => panic!("{token:?}")
        }
    }

    pub fn parse_keyword(&mut self, token: Token, keyword: token::Keyword) -> parser::Result<Stmt> {
        match keyword {
            token::Keyword::Input => self.parse_shader_input(false),
            token::Keyword::Provide => self.parse_shader_input(true),
            token::Keyword::Output => self.parse_shader_output(),
            token::Keyword::Fn => self.parse_method_decl(),
            _ => { todo!() }
        }
    }

    pub fn parse_method_decl(&mut self) -> parser::Result<Stmt> {
        let name = self.expect_ident()?;
        let mut params = vec![];

        self.expect_next(vec![Exact(TokenType::LParen)])?;

        let mut next = self.unwrap_next()?;
        while next.ty != TokenType::RParen {
            let param_name = self.expect_ident()?;
            self.expect_next(vec!(Exact(TokenType::Colon)))?;
            let param_type = self.parse_type()?;

            params.push(tree_maker::MethodParam(param_name, param_type));
            next = self.unwrap_next()?;
        }

        let mut ret_type = None;

        if let TokenType::Operator(Operator::Merge) = self.unwrap_peek()?.ty {
            self.skip();
            ret_type = Some(self.parse_type()?);
        }

        if let Stmt::Block(stmt) = self.parse_block()? {
            Ok(tree_maker::MethodDef(name, params, ret_type, stmt))
        } else {
            Err(self.tokenizer_err(ParseErrType::InternalErr))
        }
    }

    pub fn parse_block(&mut self) -> parser::Result<Stmt> {
        self.expect_next(Te!('{'))?;

        let mut block = vec![];
        while let Some(_) = self.peek_token()? {
            let s = self.parse_stmt()?;
            block.push(s);
        }

        self.expect_next(Te!('}'))?;
        
        Ok(tree_maker::Block(block))
    }

    pub fn parse_shader_output(&mut self) -> parser::Result<Stmt> {
        let ty = self.parse_type()?;
        let name = self.expect_ident()?;

        self.expect_semi()?;

        Ok(tree_maker::Output(name, ty))
    }

    pub fn parse_shader_input(&mut self, provide: bool) -> parser::Result<Stmt> {
        let mut interpolation_modifier: Option<InputInterpolation> = None;
        let ty: Option<Type> = None;

        let next = self.expect_next(Te!(ID flat smooth noperspective))?;

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
                todo!()
            }
            _ => unreachable!()
        }

        let ty = match ty {
            None => self.parse_type()?,
            Some(ty) => ty
        };

        let name = self.expect_ident()?;
        self.expect_semi()?;

        if provide {
            if let Some(ipm) = interpolation_modifier {
                Ok(tree_maker::ProvideInterp(name, ty, ipm))
            } else {
                Ok(tree_maker::Provide(name, ty))
            }
        } else {
            if let Some(ipm) = interpolation_modifier {
                Ok(tree_maker::InputInterp(name, ty, ipm))
            } else {
                Ok(tree_maker::Input(name, ty))
            }
        }
    }
}
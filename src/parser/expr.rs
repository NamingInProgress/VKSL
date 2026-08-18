use crate::ast::expr::{BinExpr, Expr};
use crate::parser;
use crate::parser::Parser;
use crate::token::{Token, TokenType};

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn parse_expr(&mut self) -> parser::Result<Expr> {
        self.parse_expr_with_precedence(0)
    }

    fn parse_expr_with_precedence(&mut self, min_precedence: u8) -> parser::Result<Expr> {
        let mut lhs = self.parse_primary_expression()?;
        while let Some(Token { ty: TokenType::Operator(op), ctx, .. }) = self.peek_token()? {
            let precedence = op.precedence(&ctx, || self.tokens.tail_default())?;

            if precedence < min_precedence {
                break;
            }

            self.skip();

            let mut rhs = self.parse_primary_expression()?;
            while let Some(Token { ty: TokenType::Operator(inner_op), ctx: inner_ctx, .. }) = self.peek_token()? {
                let inner_precedence = inner_op.precedence(&inner_ctx, || self.tokens.tail_default())?;

                if inner_precedence <= precedence {
                    break;
                }

                self.skip();

                let extra = self.parse_expr_with_precedence(inner_precedence)?;
                rhs = Expr::Binary(BinExpr {
                    lhs: Box::new(rhs),
                    op: inner_op,
                    rhs: Box::new(extra),
                })
            }
            lhs = Expr::Binary(BinExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });
        }
        Ok(lhs)
    }

    pub fn parse_primary_expression(&mut self) -> parser::Result<Expr> {
        todo!()
    }
}
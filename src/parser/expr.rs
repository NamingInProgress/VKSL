use crate::ast::expr::{AccessExpr, BinExpr, Expr, FnCallExpr, IndexExpr, LitExpr, PostFixExpr, PreFixExpr, TernaryExpr, UnaryExpr, VarExpr};
use crate::{parser, Te, T};
use crate::parser::err::ParseErrType;
use crate::parser::Parser;
use crate::token::{Operator, Token, TokenType};

impl<I: Iterator<Item = char>> Parser<I> {
    pub fn parse_expr(&mut self) -> parser::Result<Expr> {
        self.parse_expr_with_precedence(0)
    }

    fn parse_expr_with_precedence(&mut self, min_precedence: u8) -> parser::Result<Expr> {
        let mut lhs = self.parse_primary_expression()?;
        while let Some(Token {
            ty: TokenType::Operator(op),
            ctx,
            ..
        }) = self.peek_token()?
        {
            let precedence = op.precedence(&ctx, || self.tokens.tail_default())?;

            if precedence < min_precedence {
                break;
            }

            self.skip();

            let mut rhs = self.parse_primary_expression()?;
            while let Some(Token {
                ty: TokenType::Operator(inner_op),
                ctx: inner_ctx,
                ..
            }) = self.peek_token()?
            {
                let inner_precedence =
                    inner_op.precedence(&inner_ctx, || self.tokens.tail_default())?;

                if inner_precedence <= precedence {
                    break;
                }

                self.skip();

                let extra = self.parse_expr_with_precedence(inner_precedence)?;
                rhs = Expr::Binary(BinExpr {
                    lhs: Box::new(rhs),
                    op: inner_op,
                    op_tkn: inner_ctx,
                    rhs: Box::new(extra),
                })
            }
            lhs = Expr::Binary(BinExpr {
                lhs: Box::new(lhs),
                op,
                op_tkn: ctx,
                rhs: Box::new(rhs),
            });
        }
        Ok(lhs)
    }

    //     Unary(UnaryExpr),  - || ! $prim      --DONE
    //     Binary(BinExpr), $parse              --DONE
    //     FnCall(FnCallExpr), ident(params?)   --DONE
    //     Variable(VarExpr),  ident            --DONE
    //     Literal(LitExpr), lit                --DONE
    //     PreFix(PreFixExpr), -- || ++ $prim   --DONE
    //     Unnamed,             ($parse)        --DONE

    fn parse_primary_expression(&mut self) -> parser::Result<Expr> {
        let next = self.unwrap_next()?;
        let primary = match next.ty {
            TokenType::Literal(lit) => {
                Expr::Literal(LitExpr {
                    lit,
                    lit_tkn: next.ctx,
                })
            }
            TokenType::Operator(op) if matches!(op, Operator::Minus | Operator::Not) => {
                let expr = self.parse_primary_expression()?;
                Expr::Unary(UnaryExpr {
                    op,
                    op_tkn: next.ctx,
                    expr: Box::new(expr),
                })
            }
            TokenType::Operator(op) if matches!(op, Operator::MinusMinus | Operator::PlusPlus) => {
                let expr = self.parse_primary_expression()?;
                Expr::PreFix(PreFixExpr {
                    op,
                    op_tkn: next.ctx,
                    expr: Box::new(expr),
                })
            }
            TokenType::LParen => {
                let expr = self.parse_expr()?;
                self.expect_next(Te![')'])?;
                expr
            }
            TokenType::Ident(name) => {
                let ident = (name, next.ctx).into();
                if let Ok(Token { ty: TokenType::LParen, .. }) = self.unwrap_peek() {
                    let paren1 = self.unwrap_next()?;
                    let mut args = Vec::new();
                    while !matches!(self.unwrap_peek(), Ok(Token { ty: TokenType::RParen, .. })) {
                        let arg = self.parse_expr()?;
                        let comma = if let Some(Token { ty: T!(,), ctx }) = self.peek_token()? {
                            self.unwrap_next()?;
                            Some(ctx)
                        } else { None };
                        args.push((arg, comma));
                    }
                    let paren2 = self.expect_next(Te!(')'))?;
                    Expr::FnCall(FnCallExpr {
                        ident,
                        paren1_tkn: paren1.ctx,
                        args,
                        paren2_tkn: paren2.ctx,
                    })
                } else {
                    Expr::Variable(VarExpr {
                        ident,
                    })
                }
            }
            TokenType::LBrace => {
                // block stmt
                todo!()
            }
            TokenType::LBracket => {
                // array decl
                todo!()
            }
            c => {
                let (err, hint) = self.deduce_failed_primary_expression_parse_error(c);

                return Err(self.token_err_with_hint(err, next.ctx, hint));
            }
        };
        self.parse_secondary_expression(primary)
    }

    fn deduce_failed_primary_expression_parse_error(&mut self, tok: TokenType) -> (ParseErrType, String) {
        match tok {
            TokenType::Literal(_) | TokenType::LParen | TokenType::Ident(_) | TokenType::LBrace | TokenType::LBracket => unreachable!(),
            TokenType::Operator(op) if matches!(op, Operator::Minus | Operator::Not | Operator::MinusMinus | Operator::PlusPlus) => unreachable!(),
            TokenType::RParen => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::RParen), "consider trying to remove the erroneous `)`"),
            TokenType::RBrace => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::RBrace), "consider trying to remove the erroneous `}`"),
            TokenType::RBracket => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::RBracket), "consider trying to remove the erroneous `]`"),
            TokenType::Comma => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::Comma), "`,` is used to separate expressions, and thus requires and preceding valid expression\nconsider trying to remove the erroneous `,` or adding a valid expression before the `,`"),
            TokenType::Semi => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::Semi), "consider adding a valid expression before the `;`"),
            TokenType::Colon => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::Colon), "`:` cannot be used outside of tetrahedron operators, consider removing the `:` or wrapping in a tetrahedron operator\nif you already are inside a tetrahedron operator, consider adding a valid expression before the `:`"),
            TokenType::Question => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::Question), "`?` requires a preceding expression, consider adding a valid expression before the `?`"),
            TokenType::Keyword(keyword) => {}
            TokenType::Operator(op) => {}
            TokenType::OperatorAssign(op) => {}
        }

        // default
        (ParseErrType::InvalidExpression, "consider using different tokens to create a valid expression! Read the documentation for details. Maybe try using a variable name?".to_string())
    }

    //     Ternary(TernaryExpr), $secon ? $parse : $parse   --DONE
    //     Access(AccessExpr),  $secon . (ident.)*          --DONE
    //     PostFix(PostFixExpr), $secon ++ || --            --DONE
    //     Index(IndexExpr),  $secon[$parse]                --DONE

    fn parse_secondary_expression(&mut self, primary: Expr) -> parser::Result<Expr> {
        match self.unwrap_peek() {
            Ok(Token { ty: TokenType::Question, ctx }) => {
                self.skip();
                let yes = self.parse_expr()?;
                let colon_tkn = self.expect_next(Te![:])?.ctx;
                let no = self.parse_expr()?;
                Ok(Expr::Ternary(TernaryExpr {
                    cond: Box::new(primary),
                    question_tkn: ctx,
                    yes: Box::new(yes),
                    colon_tkn,
                    no: Box::new(no),
                }))
            }
            Ok(Token { ty: TokenType::Operator(Operator::Dot), ctx }) => {
                self.skip();
                let ident = self.expect_ident()?;
                self.parse_secondary_expression(Expr::Access(AccessExpr {
                    parent: Box::new(primary),
                    dot_tkn: ctx,
                    child: ident,
                }))
            }
            Ok(Token { ty: TokenType::Operator(op), ctx }) if matches!(op, Operator::MinusMinus | Operator::PlusPlus) => {
                self.skip();
                Ok(Expr::PostFix(PostFixExpr {
                    op,
                    op_tkn: ctx,
                    expr: Box::new(primary),
                }))
            }
            Ok(Token { ty: TokenType::LBracket, ctx }) => {
                self.skip();
                let expr = self.parse_expr()?;
                let close_tkn = self.expect_next(Te![']'])?.ctx;
                self.parse_secondary_expression(Expr::Index(IndexExpr {
                    array: Box::new(primary),
                    open_tkn: ctx,
                    index: Box::new(expr),
                    close_tkn
                }))
            }
            _ => Ok(primary)
        }
    }
}

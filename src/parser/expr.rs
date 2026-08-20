use crate::parser::ast::expr::{AccessExpr, AccessFnCallExpr, ArrayExpr, AssignExpr, BinExpr, BlockExpr, Expr, FnCallExpr, IndexExpr, LitExpr, NonuniformExpr, PostFixExpr, PreFixExpr, TernaryExpr, ThisExpr, TupleAccessExpr, TupleExpr, UnaryExpr, VarExpr};
use crate::{parser, Te, T};
use crate::parser::ast::{AsExpr, ConstructExpr, FieldInit};
use crate::parser::ast::ty::PrimitiveType;
use crate::parser::err::ParseErrType;
use crate::parser::Parser;
use crate::token::{Keyword, Literal, Operator, Token, TokenType};

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
            let precedence = op.precedence(&ctx)?;

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
                    inner_op.precedence(&inner_ctx)?;

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
    //     Construct(ConExpr), ident {i:$expr*} --DONE
    //     Literal(LitExpr), lit                --DONE
    //     Nu(NonUniExpr), nonuniform $parse    --DONE
    //     PreFix(PreFixExpr), -- || ++ $prim   --DONE
    //     Tuple(TupleExpr), ($parse(,$parse)*) --DONE
    //     Array(ArrayExpr), [$parse(,$parse)*] --DONE
    //     Block(BlockExpr), {$stmt*}           --DONE
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
            TokenType::Keyword(Keyword::Nonuniform) => {
                let expr = self.parse_expr()?;
                Expr::Nonuniform(NonuniformExpr {
                    nonuniform_tkn: next.ctx,
                    expr: Box::new(expr),
                })
            }
            TokenType::Keyword(Keyword::This) => {
                Expr::This(ThisExpr {
                    this_tkn: next.ctx,
                })
            }
            TokenType::Ident(name) => {
                let ident = (name, next.ctx).into();
                if let Ok(Token { ty: TokenType::LParen, .. }) = self.unwrap_peek() {
                    let open_tkn = self.unwrap_next()?.ctx;
                    let args = self.parse_punctuated(T![,], T![')'], true, Self::parse_expr).try_collect()?;
                    let close_tkn = self.expect_next(Te!(')'))?.ctx;
                    Expr::FnCall(FnCallExpr {
                        ident,
                        open_tkn,
                        args,
                        close_tkn,
                    })
                } else if let Ok(Token { ty: TokenType::LBrace, .. }) = self.unwrap_peek() {
                    fn parse_struct_assign<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> parser::Result<FieldInit> {
                        let name = parser.expect_ident()?;
                        let colon_tkn = parser.expect_next(Te![:])?.ctx;
                        let expr = parser.parse_expr()?;
                        Ok(FieldInit {
                            name,
                            colon_tkn,
                            expr: Box::new(expr),
                        })
                    }
                    let open_tkn = self.unwrap_next()?.ctx;
                    let fields = self.parse_punctuated(T![,], T!['}'], true, parse_struct_assign).try_collect()?;
                    let close_tkn = self.expect_next(Te!('}'))?.ctx;
                    Expr::Construct(ConstructExpr {
                        name: ident,
                        brace1_tkn: open_tkn,
                        fields,
                        brace2_tkn: close_tkn,
                    })
                } else {
                    Expr::Variable(VarExpr {
                        ident,
                    })
                }
            }
            TokenType::LParen => {
                let mut args = self.parse_punctuated(T![,], T![')'], true, Self::parse_expr).try_collect()?;
                let close_tkn = self.expect_next(Te![')'])?.ctx;

                if args.len() == 1 && args[0].1.is_none()  {
                    args.pop().expect("vec has exactly 1 element").0
                } else {
                    Expr::Tuple(TupleExpr {
                        open_tkn: next.ctx,
                        args,
                        close_tkn,
                    })
                }
            }
            TokenType::LBracket => {
                let args = self.parse_punctuated(T![,], T![']'], true, Self::parse_expr).try_collect()?;
                let close_tkn = self.expect_next(Te![']'])?.ctx;
                Expr::Array(ArrayExpr {
                    open_tkn: next.ctx,
                    args,
                    close_tkn,
                })
            }
            TokenType::LBrace => {
                let mut block = Vec::new();
                while !matches!(self.unwrap_peek(), Ok(Token { ty: TokenType::RBrace, .. } )) {
                    block.push(self.parse_stmt()?);
                }
                let close_tkn = self.expect_next(Te!['}'])?.ctx;
                Expr::Block(BlockExpr {
                    open_tkn: next.ctx,
                    block,
                    close_tkn,
                    fut_scope: None,
                })
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
            TokenType::Literal(_) | TokenType::Ident(_) | TokenType::LParen | TokenType::LBrace | TokenType::LBracket | TokenType::Keyword(Keyword::Nonuniform) | TokenType::Keyword(Keyword::This) => unreachable!(),
            TokenType::Operator(op) if matches!(op, Operator::Minus | Operator::Not | Operator::MinusMinus | Operator::PlusPlus) => unreachable!(),
            TokenType::RParen | TokenType::RBrace | TokenType::RBracket => (ParseErrType::UnconstrainedUnexpectedToken(tok.clone()), format!("consider trying to remove the erroneous `{tok}`")),
            TokenType::Comma => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::Comma), "`,` is used to separate expressions, and thus requires and preceding valid expression\nconsider trying to remove the erroneous `,` or adding a valid expression before the `,`".to_string()),
            TokenType::Semi => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::Semi), "consider adding a valid expression before the `;`".to_string()),
            TokenType::Colon => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::Colon), "`:` cannot be used outside of tetrahedron operators, consider removing the `:` or wrapping in a tetrahedron operator\nif you already are inside a tetrahedron operator, consider adding a valid expression before the `:`".to_string()),
            TokenType::Question => (ParseErrType::UnconstrainedUnexpectedToken(TokenType::Question), "`?` requires a preceding expression, consider adding a valid expression before the `?`".to_string()),
            TokenType::Operator(op) => (ParseErrType::NonUnaryCompatibleOperator(op), format!("consider removing `{op}` or adding an expression before it to transform into a binary expression")),
            TokenType::OperatorAssign(op) => (ParseErrType::AssignOpNotAllowedAtExpressionHead, format!("consider transforming into a non assignment expression: `{op}`, or removing the erroneous `{op}`")),
            TokenType::Keyword(keyword) => {
                todo!()
            }
            //_ => (ParseErrType::InvalidExpression, "consider using different tokens to create a valid expression! Read the documentation for details. Maybe try using a variable name?".to_string())
        }
    }

    //     Ternary(TernaryExpr), $secon ? $parse : $parse   --DONE
    //     Access(AccessExpr),  $secon . (ident.)*          --DONE
    //     AccessFn(AccessFnExpr),  $secon . ident($parse*) --DONE
    //     PostFix(PostFixExpr), $secon ++ || --            --DONE
    //     Index(IndexExpr),  $secon[$parse]                --DONE
    //     Assign(AssignExpr), $secon = $parse              --DONE
    //     As(AsExpr),  $secon as $type                     --DONE

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
                if let Ok(Token { ty: TokenType::Literal(lit), ctx }) = self.unwrap_peek() {
                    let component = match lit {
                        Literal::UIntLit(v) => v,
                        Literal::IntLit(i) if i >= 0 => i as u32,
                        Literal::IntLit(i) => return Err(self.token_err_with_hint(ParseErrType::IllegalTupleAccessLiteral(PrimitiveType::Int), ctx, format!("negative numbers cannot be used for tuple access, consider replacing with a positive number: `.{}`", -i))),
                        Literal::BoolLit(b) => return Err(self.token_err_with_hint(ParseErrType::IllegalTupleAccessLiteral(PrimitiveType::Bool), ctx, format!("booleans cannot be used to access tuple elements, consider replacing `.{b}` with a positive number like `.0`"))),
                        Literal::FloatLit(f) => return Err(self.token_err_with_hint(ParseErrType::IllegalTupleAccessLiteral(PrimitiveType::F32), ctx, format!("accessing tuple elements with fractional types is illegal, consider rounding your number: `.{}`", {let f = f.round() as i32; if f < 0 { -f } else { f }}))),
                        Literal::DoubleLit(f) => return Err(self.token_err_with_hint(ParseErrType::IllegalTupleAccessLiteral(PrimitiveType::F64), ctx, format!("accessing tuple elements with fractional types is illegal, consider rounding your number: `.{}`", {let f = f.round() as i64; if f < 0 { -f } else { f }}))),
                    };
                    self.parse_secondary_expression(Expr::TupleAccess(TupleAccessExpr {
                        tuple: Box::new(primary),
                        component,
                    }))
                } else {
                    let ident = self.expect_ident()?;
                    if let Ok(Token { ty: TokenType::LParen, .. }) = self.unwrap_peek() {
                        let open_tkn = self.unwrap_next()?.ctx;
                        let args = self.parse_punctuated(T![,], T![')'], true, Self::parse_expr).try_collect()?;
                        let close_tkn = self.expect_next(Te!(')'))?.ctx;
                        self.parse_secondary_expression(Expr::AccessFnCall(AccessFnCallExpr {
                            parent: Box::new(primary),
                            ident,
                            open_tkn,
                            args,
                            close_tkn,
                        }))
                    } else {
                        self.parse_secondary_expression(Expr::Access(AccessExpr {
                            parent: Box::new(primary),
                            dot_tkn: ctx,
                            child: ident,
                        }))
                    }
                }
            }
            Ok(Token { ty: TokenType::Operator(op), ctx }) if matches!(op, Operator::MinusMinus | Operator::PlusPlus) => {
                self.skip();
                Ok(Expr::PostFix(PostFixExpr {
                    op,
                    op_tkn: ctx,
                    expr: Box::new(primary),
                }))
            }
            Ok(Token { ty: TokenType::Operator(Operator::Assign), ctx }) => {
                self.skip();
                let value = self.parse_expr()?;
                Ok(Expr::Assign(AssignExpr {
                    lhs: Box::new(primary),
                    eq_tkn: ctx,
                    rhs: Box::new(value),
                }))
            }
            Ok(Token { ty: TokenType::Keyword(Keyword::As), ctx }) => {
                self.skip();
                let ty = self.parse_type()?;
                Ok(Expr::As(AsExpr {
                    expr: Box::new(primary),
                    as_tkn: ctx,
                    ty,
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

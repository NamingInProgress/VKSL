use std::str::FromStr;
use crate::parser::TokenType;
use crate::ast::ty::{ArrayType, PathType, PrimitiveType, Type};
use crate::{parser, Te, T};
use crate::parser::err::{ParseErrType, TokenExpectation};
use crate::parser::Parser;
use crate::token::{Keyword, Literal, Operator, Token};

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn parse_type(&mut self) -> parser::Result<Type> {
        let tok = self.unwrap_next()?;
        let base = match tok.ty {
            TokenType::Ident(start) if matches!(self.peek_token()?, Some(Token { ty: T!(.), .. })) => {
                let mut all = vec![start];
                while let Some(Token { ty: T!(.), .. }) = self.peek_token()? {
                    self.unwrap_next()?;
                    let ident = self.expect_ident()?;
                    all.push(ident);
                }
                let mut iter = all.into_iter().rev();
                let name = iter.next().expect("We had at least 1 up here");
                let parts = iter.rev().collect::<Vec<_>>();
                Type::PathType(PathType {
                    path_parts: parts,
                    name,
                })
            }
            TokenType::Ident(ident) => {
                if let Ok(prim) = PrimitiveType::from_str(&ident) {
                    Type::Primitive(prim)
                } else {
                    Type::SingleType(ident)
                }
            }
            TokenType::Keyword(Keyword::Struct) => {
                todo!()
            }
            _ => {
                todo!("add type error here for misformed type")
            }
        };

        self.handle_type(base)
    }

    fn handle_type(&mut self, mut ty: Type) -> parser::Result<Type> {
        if let Some(tok) = self.peek_token()? {
            if tok.ty == TokenType::LBracket {
                self.unwrap_next()?;
                let dim = if let Some(maybe_num) = self.peek_token()? {
                    match maybe_num.ty {
                        TokenType::Literal(Literal::UIntLit(dim)) => {
                            self.unwrap_next()?;
                            Some(dim)
                        }
                        TokenType::Literal(lit) => {
                            self.unwrap_next()?;

                            let ty = ty.to_string();
                            let hint_hint = match lit {
                                Literal::BoolLit(b) => format!("consider replacing '{b}' with a literal type: {ty}[1u]"),
                                Literal::IntLit(i) => format!("consider refactoring your literal: `{ty}[{}u]`", if i < 0 { -i } else { i }),
                                Literal::FloatLit(f) => format!("consider refactoring your literal: `{ty}[{}u]`", { let i = f as i32; if i < 0 { -i } else { i }}),
                                Literal::DoubleLit(f) => format!("consider refactoring your literal: `{ty}[{}u]`", { let i = f as i64; if i < 0 { -i } else { i }}),
                                Literal::UIntLit(_) => unreachable!(),
                            };
                            let hint = format!("array dimensions must be constant unsigned integers, {hint_hint}");
                            return Err(self.token_err_with_hint(ParseErrType::IllegalArrayDim, maybe_num, hint));
                        }
                        _ => {
                            None
                        }
                    }
                } else {
                    None
                };

                let _ = self.expect_next(Te![']'])?;
                return self.handle_type(Type::ArrayOf(ArrayType {
                    component: Box::new(ty),
                    dimension: dim,
                }));
            }
        }

        Ok(ty)
    }
}
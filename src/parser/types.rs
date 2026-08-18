use crate::ast::ty::{ArrayType, PathType, PrimitiveType, Type};
use crate::parser::err::{ParseErr, ParseErrType};
use crate::parser::Parser;
use crate::parser::TokenType;
use crate::token::{TokCtx, Keyword, Literal, Token};
use crate::{ast, parser, Te, T};
use std::str::FromStr;
use itertools::Itertools;
use mvutils::enum_val;
use crate::ast::Ident;
use crate::ast::stmt::Stmt;

pub struct NTD {
    pub name: ast::Ident,
    pub ty: Option<Type>,
    pub colon_tk: Option<TokCtx>,
}

impl<I: Iterator<Item = char>> Parser<I> {
    pub fn parse_name_type_def_single(&mut self, needs_type: bool) -> parser::Result<NTD> {
        Ok(self.parse_name_type_def(needs_type, false)?.into_iter().next().expect("parse name type def makes sure ts exists"))
    }

    pub fn parse_name_type_def(&mut self, needs_type: bool, allow_multiple: bool) -> parser::Result<Vec<NTD>> {
        let mut names = vec!();

        loop {
            names.push(self.expect_ident()?);
            let t = self.unwrap_peek()?;
            match &t.ty {
                TokenType::Comma => {
                    self.unwrap_next()?;
                }
                TokenType::Colon => {
                    self.unwrap_next()?;
                    let ty = self.parse_type()?;

                    if !allow_multiple && names.len() > 1 {
                        let converted_lines = names.into_iter()
                            .map(|n| format!("\t{n}: {ty}"))
                            .join("\n");
                        let hint = format!("multiple names per type not allowed here! Consider splitting into separate definitions:\n{converted_lines}");
                        return Err(self.token_err_with_hint(ParseErrType::NoMultipleNamesPerType, t.ctx, hint));
                    }

                    return Ok(
                        names
                            .into_iter()
                            .map(|name| NTD { name, ty: Some(ty.clone()), colon_tk: Some(t.ctx.clone()) })
                            .collect::<Vec<_>>()
                    )
                }
                _ => {
                    if needs_type {
                        let field_names = names.into_iter().map(|i| i.val).join(", ");
                        let e = self.token_err_with_hint(ParseErrType::MissingType, t.ctx, format!("type annotation required here! Consider adding a type: `{field_names}: <type>`"));
                        return Err(e);
                    }
                    break
                }
            }
        }

        Ok(
            names
                .into_iter()
                .map(|name| NTD { name, ty: None, colon_tk: None })
                .collect::<Vec<_>>()
        )
    }

    pub fn parse_type(&mut self) -> parser::Result<Type> {
        let tok = self.unwrap_peek()?;
        let base = match tok.ty {
            TokenType::Ident(start) if matches!(self.peek_token()?, Some(Token { ty: T!(.), .. })) => {
                self.unwrap_next()?;
                let mut all = vec![Ident {
                    val: start,
                    tkn: tok.ctx,
                }];
                let mut dot_tkns = vec![];
                while let Some(Token { ty: T!(.), ctx }) = self.peek_token()? {
                    self.unwrap_next()?;
                    let ident = self.expect_ident()?;
                    all.push(ident);
                    dot_tkns.push(ctx);
                }
                let mut iter = all.into_iter().rev();
                let name = iter.next().expect("We had at least 1 up here");
                let parts = iter.rev().collect::<Vec<_>>();
                Type::PathType(PathType {
                    path_parts: parts,
                    dot_tkns,
                    name,
                })
            }
            TokenType::Ident(ident) => {
                let tok = self.unwrap_next()?;
                if let Ok(prim) = PrimitiveType::from_str(&ident) {
                    Type::Primitive(prim, tok.ctx)
                } else {
                    Type::SingleType(ident, tok.ctx)
                }
            }
            TokenType::Keyword(Keyword::Struct) => {
                let strct = self.parse_struct()?;
                let strct = enum_val!(Stmt, strct, Struct);
                Type::StructDef(Box::new(strct))
            }
            _ => {
                let tail = self.tokens.tail_default();
                let e = ParseErr {
                    ty: ParseErrType::UnexpectedToken(tok.clone(), Te!(ID struct)),
                    ctx: tok.ctx,
                    tail,
                    hint: Some("consider using different tokens to build a valid type! Read the specification for details. Maybe use the type `uint`?".to_string()),
                };
                return Err(e);
            }
        };

        self.handle_type(base)
    }

    fn handle_type(&mut self, ty: Type) -> parser::Result<Type> {
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
                                Literal::BoolLit(b) => format!(
                                    "consider replacing '{b}' with a literal type: {ty}[1u]"
                                ),
                                Literal::IntLit(i) => format!(
                                    "consider refactoring your literal: `{ty}[{}u]`",
                                    if i < 0 { -i } else { i }
                                ),
                                Literal::FloatLit(f) => {
                                    format!("consider refactoring your literal: `{ty}[{}u]`", {
                                        let i = f as i32;
                                        if i < 0 { -i } else { i }
                                    })
                                }
                                Literal::DoubleLit(f) => {
                                    format!("consider refactoring your literal: `{ty}[{}u]`", {
                                        let i = f as i64;
                                        if i < 0 { -i } else { i }
                                    })
                                }
                                Literal::UIntLit(_) => unreachable!(),
                            };
                            let hint = format!(
                                "array dimensions must be constant unsigned integers, {hint_hint}"
                            );
                            return Err(self.token_err_with_hint(
                                ParseErrType::IllegalArrayDim,
                                maybe_num.ctx,
                                hint,
                            ));
                        }
                        _ => None,
                    }
                } else {
                    None
                };

                let brack2_tkn = self.expect_next(Te![']'])?;
                return self.handle_type(Type::ArrayOf(ArrayType {
                    component: Box::new(ty),
                    brack1_tkn: tok.ctx,
                    dimension: dim,
                    brack2_tkn: brack2_tkn.ctx,
                }));
            }
        }

        Ok(ty)
    }
}

use crate::ast::stmt::{BlockStmt, BreakStmt, CompoundStmt, ContinueStmt, ExprStmt, ExtensionStmt, ForStmt, IfStmt, InputStmt, MethodDeclStmt, MethodParamDecl, OutputStmt, ProvideStmt, PushConstantsStmt, ReturnStmt, SemiStmt, Stmt, StructField, StructStmt, UniformStmt, UniformType, VarDeclStmt, WhileStmt, YieldStmt};
use crate::ast::ty::Type;
use crate::parser::err::TokenExpectation::*;
use crate::parser::mods::{ModRule, Modifier, ModsUsage};
use crate::parser::Parser;
use crate::token::{Keyword, Operator, TokenType};
use crate::{parser, Te, T};
use mvutils::enum_val;
use crate::parser::err::ParseErrType;

impl<I: Iterator<Item = char>> Parser<I> {
    pub fn parse_stmt(&mut self) -> parser::Result<Stmt> {
        let token = self.unwrap_peek()?;

        match token.ty {
            TokenType::Semi => {
                Ok(SemiStmt { semi_tkn: self.expect_semi()?.ctx }.into())
            }
            TokenType::Keyword(kw) => {
                Ok(self.parse_keyword(kw)?)
            }
            TokenType::LBrace => {
                Ok(self.parse_block()?.into())
            }
            _ => Ok(self.parse_expr_stmt()?),
        }
    }

    pub fn parse_expr_stmt(&mut self) -> parser::Result<Stmt> {
        Ok(
            ExprStmt {
                expr: self.parse_expr()?,
                semi_tkn: self.expect_semi()?.ctx,
            }.into()
        )
    }

    pub fn parse_keyword(&mut self, keyword: Keyword) -> parser::Result<Stmt> {
        match keyword {
            Keyword::Input => self.parse_shader_input(false),
            Keyword::Provide => self.parse_shader_input(true),
            Keyword::Output => self.parse_shader_output(),
            Keyword::Fn => self.parse_method_decl(),
            Keyword::Struct => self.parse_struct(),
            Keyword::Uniform => self.parse_uniform_decl(),
            Keyword::PushConstants => self.parse_push_constants(),
            Keyword::Extension => self.parse_extension(),
            Keyword::Let => self.parse_var_decl(false),
            Keyword::Const => self.parse_var_decl(true),
            Keyword::Return => self.parse_return(),
            Keyword::If => self.parse_if(),
            Keyword::For => self.parse_for(),
            Keyword::While => self.parse_while(),
            Keyword::Yield => self.parse_yield(),
            Keyword::Break => Ok(Stmt::Break(BreakStmt {})),
            Keyword::Continue => Ok(Stmt::Continue(ContinueStmt {})),
            _ => {
                let tk = self.unwrap_next()?.ctx;
                Err(self.token_err_with_hint(
                    ParseErrType::UnconstrainedUnexpectedToken(TokenType::Keyword(keyword)),
                    tk,
                    format!("Keyword {keyword:?} not allowed here! Try reading the documentation.")
                ))
            }
        }
    }

    pub fn parse_yield(&mut self) -> parser::Result<Stmt> {
        let yield_tkn = self.expect_next(Te!(yield))?.ctx;

        let expr = self.parse_expr()?;

        let semi_tkn = self.expect_semi()?.ctx;
        Ok(
            YieldStmt {
                yield_tkn,
                expr,
                semi_tkn
            }.into()
        )
    }

    pub fn parse_while(&mut self) -> parser::Result<Stmt> {
        let while_tkn = self.expect_next(Te!(while))?.ctx;
        let l_paren = self.expect_next(Te!('('))?.ctx;

        let cond = self.parse_expr()?;
        
        let r_paren = self.expect_next(Te!(')'))?.ctx;
        
        let block = self.parse_block_or_stmt()?;
        
        Ok(
            WhileStmt {
                while_tkn,
                l_paren,
                cond,
                r_paren,
                block
            }.into()
        )
    }

    pub fn parse_for(&mut self) -> parser::Result<Stmt> {
        let for_tkn = self.expect_next(Te!(for))?.ctx;
        let l_paren = self.expect_next(Te!('('))?.ctx;
        
        let mut start_cond = None;
        if !self.check_next(Te!(;))? {
            start_cond = Some(self.parse_expr()?);
        }
        
        let semi1_tkn = self.expect_semi()?.ctx;
        
        let mut cond = None;
        if !self.check_next(Te!(;))? {
            cond = Some(self.parse_expr()?);
        }
        
        let semi2_tkn = self.expect_semi()?.ctx;

        let mut after_run = None;
        if !self.check_next(Te!(')'))? {
            after_run = Some(self.parse_expr()?);
        }
        
        let r_paren = self.expect_next(Te!(')'))?.ctx;
        
        let block = self.parse_block_or_stmt()?;
        
        Ok(
            ForStmt {
                for_tkn,
                l_paren,
                start_cond,
                semi1_tkn,
                cond,
                semi2_tkn,
                after_run,
                r_paren,
                block
            }.into()
        )
    }

    pub fn parse_if(&mut self) -> parser::Result<Stmt> {
        let if_tkn = self.expect_next(Te!(if))?.ctx;
        let l_paren = self.expect_next(Te!('('))?.ctx;
        let cond = self.parse_expr()?;
        let r_paren = self.expect_next(Te!(')'))?.ctx;

        let branch = self.parse_block_or_stmt()?;

        let mut else_branch = None;
        let mut else_tkn = None;
        if self.check_next(Te![else])? {
            else_tkn = Some(self.unwrap_next()?.ctx);
            else_branch = Some(self.parse_block_or_stmt()?);
        }

        Ok(
            IfStmt {
                if_tkn,
                l_paren,
                cond,
                r_paren,
                branch,
                else_tkn,
                else_branch
            }.into()
        )
    }
    
    pub fn parse_block_or_stmt(&mut self) -> parser::Result<Box<Stmt>> {
        let stmt;
        if self.check_next(Te!('{'))? {
            stmt = Box::new(Stmt::Block(self.parse_block()?));
        } else {
            stmt = Box::new(self.parse_stmt()?);
        }
        Ok(stmt)
    }

    pub fn parse_return(&mut self) -> parser::Result<Stmt> {
        let return_tkn = self.expect_next(Te!(return))?.ctx;

        if self.check_next(Te![;])? {
            Ok(
                ReturnStmt {
                    return_tkn,
                    expr: None,
                    semi_tkn: self.expect_semi()?.ctx
                }.into()
            )
        } else {
            Ok(
                ReturnStmt {
                    return_tkn,
                    expr: Some(self.parse_expr()?),
                    semi_tkn: self.expect_semi()?.ctx
                }.into()
            )
        }
    }

    pub fn parse_var_decl(&mut self, cnst: bool) -> parser::Result<Stmt> {
        let tk;
        if cnst {
            tk = self.expect_next(Te!(const))?.ctx;
        } else {
            tk = self.expect_next(Te!(let))?.ctx;
        }

        let ntd = self.parse_name_type_def(false, true)?;

        let mut init = None;
        let mut eq_tkn = None;
        let peek = self.unwrap_peek()?.ty;
        if peek == TokenType::Operator(Operator::Assign) {
            eq_tkn = Some(self.unwrap_next()?.ctx);
            init = Some(self.parse_expr()?)
        }

        let mut compound = CompoundStmt {
            components: vec![],
        };

        let semi_tkn = self.expect_semi()?.ctx;

        if cnst {
            for def in ntd {
                compound.components.push(
                    VarDeclStmt {
                        kw_tkn: tk.clone(),
                        name: def.name,
                        colon_tkn: def.colon_tk,
                        ty: def.ty,
                        eq_tkn: eq_tkn.clone(),
                        init: init.clone(),
                        semi_tkn: semi_tkn.clone(),
                        cnst: true
                    }.into()
                );
            }
        } else {
            for def in ntd {
                compound.components.push(
                    VarDeclStmt {
                        kw_tkn: tk.clone(),
                        name: def.name,
                        colon_tkn: def.colon_tk,
                        ty: def.ty,
                        eq_tkn: eq_tkn.clone(),
                        init: init.clone(),
                        semi_tkn: semi_tkn.clone(),
                        cnst: false
                    }.into()
                );
            }
        }

        Ok(compound.into())
    }

    pub fn parse_extension(&mut self) -> parser::Result<Stmt> {
        self.expect_next(Te!(extension))?;
        let extension = self.expect_ident()?;
        let mods = self.parse_mods()?;

        let res = self.validate_mods(mods, ModsUsage::default()
            .with_rule("extension", ModRule::Either(vec![Modifier::Enable, Modifier::Require, Modifier::Warn, Modifier::Disable]))
        )?;

        Ok(ExtensionStmt {
            mods: res,
            extension,
            semi_tkn: self.expect_semi()?.ctx,
        }.into())
    }

    pub fn parse_push_constants(&mut self) -> parser::Result<Stmt> {
        let pc_tkn = self.expect_next(Te!(push_constants))?.ctx;

        let ntd = self.parse_name_type_def_single(true)?;
        let name = ntd.name;
        let ty = ntd.ty.unwrap();

        Ok(PushConstantsStmt {
            pc_tkn,
            name,
            ty,
            semi_tkn: self.expect_semi()?.ctx
        }.into())
    }

    pub fn parse_uniform_decl(&mut self) -> parser::Result<Stmt> {
        let uniform_tkn = self.expect_next(Te!(uniform))?.ctx;

        let set_tkn = self.expect_ident_exact("set")?;
        let set_eq_tkn = self.expect_next(Te!(=))?.ctx;
        let (set, set_lit_tkn) = self.expect_non_negative_int()?;

        let binding_tkn = self.expect_ident_exact("binding")?;
        let binding_eq_tkn = self.expect_next(Te!(=))?.ctx;
        let (binding, binding_lit_tkn) =  self.expect_non_negative_int()?;

        let mods = self.parse_mods()?;

        let ssbo = self.check_next(Te!(buffer))?;
        if ssbo { self.unwrap_next()?; }

        let ntd = self.parse_name_type_def_single(true)?;
        let name = ntd.name;
        let ty = ntd.ty.unwrap();

        let semi_tkn = self.expect_semi()?.ctx;

        if ssbo {
            let usage = ModsUsage::default()
                .with_rule("packing_type", ModRule::EitherOr(vec![Modifier::STD140, Modifier::STD430]))
                .with_rule("access", ModRule::EitherOr(vec![Modifier::Readonly, Modifier::Writeonly]));

            let resolved = self.validate_mods(mods, usage)?;
            Ok(
                UniformStmt {
                    uniform_tkn,
                    name,
                    ty,

                    set_tkn: set_tkn.tkn,
                    set_eq_tkn,
                    set_lit_tkn,
                    set,

                    binding_tkn: binding_tkn.tkn,
                    binding_eq_tkn,
                    binding_lit_tkn,
                    binding,

                    mods: resolved,

                    uniform_type: UniformType::SSBO,
                    semi_tkn
                }.into()
            )
        } else if let Type::StructDef(_) = ty {
            let usage = ModsUsage::default()
                .with_rule("packing_type", ModRule::EitherOr(vec![Modifier::STD140, Modifier::STD430]));

            let resolved = self.validate_mods(mods, usage)?;
            Ok(
                UniformStmt {
                    uniform_tkn,
                    name,
                    ty,

                    set_tkn: set_tkn.tkn,
                    set_eq_tkn,
                    set_lit_tkn,
                    set,

                    binding_tkn: binding_tkn.tkn,
                    binding_eq_tkn,
                    binding_lit_tkn,
                    binding,

                    mods: resolved,

                    uniform_type: UniformType::UBO,
                    semi_tkn
                }.into()
            )
        } else {
            let resolved = self.validate_mods(mods, ModsUsage::default())?;
            Ok(
                UniformStmt {
                    uniform_tkn,
                    name,
                    ty,

                    set_tkn: set_tkn.tkn,
                    set_eq_tkn,
                    set_lit_tkn,
                    set,

                    binding_tkn: binding_tkn.tkn,
                    binding_eq_tkn,
                    binding_lit_tkn,
                    binding,

                    mods: resolved,

                    uniform_type: UniformType::Uniform,
                    semi_tkn
                }.into()
            )
        }
    }

    pub fn parse_method_decl(&mut self) -> parser::Result<Stmt> {
        let fn_tkn = self.expect_next(Te!(fn))?.ctx;
        let name = self.expect_ident()?;
        let mut params = vec![];

        let l_paren = self.expect_next(vec![Exact(TokenType::LParen)])?.ctx;

        let mut next = self.unwrap_peek()?;
        while next.ty != TokenType::RParen {
            let param = self.parse_name_type_def(true, false)?.into_iter().next().unwrap();

            next = self.unwrap_peek()?;
            let mut comma = None;

            if next.ty == TokenType::Comma {
                self.unwrap_next()?;
                comma = Some(next.ctx)
            }

            params.push(
                MethodParamDecl {
                    name: param.name,
                    colon_tkn: param.colon_tk.expect("requires type"),
                    ty: param.ty.expect("requires type"),
                    comma_tkn: comma
                }
            );
        }

        let r_paren = self.expect_next(Te!(')'))?.ctx;

        let mut return_type = None;
        let mut arrow_tkn = None;

        if let TokenType::Operator(Operator::Merge) = self.unwrap_peek()?.ty {
            arrow_tkn = Some(self.unwrap_next()?.ctx);
            return_type = Some(self.parse_type()?);
        }

        let block = self.parse_block()?;
        Ok(
            MethodDeclStmt {
                fn_tkn,
                name,
                l_paren,
                params,
                r_paren,
                arrow_tkn,
                return_type,
                block: Box::new(block.into())
            }.into()
        )
    }

    pub fn parse_block(&mut self) -> parser::Result<BlockStmt> {
        let l_brace = self.expect_next(Te!('{'))?.ctx;

        let mut block = vec![];
        while self.unwrap_peek()?.ty != T!('}') {
            let s = self.parse_stmt()?;
            block.push(s);
        }

        let r_brace = self.expect_next(Te!('}'))?.ctx;

        Ok(
            BlockStmt {
                l_brace,
                stmts: block,
                r_brace,
            }
        )
    }

    pub fn parse_shader_output(&mut self) -> parser::Result<Stmt> {
        let output_tkn = self.expect_next(Te!(output))?.ctx;
        let ty = self.parse_type()?;
        let name = self.expect_ident()?;

        Ok(
            OutputStmt {
                output_tkn,
                ty,
                name,
                semi_tkn: self.expect_semi()?.ctx,
            }.into()
        )
    }

    pub fn parse_shader_input(&mut self, provide: bool) -> parser::Result<Stmt> {
        let tkn = self.expect_next(Te!(input))?.ctx;
        let mods = self.parse_mods()?;

        let ntd = self.parse_name_type_def_single(true)?;
        let name = ntd.name;
        let ty = ntd.ty.unwrap();

        let res = self.validate_mods(mods, ModsUsage::default()
            .with_rule("interpolation", ModRule::EitherOr(vec![Modifier::Flat, Modifier::Smooth, Modifier::Noperspective]))
        )?;

        let semi_tkn = self.expect_semi()?.ctx;

        if provide {
            Ok(
                ProvideStmt {
                    provide_tkn: tkn,
                    ty,
                    name,
                    mods: res,
                    semi_tkn
                }.into()
            )
        } else {
            Ok(InputStmt {
                input_tkn: tkn,
                ty,
                name,
                mods: res,
                semi_tkn
            }.into())
        }
    }

    pub fn parse_struct(&mut self) -> parser::Result<Stmt> {
        self.expect_next(Te!(struct))?;
        let name = self.expect_ident()?;
        let brack1 = self.expect_next(Te!('{'))?;

        let mut fields = vec![];
        let mut meths = vec![];

        while self.unwrap_peek()?.ty != T!('}') {
            if let T!(fn) = self.unwrap_peek()?.ty {
                let meth = self.parse_method_decl()?;
                let meth = enum_val!(Stmt, meth, MethodDecl);
                meths.push(meth);
            } else {
                let ntds = self.parse_name_type_def(true, true)?;
                let semi = self.expect_semi()?;
                for ntd in ntds {
                    if let Some(ty) = ntd.ty {
                        let field = StructField {
                            name: ntd.name,
                            colon_tkn: ntd.colon_tk.expect("needs_type is on"),
                            ty,
                            semi_tkn: semi.ctx.clone()
                        };
                        fields.push(field);
                    }
                }
            }
        }

        let brack2 = self.expect_next(Te!('}'))?;

        Ok(StructStmt {
            name,
            brace1_tkn: brack1.ctx,
            fields,
            methods: meths,
            brace2_tkn: brack2.ctx
        }.into())
    }
}

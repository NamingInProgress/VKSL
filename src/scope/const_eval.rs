use crate::{parser, symbol};
use crate::parser::err::{ParseErr, ParseErrType};
use crate::scope::ast::Ast;
use crate::scope::ast::expr::{Expr, LitExpr};
use crate::scope::ast::stmt::{ExprStmt, Stmt};
use crate::scope::ast::ty::Type;
use crate::scope::{SharedScope};
use crate::token::{Literal, Operator, TokCtx};

pub fn collapse_constants(mut ast: Ast, scope: &SharedScope) -> parser::Result<Ast> {

    Ok(ast)
}

fn handle_stmt(stmt: Stmt, scope: &SharedScope) -> parser::Result<Stmt> {
    macro_rules! optional_stmt {
        ($s:ident,$f:ident) => {
            if let Some(stmt) = $s.$f.take() {
                $s.$f = Some(Box::new(handle_stmt(*stmt, scope)?))
            }
        };
    }
    macro_rules! optional_expr {
        ($s:ident,$f:ident) => {
            if let Some(expr) = $s.$f.take() {
                $s.$f = Some(handle_non_const_expr(expr, scope)?)
            }
        };
    }
    macro_rules! optional_ty {
        ($s:ident,$f:ident,$scope:ident) => {
            if let Some(ty) = $s.$f.take() {
                $s.$f = Some(handle_type(ty, $scope)?);
            }
        };
    }
    Ok(match stmt {
        Stmt::If(mut s) => {
            s.cond = handle_non_const_expr(s.cond, scope)?;
            s.branch = Box::new(handle_stmt(*s.branch, scope)?);
            if let Some(branch) = s.else_branch {
                s.else_branch = Some(Box::new(handle_stmt(*branch, scope)?))
            }
            optional_stmt!(s, else_branch);
            Stmt::If(s)
        }
        Stmt::For(mut s) => {
            optional_expr!(s, start_cond);
            optional_expr!(s, cond);
            optional_expr!(s, after_run);
            s.block = Box::new(handle_stmt(*s.block, &s.scope)?);
            Stmt::For(s)
        }
        Stmt::While(mut s) => {
            s.cond = handle_non_const_expr(s.cond, scope)?;
            s.block = Box::new(handle_stmt(*s.block, scope)?);
            Stmt::While(s)
        }
        Stmt::MethodDecl(mut s) => {
            let sub_scope = &s.scope;
            symbol!((scope => s.symbol) as mut Function, sym => {
                for field in &sym.params {
                    symbol!((sub_scope => *field) as mut Variable, field_sym => {
                        optional_ty!(field_sym, ty, sub_scope);
                    });
                }
                optional_ty!(sym, return_type, sub_scope);
            });

            s.block = Box::new(handle_stmt(*s.block, scope)?);
            Stmt::MethodDecl(s)
        }
        Stmt::VarDecl(s) => {
            symbol!((scope => s.symbol) as mut Variable, sym => {
                optional_expr!(sym, init);
                optional_ty!(sym, ty, scope);
            });
            Stmt::VarDecl(s)
        }
        Stmt::ConstDecl(s) => {
            symbol!((scope => s.symbol) as mut Variable, sym => {
                if sym.constant.is_none() {
                optional_ty!(sym, ty, scope);
                let init = handle_const_expr(sym.init.take().expect("const init must exist"), scope)?;
                if let Expr::Literal(lit) = init {
                    sym.constant = Some(lit.lit);
                    sym.init = Some(Expr::Literal(lit));
                } else {
                    return Err(ParseErr {
                        ty: ParseErrType::ConstantNotAConstant,
                        ctx: sym.eq_tkn.take().expect("consts have a value"),
                        hint: Some("try replacing the initialization expression with a literal like `15 + (8 - 3 * 4)`".to_string()),
                    })
                }
            }
            });
            Stmt::ConstDecl(s)
        }
        Stmt::Return(mut r) => {
            optional_expr!(r, expr);
            Stmt::Return(r)
        }
        Stmt::Yield(mut y) => {
            y.expr = handle_non_const_expr(y.expr, scope)?;
            Stmt::Yield(y)
        }
        Stmt::Input(mut i) => {
            symbol!((scope => i.symbol) as mut Input, sym => {
               sym.ty = Some(handle_type(sym.ty.take().expect("shader-specific entries have a type"), scope)?);
            });

            Stmt::Input(i)
        }
        Stmt::Output(mut o) => {
            symbol!((scope => o.symbol) as mut Output, sym => {
                sym.ty = Some(handle_type(sym.ty.take().expect("shader-specific entries have a type"), scope)?);
            });

            Stmt::Output(o)
        }
        Stmt::Provide(mut p) => {
            symbol!((scope => p.symbol) as mut Provide, sym => {
                sym.ty = Some(handle_type(sym.ty.take().expect("shader-specific entries have a type"), scope)?);
            });

            Stmt::Provide(p)
        }
        Stmt::PushConstants(mut p) => {
            symbol!((scope => p.symbol) as mut PushConstant, sym => {
               sym.ty = Some(handle_type(sym.ty.take().expect("shader-specific entries have a type"), scope)?);
            });

            Stmt::PushConstants(p)
        }
        Stmt::Uniform(mut u) => {
            symbol!((scope => u.symbol) as mut Uniform, sym => {
                sym.ty = handle_type(sym.ty.clone(), scope)?;
            });

            Stmt::Uniform(u)
        }
        Stmt::Struct(mut s) => {
            let struct_scope = &s.scope;
            symbol!((scope => s.symbol) as mut Struct, sym => {
                for field in &sym.fields {
                    symbol!((struct_scope => *field) as mut Variable, field_sym => {
                        field_sym.ty = Some(handle_type(field_sym.ty.take().expect("struct fields must have type"), struct_scope)?);
                    });
                }

                let mut new_methods = Vec::with_capacity(s.methods.len());
                for mut meth_body in s.methods.drain(..) {
                    let sub_scope = &meth_body.scope;
                    symbol!((scope => meth_body.symbol) as mut Function, sym => {
                        for field in &sym.params {
                            symbol!((sub_scope => *field) as mut Variable, field_sym => {
                                optional_ty!(field_sym, ty, sub_scope);
                            });
                        }
                        optional_ty!(sym, return_type, sub_scope);

                        meth_body.block = Box::new(handle_stmt(*meth_body.block, scope)?);
                        new_methods.push(meth_body);
                    });
                }

                s.methods = new_methods;
            });

            Stmt::Struct(s)
        }
        Stmt::Block(mut b) => {
            b.stmts = handle_block(b.stmts, &b.scope)?;
            Stmt::Block(b)
        }
        Stmt::Compound(mut c) => {
            c.components = handle_block(c.components, scope)?;
            Stmt::Compound(c)
        }
        Stmt::Expr(ExprStmt { expr: Expr::Block(mut b), semi_tkn }) => {
            b.block = handle_block(b.block, &b.scope)?;
            Stmt::Expr(ExprStmt { expr: Expr::Block(b), semi_tkn })
        }
        Stmt::Expr(mut e) => {
            e.expr = handle_non_const_expr(e.expr, scope)?;
            Stmt::Expr(e)
        }
        s => s,
    })
}

fn handle_block<I: IntoIterator<Item=Stmt>>(block: I, scope: &SharedScope) -> parser::Result<Vec<Stmt>> {
    let mut vec = vec![];
    for item in block {
        vec.push(handle_stmt(item, scope)?);
    }
    Ok(vec)
}

fn handle_type(ty: Type, scope: &SharedScope) -> parser::Result<Type> {
    match ty {
        Type::Primitive(p) => Ok(Type::Primitive(p)),
        Type::ArrayOf(mut a) => {
            if let Some(dim) = a.dimension.take() {
                let value = handle_const_expr(*dim, scope)?;
                if let Expr::Literal(lit) = value {
                    if let Literal::UIntLit(i) = lit.lit {
                        a.evaluated = Some(i);
                        a.dimension = Some(Box::new(Expr::Literal(lit)));
                        Ok(Type::ArrayOf(a))
                    } else {
                        Err(ParseErr {
                            ty: ParseErrType::IllegalArrayDim,
                            ctx: a.brack1_tkn,
                            hint: Some("Try replacing the array dimension with a constant that evaluates to an unsigned integer like `12u + 10u`".to_string())
                        })
                    }
                } else {
                    Err(ParseErr {
                        ty: ParseErrType::ConstantNotAConstant,
                        ctx: a.brack1_tkn,
                        hint: Some("try replacing the array dimension expression with a literal like `51 + (12 - 5 * 3)`".to_string()),
                    })
                }
            } else {
                Ok(Type::ArrayOf(a))
            }
        }
        Type::StructRef(s) => {
            // in theory if we have a ref to a struct, we must have a def of it somewhere, so it must have already been processed
            Ok(Type::StructRef(s))
        }
        Type::StructDef(mut s) => {
            let struct_scope = &s.scope;
            symbol!((scope => s.sym) as mut Struct, sym => {
                for field in &sym.fields {
                    symbol!((struct_scope => *field) as mut Variable, field_sym => {
                        field_sym.ty = Some(handle_type(field_sym.ty.take().expect("struct fields must have type"), struct_scope)?);
                    });
                }

                let mut new_methods = Vec::with_capacity(s.methods.len());
                for mut meth_body in s.methods.drain(..) {
                    let sub_scope = &meth_body.scope;
                    symbol!((scope => meth_body.symbol) as mut Function, sym => {
                        for field in &sym.params {
                            symbol!((sub_scope => *field) as mut Variable, field_sym => {
                                if let Some(ty) = field_sym.ty.take() {
                                    field_sym.ty = Some(handle_type(ty, sub_scope)?);
                                }
                            });
                        }
                        if let Some(ty) = sym.return_type.take() {
                            sym.return_type = Some(handle_type(ty, sub_scope)?);
                        }

                        meth_body.block = Box::new(handle_stmt(*meth_body.block, scope)?);
                        new_methods.push(meth_body);
                    });
                }

                s.methods = new_methods;
            });

            Ok(Type::StructDef(s))
        }
        Type::Tuple(mut t) => {
            let mut new_types = Vec::with_capacity(t.types.len());
            for (ty, tok) in t.types.drain(..) {
                let new_ty = handle_type(ty, scope)?;
                new_types.push((new_ty, tok));
            }
            t.types = new_types;
            Ok(Type::Tuple(t))
        }
    }
}

fn handle_non_const_expr(expr: Expr, scope: &SharedScope) -> parser::Result<Expr> {
    match expr {
        Expr::Unary(mut u) => {
            u.expr = Box::new(handle_non_const_expr(*u.expr, scope)?);
            Ok(Expr::Unary(u))
        }
        Expr::Binary(mut b) => {
            b.lhs = Box::new(handle_non_const_expr(*b.lhs, scope)?);
            b.rhs = Box::new(handle_non_const_expr(*b.rhs, scope)?);
            Ok(Expr::Binary(b))
        }
        Expr::FnCall(mut f) => {
            let mut new_args = Vec::with_capacity(f.args.len());
            for (arg, tok) in f.args.drain(..) {
                let new_arg = handle_non_const_expr(arg, scope)?;
                new_args.push((new_arg, tok));
            }
            f.args = new_args;
            Ok(Expr::FnCall(f))
        }
        Expr::Access(mut a) => {
            a.parent = Box::new(handle_non_const_expr(*a.parent, scope)?);
            Ok(Expr::Access(a))
        }
        Expr::Variable(v) => {
            Ok(Expr::Variable(v))
        }
        Expr::Literal(l) => {
            Ok(Expr::Literal(l))
        }
        Expr::Index(mut i) => {
            i.array = Box::new(handle_non_const_expr(*i.array, scope)?);
            i.index = Box::new(handle_non_const_expr(*i.index, scope)?);
            Ok(Expr::Index(i))
        }
        Expr::Ternary(mut t) => {
            t.cond = Box::new(handle_non_const_expr(*t.cond, scope)?);
            t.yes = Box::new(handle_non_const_expr(*t.yes, scope)?);
            t.no = Box::new(handle_non_const_expr(*t.no, scope)?);
            Ok(Expr::Ternary(t))
        }
        Expr::PreFix(mut p) => {
            p.expr = Box::new(handle_non_const_expr(*p.expr, scope)?);
            Ok(Expr::PreFix(p))
        }
        Expr::PostFix(mut p) => {
            p.expr = Box::new(handle_non_const_expr(*p.expr, scope)?);
            Ok(Expr::PostFix(p))
        }
        Expr::Tuple(mut t) => {
            let mut new_args = Vec::with_capacity(t.args.len());
            for (arg, tok) in t.args.drain(..) {
                let new_arg = handle_non_const_expr(arg, scope)?;
                new_args.push((new_arg, tok));
            }
            t.args = new_args;
            Ok(Expr::Tuple(t))
        }
        Expr::Array(mut a) => {
            let mut new_args = Vec::with_capacity(a.args.len());
            for (arg, tok) in a.args.drain(..) {
                let new_arg = handle_non_const_expr(arg, scope)?;
                new_args.push((new_arg, tok));
            }
            a.args = new_args;
            Ok(Expr::Array(a))
        }
        Expr::Block(mut b) => {
            let mut new_stmts = Vec::with_capacity(b.block.len());
            for stmt in b.block.drain(..) {
                new_stmts.push(handle_stmt(stmt, &b.scope)?);
            }
            b.block = new_stmts;
            Ok(Expr::Block(b))
        }
        Expr::Assign(mut a) => {
            a.lhs = Box::new(handle_non_const_expr(*a.lhs, scope)?);
            a.rhs = Box::new(handle_non_const_expr(*a.rhs, scope)?);
            Ok(Expr::Assign(a))
        }
        Expr::Nonuniform(mut n) => {
            n.expr = Box::new(handle_non_const_expr(*n.expr, scope)?);
            Ok(Expr::Nonuniform(n))
        }
        Expr::TupleAccess(mut t) => {
            t.tuple = Box::new(handle_non_const_expr(*t.tuple, scope)?);
            Ok(Expr::TupleAccess(t))
        }
        Expr::AccessFnCall(mut a) => {
            a.parent = Box::new(handle_non_const_expr(*a.parent, scope)?);
            let mut new_args = Vec::with_capacity(a.args.len());
            for (arg, tok) in a.args.drain(..) {
                let new_arg = handle_non_const_expr(arg, scope)?;
                new_args.push((new_arg, tok));
            }
            a.args = new_args;
            Ok(Expr::AccessFnCall(a))
        }
        Expr::This(t) => {
            Ok(Expr::This(t))
        }
        Expr::As(mut a) => {
            a.expr = Box::new(handle_non_const_expr(*a.expr, scope)?);
            a.target_ty = handle_type(a.target_ty, scope)?;
            Ok(Expr::As(a))
        }
        Expr::Construct(mut c) => {
            let mut new_fields = Vec::with_capacity(c.fields.len());
            for (mut field, tok) in c.fields.drain(..) {
                field.expr = Box::new(handle_non_const_expr(*field.expr, scope)?);
                new_fields.push((field, tok));
            }
            c.fields = new_fields;
            Ok(Expr::Construct(c))
        }
        Expr::Constructor(mut c) => {
            let mut new_args = Vec::with_capacity(c.args.len());
            for (arg, tok) in c.args.drain(..) {
                let new_arg = handle_non_const_expr(arg, scope)?;
                new_args.push((new_arg, tok));
            }
            c.args = new_args;
            Ok(Expr::Constructor(c))
        }
    }
}

fn handle_const_expr(expr: Expr, scope: &SharedScope) -> parser::Result<Expr> {
    match expr {
        Expr::Unary(e) => {
            let literal = match handle_const_expr(*e.expr, scope)? {
                Expr::Literal(lit) => {
                    lit
                },
                Expr::Variable(var) => {
                    todo!()
                }
                other => return Ok(other),
            };

            let tkn = literal.lit_tkn.clone();
            let literal = apply_op(literal, e.op, None)?;
            Ok(Expr::Literal(LitExpr {
                lit: literal,
                lit_tkn: tkn,
                ty: None,
            }))
        }
        Expr::Binary(e) => {}
        Expr::FnCall(e) => {}
        Expr::Access(e) => {}
        Expr::Variable(e) => {}
        Expr::Literal(e) => {}
        Expr::Index(e) => {}
        Expr::Ternary(e) => {}
        Expr::PreFix(e) => {}
        Expr::PostFix(e) => {}
        Expr::Tuple(e) => {}
        Expr::Array(e) => {}
        Expr::Block(e) => {}
        Expr::Assign(e) => {}
        Expr::Nonuniform(e) => {}
        Expr::TupleAccess(e) => {}
        Expr::AccessFnCall(e) => {}
        Expr::This(e) => {}
        Expr::As(e) => {}
        Expr::Construct(e) => {}
        Expr::Constructor(e) => {}
    }
}

trait Lit {
    fn lit(&self) -> &Literal;
    fn token(&self) -> TokCtx;
    fn ty(&self) -> parser::Result<&Type>;
}

impl Lit for LitExpr {
    fn lit(&self) -> &Literal {
        &self.lit
    }

    fn token(&self) -> TokCtx {
        self.lit_tkn.clone()
    }

    fn ty(&self) -> parser::Result<&Type> {
        if let Some(ty) = &self.ty {
            Ok(ty)
        } else {
            Err(ParseErr {
                ty: ParseErrType::MissingType,
                ctx: self.token(),
                hint: None,
            })
        }
    }
}

fn apply_op(lhs: impl Lit, op: Operator, rhs: Option<impl Lit>) -> parser::Result<Literal> {
    match op {
        Operator::Assign => {}
        Operator::Plus => {}
        Operator::Minus => {}
        Operator::Mul => {}
        Operator::Div => {}
        Operator::Dot => {}
        Operator::Modulo => {}
        Operator::PlusPlus => {}
        Operator::MinusMinus => {}
        Operator::BitOr => {}
        Operator::BitAnd => {}
        Operator::BitXor => {}
        Operator::BitNegate => {}
        Operator::Greater => {}
        Operator::GreaterEq => {}
        Operator::Less => {}
        Operator::LessEq => {}
        Operator::And => {}
        Operator::Or => {}
        Operator::EqEq => {}
        Operator::Neq => {}
        Operator::Not => {}
        Operator::Lsh => {}
        Operator::Rsh => {}
        Operator::LogicalRsh => {}
        Operator::Merge => {}
    }
}
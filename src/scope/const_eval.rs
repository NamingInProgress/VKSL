use crate::parser::ast::PrimitiveType;
use crate::parser::err::{ParseErr, ParseErrType};
use crate::scope::ast::expr::{Expr, LitExpr};
use crate::scope::ast::stmt::{ExprStmt, Stmt};
use crate::scope::ast::ty::{ArrayType, CastTarget, StructRef, TupleDef, Type};
use crate::scope::ast::{Ast, Literal};
use crate::scope::{SharedScope, SwizzlePart, Symbol};
use crate::token::utils::generate_dummy_expression;
use crate::token::{Operator, TokCtx};
use crate::{parser, symbol};
use mvutils::enum_val_ref_mut;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::scope::builtins::BuiltinFnError;

pub fn collapse_constants(mut ast: Ast, scope: &SharedScope) -> parser::Result<Ast> {
    let mut new_ast = Vec::with_capacity(ast.len());
    for stmt in ast {
        new_ast.push(handle_stmt(stmt, scope)?);
    }
    Ok(new_ast)
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
                    symbol!((sub_scope => *field) as mut FnParam, field_sym => {
                        field_sym.ty = handle_type(field_sym.ty.clone(), scope)?;
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
                    sym.is_currently_being_visited = true;
                    let lit = handle_const_expr(sym.init.take().expect("const init must exist"), scope)?;
                    sym.constant = Some(lit.clone());
                    if sym.ty.as_ref().expect("all consts have type") != &lit.lit.ty() {
                        return Err(ParseErr {
                            ty: ParseErrType::TypeError(sym.ty.take().unwrap(), lit.lit.ty()),
                            hint: Some(format!("change type of the constant: `const {}: {} = {};`", sym.name.val, lit.lit.ty().as_raw(), lit.lit_tkn.get_parts().2)),
                            ctx: lit.lit_tkn,
                        })
                    }
                    sym.init = Some(Expr::Literal(lit));
                    sym.is_currently_being_visited = false;
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
        Stmt::Input(i) => {
            symbol!((scope => i.symbol) as mut Input, sym => {
               sym.ty = Some(handle_type(sym.ty.take().expect("shader-specific entries have a type"), scope)?);
            });

            Stmt::Input(i)
        }
        Stmt::Output(o) => {
            symbol!((scope => o.symbol) as mut Output, sym => {
                sym.ty = Some(handle_type(sym.ty.take().expect("shader-specific entries have a type"), scope)?);
            });

            Stmt::Output(o)
        }
        Stmt::Provide(p) => {
            symbol!((scope => p.symbol) as mut Provide, sym => {
                sym.ty = Some(handle_type(sym.ty.take().expect("shader-specific entries have a type"), scope)?);
            });

            Stmt::Provide(p)
        }
        Stmt::PushConstants(p) => {
            symbol!((scope => p.symbol) as mut PushConstant, sym => {
               sym.ty = Some(handle_type(sym.ty.take().expect("shader-specific entries have a type"), scope)?);
            });

            Stmt::PushConstants(p)
        }
        Stmt::Uniform(u) => {
            symbol!((scope => u.symbol) as mut Uniform, sym => {
                sym.ty = handle_type(sym.ty.clone(), scope)?;
            });

            Stmt::Uniform(u)
        }
        Stmt::Struct(mut s) => {
            let struct_scope = &s.scope;
            symbol!((scope => s.symbol) as mut Struct, sym => {
                if sym.resolved_fields.is_none() {
                    let mut resolved_fields = HashMap::new();
                    sym.is_currently_being_visited = true;
                    for field in &sym.fields {
                        symbol!((struct_scope => *field) as mut Field, field_sym => {
                            field_sym.ty = handle_type(field_sym.ty.clone(), scope)?;
                            resolved_fields.insert(field_sym.name.val.clone(), field_sym.ty.clone());
                        });
                    }
                    sym.is_currently_being_visited = false;
                    sym.resolved_fields = Some(resolved_fields);
                }

                let mut new_methods = Vec::with_capacity(s.methods.len());
                for mut meth_body in s.methods.drain(..) {
                    let sub_scope = &meth_body.scope;
                    symbol!((scope => meth_body.symbol) as mut Function, sym => {
                        for field in &sym.params {
                            symbol!((sub_scope => *field) as mut FnParam, field_sym => {
                                field_sym.ty = handle_type(field_sym.ty.clone(), sub_scope)?;
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
                let i = match value.lit {
                    Literal::UIntLit(u) => u,
                    Literal::IntLit(i) if i >= 0 => i as u32,
                    Literal::IntLit(i) => {
                        return Err(ParseErr {
                            ty: ParseErrType::NegativeArraySize(i),
                            ctx: value.lit_tkn,
                            hint: Some(format!("try making the array size positive: `{}[{}]`", a.component.as_raw(), -i)),
                        })
                    }
                    Literal::FloatLit(_) | Literal::DoubleLit(_) => {
                        return Err(ParseErr {
                            ty: ParseErrType::IllegalArrayDim(value.lit.ty()),
                            hint: Some(format!("try casting the size to a uint: `{}[{} as uint]`", a.component.as_raw(), value.lit_tkn.get_parts().2)),
                            ctx: value.lit_tkn,
                        })
                    }
                    _ => {
                        return Err(ParseErr {
                            ty: ParseErrType::IllegalArrayDim(value.lit.ty()),
                            ctx: value.lit_tkn,
                            hint: Some(format!("Try replacing the array dimension with a constant that evaluates to an unsigned integer like `{}[12u + 10u]`", *a.component))
                        })
                    }
                };
                a.evaluated = Some(i);
                a.evaluated_tok = Some(value.lit_tkn.clone());
                a.dimension = Some(Box::new(Expr::Literal(value)));
                Ok(Type::ArrayOf(a))
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
                if sym.resolved_fields.is_none() {
                    let mut resolved_fields = HashMap::new();
                    sym.is_currently_being_visited = true;
                    for field in &sym.fields {
                        symbol!((struct_scope => *field) as mut Field, field_sym => {
                            field_sym.ty = handle_type(field_sym.ty.clone(), scope)?;
                            resolved_fields.insert(field_sym.name.val.clone(), field_sym.ty.clone());
                        });
                    }
                    sym.is_currently_being_visited = false;
                    sym.resolved_fields = Some(resolved_fields);
                }

                let mut new_methods = Vec::with_capacity(s.methods.len());
                for mut meth_body in s.methods.drain(..) {
                    let sub_scope = &meth_body.scope;
                    symbol!((scope => meth_body.symbol) as mut Function, sym => {
                        for field in &sym.params {
                            symbol!((sub_scope => *field) as mut FnParam, field_sym => {
                                field_sym.ty = handle_type(field_sym.ty.clone(), sub_scope)?;
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
        Type::Unknown => Ok(Type::Unknown),
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
        Expr::BuiltinFnCall(mut f) => {
            let mut new_args = Vec::with_capacity(f.args.len());
            for (arg, tok) in f.args.drain(..) {
                let new_arg = handle_non_const_expr(arg, scope)?;
                new_args.push((new_arg, tok));
            }
            f.args = new_args;
            Ok(Expr::BuiltinFnCall(f))
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

fn handle_const_expr(expr: Expr, scope: &SharedScope) -> parser::Result<LitExpr> {
    match expr {
        Expr::Unary(u) => {
            let literal = handle_const_expr(*u.expr, scope)?;
            apply_un_op(literal, u.op, u.op_tkn)
        }
        Expr::Binary(b) => {
            let lhs = handle_const_expr(*b.lhs, scope)?;
            let rhs = handle_const_expr(*b.rhs, scope)?;
            apply_bin_op(lhs, b.op, b.op_tkn, rhs)
        }
        Expr::BuiltinFnCall(mut f) => {
            let mut args = Vec::with_capacity(f.args.len());
            for (arg, _) in f.args.drain(..) {
                let new_arg = handle_const_expr(arg, scope)?;
                args.push(new_arg.lit);
            }

            let result = f.function.apply(args);

            let result = match result {
                Ok(val) => val,
                Err(BuiltinFnError::NotConstable) => {
                    return Err(ParseErr {
                        ty: ParseErrType::NotConstFunction(f.function),
                        ctx: f.name_tkn,
                        hint: Some("try replacing with a constantly callable function like `sin`".to_string()),
                    })
                }
                Err(BuiltinFnError::IllegalParamCount) => {
                    return Err(ParseErr {
                        ty: ParseErrType::IllegalParamCount(f.function),
                        ctx: f.name_tkn,
                        hint: None,
                    })
                }
                Err(BuiltinFnError::IllegalParams) => {
                    return Err(ParseErr {
                        ty: ParseErrType::IllegalParams(f.function),
                        ctx: f.name_tkn,
                        hint: None,
                    })
                }
                Err(BuiltinFnError::NotImplemented) => {
                    return Err(ParseErr {
                        ty: ParseErrType::CompilerVersionAlpha,
                        ctx: f.name_tkn,
                        hint: None,
                    })
                }
            };

            Ok(LitExpr {
                ty: Some(result.ty()),
                lit: result,
                lit_tkn: TokCtx::merge(f.name_tkn, f.close_tkn),
            })
        }
        Expr::Access(a) => {
            let literal = handle_const_expr(*a.parent, scope)?;

            if let Literal::StructLit { ty, items } = literal.lit {
                let Type::StructRef(ty) = *ty else { unreachable!() };
                let Some((_, lit)) = items.into_iter().find(|(name, _)| name == &a.child.val) else {
                    return Err(ParseErr {
                        hint: Some(format!("replace .{} with a different field", a.child.val)),
                        ty: ParseErrType::NoSuchField(ty.debug_name, a.child.val),
                        ctx: a.child.tkn,
                    })
                };

                Ok(LitExpr {
                    ty: Some(lit.ty()),
                    lit,
                    lit_tkn: TokCtx::merge(literal.lit_tkn, a.child.tkn),
                })
            } else if literal.lit.is_swizzleable() {
                let swizzle_parts = a.child.val.chars().map(|c| c.swizzle_part().ok_or(c)).try_collect::<usize, Vec<_>, char>();
                let parts = match swizzle_parts {
                    Ok(parts) => parts,
                    Err(e) => {
                        let corrected = a.child.val.chars().filter(|c| c.swizzle_part().is_some()).collect::<String>();
                        return Err(ParseErr {
                            ty: ParseErrType::IllegalSwizzleComponent(e),
                            ctx: a.child.tkn,
                            hint: Some(format!("remove `{}` from swizzle: `{}.{}`", e, literal.lit_tkn.get_parts().2, corrected)),
                        });
                    }
                };
                if parts.len() > 4 {
                    let mut new_swizzle = a.child.val.clone();
                    new_swizzle.truncate(4);
                    return Err(ParseErr {
                        ty: ParseErrType::SwizzleTooLong,
                        ctx: a.child.tkn,
                        hint: Some(format!("remove trailing components from swizzle: `{}.{}`", literal.lit_tkn.get_parts().2, new_swizzle)),
                    })
                }

                let lit = match literal.lit.swizzle(parts) {
                    Ok(l) => l,
                    Err(e) => {
                        let illegal_component = a.child.val[e..=e].chars().next().expect("this just shouldn't fail");
                        let max_size = literal.lit.max_swizzle_size();
                        let corrected = a.child.val.chars().filter(|c|
                            c.swizzle_part().expect("already checked this") < max_size
                        ).collect::<String>();
                        return Err(ParseErr {
                            ty: ParseErrType::SwizzleComponentOutOfBounds(illegal_component, literal.lit.ty()),
                            ctx: a.child.tkn,
                            hint: Some(format!("remove `{}` from swizzle: `{}.{}`", illegal_component, literal.lit_tkn.get_parts().2, corrected)),
                        });
                    }
                };

                Ok(LitExpr {
                    ty: Some(lit.ty()),
                    lit,
                    lit_tkn: TokCtx::merge(literal.lit_tkn, a.child.tkn),
                })
            } else {
                return Err(ParseErr {
                    ty: ParseErrType::AccessNotAllowed(literal.lit.ty()),
                    ctx: TokCtx::merge(a.dot_tkn, a.child.tkn),
                    hint: Some(format!("try removing the access expression: `{}`", literal.lit_tkn.get_parts().2)),
                })
            }
        }

        Expr::Variable(v) => {
            let value = scope.borrow().get_symbol(v.ident.symbol);
            let value = value.get_mut();
            let Symbol::Variable(variable) = value else {
                return Err(ParseErr {
                    ty: ParseErrType::NotAConstant,
                    ctx: v.ident.tkn,
                    hint: Some(format!("try removing `{}` or replacing with the name of a constant", v.ident.val)),
                })
            };
            if variable.is_currently_being_visited {
                return Err(ParseErr {
                    ty: ParseErrType::CyclicalConstant,
                    ctx: v.ident.tkn,
                    hint: Some(format!("try removing the cyclical constant definition by removing `{}` from the expression", v.ident.val)),
                })
            }
            if !variable.cnst {
                return Err(ParseErr {
                    ty: ParseErrType::NotAConstant,
                    ctx: v.ident.tkn,
                    hint: Some(format!("try removing `{}` or replacing with the name of a constant", v.ident.val)),
                })
            }
            if variable.constant.is_none() {
                if let Some(ty) = variable.ty.take() {
                    variable.ty = Some(handle_type(ty, &variable.scope)?);
                }
                variable.is_currently_being_visited = true;
                let lit = handle_const_expr(variable.init.take().expect("const init must exist"), scope)?;
                variable.constant = Some(lit.clone());
                variable.init = Some(Expr::Literal(lit));
                variable.is_currently_being_visited = false;
            }
            let mut cnst = variable.constant.clone().expect("const should have been initialized by now");
            cnst.lit_tkn = v.ident.tkn;
            Ok(cnst)
        }
        Expr::Literal(l) => Ok(l),
        Expr::Index(i) => {
            let literal = handle_const_expr(*i.array, scope)?;

            let Literal::ArrayLit { items, len, ty } = literal.lit else {
                return Err(ParseErr {
                    ty: ParseErrType::NotAnArray,
                    ctx: literal.lit_tkn,
                    hint: Some("tuple indexing is only permitted on tuple values".to_string()),
                })
            };
            let Type::ArrayOf(ty) = *ty else { unreachable!() };

            let index_expr = handle_const_expr(*i.index, scope)?;
            let index = match index_expr.lit {
                Literal::UIntLit(u) => u,
                Literal::IntLit(i) if i >= 0 => i as u32,
                Literal::IntLit(i) => {
                    return Err(ParseErr {
                        ty: ParseErrType::NegativeArraySize(i),
                        ctx: index_expr.lit_tkn,
                        hint: Some(format!("try making the array index positive: `{}[{}]`", literal.lit_tkn.get_parts().2, -i)),
                    })
                }
                Literal::FloatLit(_) | Literal::DoubleLit(_) => {
                    return Err(ParseErr {
                        ty: ParseErrType::IllegalArrayDim(index_expr.lit.ty()),
                        hint: Some(format!("try casting the size to a uint: `{}[{} as uint]`", literal.lit_tkn.get_parts().2, index_expr.lit_tkn.get_parts().2)),
                        ctx: index_expr.lit_tkn,
                    })
                }
                l => {
                    return Err(ParseErr {
                        ty: ParseErrType::IllegalArrayDim(l.ty()),
                        ctx: index_expr.lit_tkn,
                        hint: Some(format!("try replacing with a whole positive number: `{}[10]`", literal.lit_tkn.get_parts().2)),
                    })
                }
            };


            if index >= len {
                return Err(ParseErr {
                    ty: ParseErrType::ArrayIndexOutOfBounds(1u32, len),
                    ctx: index_expr.lit_tkn,
                    hint: Some(format!("replace out of bounds literal `{}` with a number between `0` and `{}`", index, len - 1)),
                })
            }

            let item = items[index as usize].clone();
            let ty = *ty.component;

            Ok(LitExpr {
                lit: item,
                lit_tkn: TokCtx::merge(literal.lit_tkn, index_expr.lit_tkn),
                ty: Some(ty),
            })
        }
        Expr::Ternary(t) => {
            let cond = handle_const_expr(*t.cond, scope)?;
            let yes = handle_const_expr(*t.yes, scope)?;
            let no = handle_const_expr(*t.no, scope)?;

            let Literal::BoolLit(cond_val) = cond.lit else {
                return Err(ParseErr {
                    ty: ParseErrType::TypeError(PrimitiveType::Bool.into(), cond.lit.ty()),
                    ctx: cond.lit_tkn,
                    hint: Some("replace condition of ternary statement with an expression that evaluates to a boolean like `true && (false || true)`".to_string()),
                })
            };

            let lit = if cond_val { yes.lit } else { no.lit };

            Ok(LitExpr {
                ty: Some(lit.ty()),
                lit,
                lit_tkn: TokCtx::merge(cond.lit_tkn, no.lit_tkn),
            })
        }
        Expr::TupleAccess(t) => {
            let literal = handle_const_expr(*t.tuple, scope)?;

            let Literal::TupleLit { items, ty } = literal.lit else {
                return Err(ParseErr {
                    ty: ParseErrType::NotATuple,
                    ctx: literal.lit_tkn,
                    hint: Some("tuple indexing is only permitted on tuple values".to_string()),
                })
            };

            let Type::Tuple(ty) = *ty else { unreachable!() };

            if t.component as usize >= items.len() {
                return Err(ParseErr {
                    ty: ParseErrType::TupleIndexOutOfBounds(t.component, items.len() as u32),
                    ctx: t.component_tkn,
                    hint: Some(format!("replace out of bounds literal `{}` with a number between `0` and `{}`", t.component, items.len() - 1)),
                })
            }

            let item = items[t.component as usize].clone();
            let ty = ty.types[t.component as usize].0.clone();

            Ok(LitExpr {
                lit: item,
                lit_tkn: TokCtx::merge(literal.lit_tkn, t.component_tkn),
                ty: Some(ty),
            })
        }
        Expr::As(a) => {
            let literal = handle_const_expr(*a.expr, scope)?;

            let Some(ty) = a.target_ty.as_cast() else {
                return Err(ParseErr {
                    ty: ParseErrType::CannotCastToType(a.target_ty),
                    ctx: a.as_tkn,
                    hint: Some("only primitive types like `int` or `f32` may be used as cast targets".to_string()),
                })
            };

            let Type::Primitive(prim_ty) = a.target_ty.clone() else { unreachable!() };

            let lit_ty = literal.lit.ty();

            macro_rules! quick_cast {
                ($ty:ident, $val:ident, $not:literal) => {
                    match $ty {
                        CastTarget::F32 => Literal::FloatLit($val as f32),
                        CastTarget::F64 => Literal::DoubleLit($val as f64),
                        CastTarget::Int => Literal::IntLit($val as i32),
                        CastTarget::Uint => Literal::UIntLit($val as u32),
                        CastTarget::Bool => Literal::BoolLit($val != $not),
                    }
                };
            }

            let lit = match literal.lit {
                Literal::BoolLit(val) => {
                    match ty {
                        CastTarget::F32 => Literal::FloatLit(val as i32 as f32),
                        CastTarget::F64 => Literal::DoubleLit(val as i64 as f64),
                        CastTarget::Int => Literal::IntLit(val as i32),
                        CastTarget::Uint => Literal::UIntLit(val as u32),
                        CastTarget::Bool => Literal::BoolLit(val),
                    }
                }
                Literal::IntLit(val) => {
                    quick_cast!(ty, val, 0)
                }
                Literal::UIntLit(val) => {
                    quick_cast!(ty, val, 0)
                }
                Literal::FloatLit(val) => {
                    quick_cast!(ty, val, 0.0)
                }
                Literal::DoubleLit(val) => {
                    quick_cast!(ty, val, 0.0)
                }
                _ => return Err(ParseErr {
                    ty: ParseErrType::CannotCastToType(lit_ty),
                    ctx: literal.lit_tkn,
                    hint: Some("only primitive types like `int` and `f32` may be cast using `as`".to_string()),
                })
            };

            Ok(LitExpr {
                lit,
                lit_tkn: TokCtx::merge(literal.lit_tkn, prim_ty.tkn),
                ty: Some(a.target_ty),
            })
        }
        Expr::Array(a) => {
            let mut args = Vec::with_capacity(a.args.len());
            let mut ty = None;

            for (arg, _) in a.args {
                let value = handle_const_expr(arg, scope)?;

                if let Some(ty) = &ty {
                    if ty != &value.lit.ty() {
                        return Err(ParseErr {
                            ty: ParseErrType::ArrayTypeMismatch,
                            ctx: value.lit_tkn,
                            hint: Some("replace erroneous type value with a different value".to_string()),
                        })
                    }
                } else {
                    ty = Some(value.lit.ty());
                }

                args.push(value.lit);
            }

            let ty = ty.unwrap_or(Type::Unknown);
            let ty = Type::ArrayOf(ArrayType {
                component: Box::new(ty),
                brack1_tkn: TokCtx::empty(),
                dimension: None,
                evaluated: Some(args.len() as u32),
                evaluated_tok: None,
                brack2_tkn: TokCtx::empty(),
                default_size: None,
                default_size_tkn: None,
                default_kw_tkn: None,
            });

            Ok(LitExpr {
                lit: Literal::ArrayLit {
                    len: args.len() as u32,
                    items: args,
                    ty: Box::new(ty.clone()),
                },
                lit_tkn: TokCtx::merge(a.open_tkn, a.close_tkn),
                ty: Some(ty),
            })
        }
        Expr::Tuple(t) => {
            let mut args = Vec::with_capacity(t.args.len());
            let mut types = Vec::with_capacity(t.args.len());

            for (arg, tok) in t.args {
                let value = handle_const_expr(arg, scope)?;

                types.push((value.lit.ty(), tok));
                args.push(value.lit);
            }

            let ty = Type::Tuple(TupleDef {
                paren1_tok: TokCtx::empty(),
                types,
                paren2_tok: TokCtx::empty(),
            });

            Ok(LitExpr {
                lit: Literal::TupleLit {
                    items: args,
                    ty: Box::new(ty.clone()),
                },
                lit_tkn: TokCtx::merge(t.open_tkn, t.close_tkn),
                ty: Some(ty),
            })
        }
        Expr::Construct(c) => {
            let mut args = Vec::with_capacity(c.fields.len());

            let desired_struct_owned = scope.borrow().get_symbol(c.name.symbol);
            let r = desired_struct_owned.get_mut();
            let desired_struct = enum_val_ref_mut!(Symbol, r, Struct);

            if desired_struct.is_currently_being_visited {
                return Err(ParseErr {
                    ty: ParseErrType::CyclicalConstantInStruct,
                    ctx: TokCtx::merge(c.name.tkn, c.brace2_tkn),
                    hint: Some(format!("try removing the reference to this constant from the definition of `{}`, or avoid using this struct in the constant", c.name.val)),
                })
            }

            if desired_struct.resolved_fields.is_none() {
                let mut resolved_fields = HashMap::new();
                desired_struct.is_currently_being_visited = true;
                for field in &desired_struct.fields {
                    let struct_scope = &desired_struct.internal_scope;
                    symbol!((struct_scope => *field) as mut Field, field_sym => {
                            field_sym.ty = handle_type(field_sym.ty.clone(), struct_scope)?;
                            resolved_fields.insert(field_sym.name.val.clone(), field_sym.ty.clone());
                        });
                }
                desired_struct.is_currently_being_visited = false;
                desired_struct.resolved_fields = Some(resolved_fields);
            }

            let mut remaining_fields = desired_struct.resolved_fields.clone().expect("resolved struct must have resolved fields");
            let mut existing_fields = HashSet::new();
            drop(desired_struct_owned);

            for (arg, _) in c.fields {
                let value = handle_const_expr(*arg.expr, scope)?;

                if !existing_fields.insert(arg.name.val.clone()) {
                    return Err(ParseErr {
                        hint: Some(format!("remove `{}: {}`", arg.name.val, value.lit_tkn.get_parts().2)),
                        ty: ParseErrType::DuplicateField(c.name.val, arg.name.val),
                        ctx: TokCtx::merge(arg.name.tkn, value.lit_tkn),
                    })
                }

                if let Some(expected_type) = remaining_fields.remove(&arg.name.val) {
                    if expected_type != value.lit.ty() {
                        return Err(ParseErr {
                            ty: ParseErrType::TypeError(expected_type, value.lit.ty()),
                            ctx: value.lit_tkn,
                            hint: Some(format!("change type of `{}` to `{}`", arg.name.val, value.lit.ty())),
                        })
                    }
                } else {
                    return Err(ParseErr {
                        hint: Some(format!("remove `{}: {}`", arg.name.val, value.lit_tkn.get_parts().2)),
                        ty: ParseErrType::NoSuchField(c.name.val, arg.name.val),
                        ctx: TokCtx::merge(arg.name.tkn, value.lit_tkn),
                    })
                }

                args.push((arg.name.val, value.lit));
            }

            if !remaining_fields.is_empty() {
                let remaining = remaining_fields.into_keys().collect::<Vec<_>>();
                return Err(ParseErr {
                    ty: ParseErrType::MissingFields(c.name.val, remaining),
                    ctx: c.brace2_tkn,
                    hint: Some("try adding the missing fields to constructor".to_string()),
                })
            }

            let ty = Type::StructRef(StructRef {
                sym: c.name.symbol,
                name_tkn: c.name.tkn,
                debug_name: c.name.val,
            });

            Ok(LitExpr {
                lit: Literal::StructLit {
                    items: args,
                    ty: Box::new(ty.clone()),
                },
                lit_tkn: TokCtx::merge(c.brace1_tkn, c.brace2_tkn),
                ty: Some(ty),
            })
        }

        Expr::This(t) => {
            Err(ParseErr {
                ty: ParseErrType::ThisInConst,
                ctx: t.this_tkn,
                hint: None,
            })
        }
        Expr::Block(b) => {
            Err(ParseErr {
                ty: ParseErrType::BlockInConst,
                ctx: TokCtx::merge(b.open_tkn, b.close_tkn),
                hint: None,
            })
        }
        Expr::FnCall(f) => {
            Err(ParseErr {
                ty: ParseErrType::FunctionInConst,
                ctx: TokCtx::merge(f.ident.tkn, f.close_tkn),
                hint: None,
            })
        }
        Expr::AccessFnCall(a) => {
            Err(ParseErr {
                ty: ParseErrType::FunctionInConst,
                ctx: TokCtx::merge(a.ident.tkn, a.close_tkn),
                hint: None,
            })
        }
        Expr::Constructor(c) => {
            Err(ParseErr {
                ty: ParseErrType::FunctionInConst,
                ctx: TokCtx::merge(c.ident.tkn, c.close_tkn),
                hint: None,
            })
        }
        Expr::PreFix(p) => {
            Err(ParseErr {
                ty: ParseErrType::ModificationInConst,
                ctx: p.op_tkn,
                hint: None,
            })
        }
        Expr::PostFix(p) => {
            Err(ParseErr {
                ty: ParseErrType::ModificationInConst,
                ctx: p.op_tkn,
                hint: None,
            })
        }
        Expr::Assign(a) => {
            Err(ParseErr {
                ty: ParseErrType::ModificationInConst,
                ctx: a.eq_tkn,
                hint: None,
            })
        }
        Expr::Nonuniform(n) => {
            Err(ParseErr {
                ty: ParseErrType::NotAllowed,
                ctx: n.nonuniform_tkn,
                hint: None,
            })
        }
    }
}

// temporarily out of service
fn extract_const(value: Expr, token: TokCtx) -> parser::Result<LitExpr> {
    if let Expr::Literal(lit) = value {
        Ok(lit)
    } else {
        let example = generate_dummy_expression();
        Err(ParseErr {
            ty: ParseErrType::ConstantNotAConstant,
            ctx: token,
            hint: Some(format!("try replacing the initialization expression with a literal like `{example}`")),
        })
    }
}
fn apply_bin_op(lhs: LitExpr, op: Operator, op_tok: TokCtx, rhs: LitExpr) -> parser::Result<LitExpr> {
    todo!()
}


fn apply_un_op(lhs: LitExpr, op: Operator, ctx: TokCtx) -> parser::Result<LitExpr> {
    macro_rules! unary_op {
        (
            ($opname:literal, $op:literal),
            ($($t1:ident),*),
            ($($t2:ident),*),
            ($($t3:ident),*),
            $name:ident => $expr:expr
        ) => {
            match lhs.lit {
                $(
                Literal::$t1($name) => Ok(Literal::$t1($expr))
                ),*,
                $(
                Literal::$t2(a) => Ok(Literal::$t2(a.map(|$name| $expr)))
                ),*,
                $(
                Literal::$t3(a) => Ok(Literal::$t3(a.map(|b| b.map(|$name| $expr)))),
                )*
                l => {
                    let (_, mut hist, data, _) = lhs.lit_tkn.get_parts();
                    let future = lhs.lit_tkn.get_future_line();
                    hist.pop();
                    Err(ParseErr {
                        ty: ParseErrType::IllegalUnaryType(l.ty(), $opname),
                        ctx: lhs.lit_tkn.clone(),
                        hint: Some(format!("Try removing the {} operator: `{hist}{data}{future}`", $op)),
                    })
                }
            }
        };
    }

    match op {
        Operator::Minus => {
            let lit = unary_op!(
                ("minus", "-"),
                (IntLit, FloatLit, DoubleLit),
                (Vec2, IVec2, DVec2, Vec3, IVec3, DVec3, Vec4, IVec4, DVec4),
                (Mat2, Mat3, Mat4, Mat2x3, Mat2x4, Mat3x2, Mat3x4, Mat4x2, Mat4x3, DMat2, DMat3, DMat4, DMat2x3, DMat2x4, DMat3x2, DMat3x4, DMat4x2, DMat4x3),
                i => -i
            )?;

            Ok(LitExpr {
                ty: Some(lit.ty()),
                lit,
                lit_tkn: TokCtx::merge(ctx, lhs.lit_tkn),
            })
        }
        Operator::BitNegate => {
            let lit = unary_op!(("bitwise not", "~"), (IntLit, UIntLit), (IVec2, UVec2, IVec3, UVec3, IVec4, UVec4), (), i => i.reverse_bits())?;

            Ok(LitExpr {
                ty: Some(lit.ty()),
                lit,
                lit_tkn: TokCtx::merge(ctx, lhs.lit_tkn),
            })
        }
        Operator::Not => {
            match lhs.lit {
                Literal::BoolLit(b) => {
                    let l = Literal::BoolLit(!b);
                    Ok(LitExpr {
                        ty: Some(l.ty()),
                        lit: l,
                        lit_tkn: TokCtx::merge(ctx, lhs.lit_tkn),
                    })
                }
                l => {
                    let (_, mut hist, data, _) = lhs.lit_tkn.get_parts();
                    let future = lhs.lit_tkn.get_future_line();
                    hist.pop();
                    Err(ParseErr {
                        ty: ParseErrType::IllegalUnaryType(l.ty(), "boolean not"),
                        ctx: lhs.lit_tkn,
                        hint: Some(format!("Try removing the ! operator: `{hist}{data}{future}`")),
                    })
                }
            }
        }
        op => unreachable!("parser generated unary expr with illegal operator '{}'", op)
    }
}
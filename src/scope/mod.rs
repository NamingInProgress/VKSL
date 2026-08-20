pub mod old;
pub mod ast;
pub mod name_res;
pub mod struct_res;

use crate::parser;
use crate::parser::ast::UniformType;
use crate::parser::err::{ParseErr, ParseErrType};
use crate::parser::mods::ResMods;
use crate::scope::ast::expr::Expr;
use crate::scope::ast::ty::Type;
use crate::scope::ast::Ident;
use crate::token::TokCtx;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use itertools::Itertools;

pub type SharedScope = Rc<RefCell<ResolvedScope>>;
pub type SharedSymbol = Rc<RefCell<Symbol>>;
pub type SymbolTable = Rc<RefCell<HashMap<SymbolId, SharedSymbol>>>;

const COUNTER_ID: &str = "SymbolId";

#[derive(Clone)]
pub struct ResolvedScope {
    parent: Option<SharedScope>,
    pub symbols: SymbolTable,
    by_name: HashMap<String, SymbolId>,
}

impl ResolvedScope {
    pub fn global() -> SharedScope {
        Rc::new(RefCell::new(Self {
            parent: None,
            symbols: Rc::new(RefCell::new(HashMap::new())),
            by_name: HashMap::new(),
        }))
    }

    pub fn with_parent(parent: SharedScope) -> SharedScope {
        let symbols = parent.borrow().symbols.clone();
        Rc::new(RefCell::new(Self {
            parent: Some(parent),
            symbols,
            by_name: HashMap::new(),
        }))
    }
}

impl Debug for ResolvedScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("<scope>")
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SymbolSpecies {
    Ident,
    Type,
    Fn
}

impl ResolvedScope {
    pub fn insert_symbol(&self, id: SymbolId, symbol: SharedSymbol) {
        let mut guard = self.symbols.borrow_mut();
        guard.insert(id, symbol);
    }

    fn collect_all(&self, species: SymbolSpecies) -> Vec<String> {
        let base = self.by_name.iter()
            .filter_map(|(k, v)| {
                let sym = self.get_symbol(*v);
                match &*sym.borrow() {
                    Symbol::Variable(_) if species == SymbolSpecies::Ident => Some(k.clone()),
                    Symbol::Function(_) if species == SymbolSpecies::Fn => Some(k.clone()),
                    Symbol::Struct(_) if species == SymbolSpecies::Type => Some(k.clone()),
                    Symbol::Field(_) if species == SymbolSpecies::Ident => Some(k.clone()),
                    Symbol::Uniform(_) if species == SymbolSpecies::Ident => Some(k.clone()),
                    Symbol::PushConstant(_) if species == SymbolSpecies::Ident => Some(k.clone()),
                    Symbol::Input(_) if species == SymbolSpecies::Ident => Some(k.clone()),
                    Symbol::Output(_) if species == SymbolSpecies::Ident => Some(k.clone()),
                    Symbol::Provide(_) if species == SymbolSpecies::Ident => Some(k.clone()),
                    _ => None
                }
            });
        if let Some(parent) = &self.parent {
            let guard = parent.borrow();
            base.chain(guard.collect_all(species)).collect_vec()
        } else {
            base.collect_vec()
        }
    }

    fn get_score(target: &str, test: &str) -> u32 {
        let mut score = 0;
        let mut c1i = target.chars();
        let mut c2i = test.chars();
        while let Some(c1) = c1i.next() && let Some(c2) = c2i.next() {
            score += (c1 as i32 - c2 as i32).abs() as u32;
        }

        while let Some(_) = c1i.next() {
            score += 26;
        }
        while let Some(_) = c2i.next() {
            score += 26;
        }

        score
    }

    fn find_closest_match(&self, name: &str, species: SymbolSpecies) -> Option<String> {
        self.collect_all(species)
            .into_iter()
            .min_by_key(|s| Self::get_score(name, s))
    }

    pub fn resolve_symbol_id(&self, name: &impl SymbolName, species: SymbolSpecies) -> parser::Result<SymbolId> {
        let s = name.get_name();
        match self.by_name.get(s) {
            None => {
                if let Some(parent) = &self.parent {
                    let parent_guard = parent.borrow();
                    let id = parent_guard.resolve_symbol_id(name, species)?;
                    Ok(id)
                } else {
                    let mut hint = self.find_closest_match(s, species);

                    let err_kind = if species == SymbolSpecies::Type {
                        ParseErrType::UnknownType(s.clone())
                    } else {
                        ParseErrType::UnknownIdent(s.clone())
                    };

                    hint = hint.map(|m| format!("did you perhaps mean `{m}`?"));

                    Err(ParseErr {
                        ty: err_kind,
                        ctx: name.get_error_token(),
                        tail: "".to_string(),
                        hint,
                    })
                }
            }
            Some(id) => {
                Ok(*id)
            }
        }
    }

    pub fn resolve_symbol(&self, name: &impl SymbolName, species: SymbolSpecies) -> parser::Result<SharedSymbol> {
        let id = self.resolve_symbol_id(name, species)?;
        Ok(self.get_symbol(id))
    }

    pub fn get_symbol(&self, id: SymbolId) -> SharedSymbol {
        let guard = self.symbols.borrow();
        guard.get(&id).expect("Critical parser bug").clone()
    }
}

pub type SymbolId = u64;

#[derive(Clone, Debug)]
pub enum Symbol {
    Variable(VarSym),
    Function(FnSym),
    FnParam(FnParamSym),
    Struct(StructSym),
    Field(FieldSym),
    Uniform(UniformSym),
    PushConstant(VarSym),
    Input(VarSym),
    Output(VarSym),
    Provide(VarSym),
}

#[derive(Clone, Debug)]
pub struct VarSym {
    pub id: SymbolId,
    pub kw_tkn: TokCtx, //None when method param
    pub name: Ident,
    pub colon_tkn: Option<TokCtx>,
    pub ty: Option<Type>,
    pub eq_tkn: Option<TokCtx>,
    pub init: Option<Expr>,
    pub mods: ResMods,
    pub semi_tkn: TokCtx,
    pub cnst: bool,
}

#[derive(Clone, Debug)]
pub struct FnSym {
    pub id: SymbolId,
    pub fn_tkn: TokCtx,
    pub name: Ident,
    pub l_paren: TokCtx,
    pub params: Vec<SymbolId>,
    pub r_paren: TokCtx,
    pub arrow_tkn: Option<TokCtx>,
    pub return_type: Option<Type>,
    pub scope: SharedScope
}

#[derive(Clone, Debug)]
pub struct FnParamSym {
    pub id: SymbolId,
    pub name: Ident,
    pub colon_tkn: TokCtx,
    pub ty: Type,
    pub comma_tkn: Option<TokCtx>
}

#[derive(Clone, Debug)]
pub struct StructSym {
    pub id: SymbolId,
    pub internal_scope: SharedScope,
    pub name: Ident,
    pub brace1_tkn: TokCtx,
    pub fields: Vec<SymbolId>,
    pub methods: Vec<SymbolId>,
    pub brace2_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct UniformSym {
    pub uniform_tkn: TokCtx,
    pub name: Ident,
    pub ty: Type,

    pub set_tkn: TokCtx,
    pub set_eq_tkn: TokCtx,
    pub set_lit_tkn: TokCtx,
    pub set: u32,

    pub binding_tkn: TokCtx,
    pub binding_eq_tkn: TokCtx,
    pub binding_lit_tkn: TokCtx,
    pub binding: u32,
    pub mods: ResMods,
    pub uniform_type: UniformType,
    pub semi_tkn: TokCtx,
}

#[derive(Clone, Debug)]
pub struct FieldSym {
    pub name: Ident,
    pub colon_tkn: TokCtx,
    pub ty: Type,
    pub semi_tkn: TokCtx
}

pub trait SymbolName {
    fn get_name(&self) -> &String;
    fn get_error_token(&self) -> TokCtx;
}
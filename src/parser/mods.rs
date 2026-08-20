use crate::parser;
use crate::parser::err::{ParseErr, ParseErrType};
use crate::parser::Parser;
use crate::token::{TokCtx, TokenType};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::mem;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Modifier {
    Flat,
    Smooth,
    Noperspective,
    Enable,
    Require,
    Warn,
    Disable,
    Readonly,
    Writeonly,
    STD140,
    STD430
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Modifier::Flat => "flat",
            Modifier::Smooth => "smooth",
            Modifier::Noperspective => "noperspective",
            Modifier::Enable => "enable",
            Modifier::Require => "require",
            Modifier::Warn => "warn",
            Modifier::Disable => "disable",
            Modifier::Readonly => "readonly",
            Modifier::Writeonly => "writeonly",
            Modifier::STD140 => "std140",
            Modifier::STD430 => "std430",
        }.to_string();
        write!(f, "{}", str)
    }
}

#[derive(Clone, Debug)]
pub struct Mods {
    mods: HashMap<Modifier, TokCtx>
}

#[derive(Clone, Debug)]
pub struct ResMods {
    found: HashMap<RuleKey, (Modifier, Option<TokCtx>)>,
}

impl ResMods {
    pub fn empty() -> Self {
        Self {
            found: HashMap::default(),
        }
    }

    pub fn test(&self, key: RuleKey) -> Option<Modifier> {
        self.found.get(key).map(|x| x.0).clone()
    }

    pub fn test_or(&self, key: RuleKey, default: Modifier) -> Modifier {
        self.test(key).unwrap_or(default)
    }

    pub fn or_default(&mut self, key: RuleKey, default: Modifier) {
        self.found.entry(key).or_insert((default, None));
    }
}

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn parse_mods(&mut self) -> parser::Result<Mods> {
        let mut hashset = HashMap::new();

        while let Some(token) = self.peek_token()? {
            if let TokenType::Keyword(kw) = token.ty {
                if kw as u8 <= Modifier::STD430 as u8 {
                    self.unwrap_next()?;
                    let m = unsafe { mem::transmute(kw) };
                    hashset.insert(m, token.ctx);
                    continue
                }
            }
            break;
        }

        Ok(Mods {
            mods: hashset,
        })
    }

    pub fn validate_mods(&mut self, mut mods: Mods, usage: ModsUsage) -> parser::Result<ResMods> {
        let mut found = HashMap::new();

        for (rule_key, rule) in usage.rules {
            match rule {
                ModRule::Either(options) => {
                    let mut success = false;
                    for option in &options {
                        if let Some(token) = mods.mods.remove(option) {
                            if success {
                                let options_str = options.iter()
                                    .map(|s| format!("`{s}`"))
                                    .join(", ");

                                let ctx = token.clone();
                                let e = ParseErr {
                                    ty: ParseErrType::IllegalModifier(*option),
                                    ctx,
                                    hint: Some(format!("only one of {options_str} is permitted. Consider removing `{option}`")),
                                };

                                return Err(e);
                            }

                            found.insert(rule_key, (*option, Some(token)));
                            success = true;
                        }
                    }

                    if !success {
                        let options_str = options.iter()
                            .map(|s| format!("`{s}`"))
                            .join(", ");

                        let recommendation = options.first()
                            .copied()
                            .unwrap_or(Modifier::Enable)
                            .to_string();

                        let e = ParseErr {
                            ty: ParseErrType::MissingModifier,
                            ctx: self.tokens.create_context(),
                            hint: Some(format!("one of {options_str} is required. Consider adding `{recommendation}`")),
                        };

                        return Err(e);
                    }
                }
                ModRule::OnceOr(m) => {
                    if let Some(token) = mods.mods.get(&m) {
                        found.insert(rule_key, (m, Some(token.clone())));
                    }
                }
                ModRule::EitherOr(options) => {
                    let mut success = false;
                    for option in &options {
                        if let Some(token) = mods.mods.remove(option) {
                            if success {
                                let options_str = options.iter()
                                    .map(|s| s.to_string())
                                    .join(", ");

                                let e = ParseErr {
                                    ty: ParseErrType::IllegalModifier(*option),
                                    ctx: token,
                                    hint: Some(format!("only one of {options_str} is permitted. Consider removing `{option}`")),
                                };

                                return Err(e);
                            }

                            found.insert(rule_key, (*option, Some(token)));
                            success = true;
                        }
                    }
                }
            }
        }

        if let Some((unhandled_mod, token)) = mods.mods.into_iter().next() {
            return Err(ParseErr {
                ty: ParseErrType::IllegalModifier(unhandled_mod),
                ctx: token,
                hint: Some(format!("`{unhandled_mod}` is not a valid modifier in this context")),
            });
        }

        Ok(ResMods {
            found,
        })
    }
}

pub enum ModRule {
    Either(Vec<Modifier>),
    OnceOr(Modifier),
    EitherOr(Vec<Modifier>)
}

pub type RuleKey = &'static str;

#[derive(Default)]
pub struct ModsUsage {
    rules: Vec<(RuleKey, ModRule)>
}

impl ModsUsage {
    pub fn with_rule(mut self, key: RuleKey, rule: ModRule) -> Self {
        self.rules.push((key, rule));
        self
    }
}
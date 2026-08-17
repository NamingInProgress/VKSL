use crate::ast::ty::Type::SingleType;

#[derive(Clone, Debug)]
pub enum Type {
    SingleType(String),
    MultiType(Vec<String>),
    ArrayOf(Box<Type>)
}

impl Type {
    pub fn from_str(s: String) -> Self {
        SingleType(s)
    }
}

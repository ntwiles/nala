use crate::errors::*;

use super::*;

#[derive(Debug, Clone)]
pub enum SymbolOrTerm {
    Symbol(String),
    Term(Term),
}

#[derive(Debug, Clone)]
pub enum Term {
    Array(Vec<Term>),
    Bool(bool),
    Func(Box<Params>, Box<Block>),
    Kind(String),
    Num(f32),
    String(String),
    Type(TypeVariant),

    Break(Box<Expr>),
    Exception(NalaRuntimeError),
    Void,
}

impl Term {
    pub fn to_string(&self) -> String {
        match self {
            Term::Array(a) => String::from(format!("Array[{}]", a.len())),
            Term::String(t) => t.to_owned(),
            Term::Num(n) => n.to_string(),
            Term::Bool(b) => b.to_string(),
            Term::Func(_, _) => String::from(format!("[{}]", self.get_type().to_string())),
            Term::Void => String::from("<Void>"),
            Term::Break(_) => String::from("<Break>"),
            Term::Type(type_kind) => type_kind.to_string(),
            Term::Kind(k) => k.to_owned(),
            Term::Exception(e) => e.message.clone(),
        }
    }

    pub fn get_type(&self) -> TypeVariant {
        match self {
            Term::Array(items) => {
                let elem_type = if items.len() > 0 {
                    items.first().unwrap().get_type()
                } else {
                    // TODO: We need to get rid of the Unknown primitive type and solve this problem another way.
                    TypeVariant::Primitive(PrimitiveType::Unknown)
                };

                let elem_type = Types::Type(elem_type);
                TypeVariant::Nested(PrimitiveType::Array, Box::new(elem_type))
            }
            Term::Func(params, _) => {
                let params = params.to_vec();
                if params.len() > 0 {
                    let param_types: Vec<TypeVariant> =
                        params.iter().map(|p| p.clone().param_type).collect();
                    let param_types = Types::from_vec(param_types);
                    TypeVariant::Nested(PrimitiveType::Func, Box::new(param_types))
                } else {
                    TypeVariant::Primitive(PrimitiveType::Func)
                }
            }
            Term::Bool(_) => TypeVariant::Primitive(PrimitiveType::Bool),
            Term::Break(_) => TypeVariant::Primitive(PrimitiveType::Break),
            Term::Num(_) => TypeVariant::Primitive(PrimitiveType::Number),
            Term::String(_) => TypeVariant::Primitive(PrimitiveType::String),
            Term::Void => TypeVariant::Primitive(PrimitiveType::Void),
            Term::Type(_) => TypeVariant::Primitive(PrimitiveType::Enum),
            Term::Kind(_) => TypeVariant::Primitive(PrimitiveType::Kind),
            Term::Exception(_) => TypeVariant::Primitive(PrimitiveType::Exception),
        }
    }
}

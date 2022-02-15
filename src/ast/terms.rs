use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use super::*;

#[derive(Debug, Clone)]
pub enum SymbolOrTerm {
    Symbol(String),
    Term(Term),
}

#[derive(Debug, Clone)]
pub enum Term {
    Array(Arc<Mutex<Vec<Term>>>),
    Bool(bool),
    Func(Box<Params>, Box<Block>),
    Variant(String),
    Num(f32),
    Object(Arc<Mutex<HashMap<String, Term>>>),
    String(String),
    Type(TypeVariant),

    Break(Box<Expr>),
    Void,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Array(a) => {
                let a = Arc::clone(a);
                let a = a.lock().unwrap();
                write!(f, "<Array[{}]>", a.len())
            }
            Term::String(t) => write!(f, "{}", t),
            Term::Num(n) => write!(f, "{}", n),
            Term::Bool(b) => write!(f, "{}", b),
            Term::Func(_, _) => write!(f, "[{}]", self.get_type()),
            Term::Void => write!(f, "<Void>"),
            Term::Break(_) => write!(f, "<Break>"),
            Term::Type(type_kind) => write!(f, "{}", type_kind),
            Term::Variant(k) => write!(f, "{}", k),
            Term::Object(_) => write!(f, "<Object>"),
        }
    }
}

impl Term {
    pub fn get_type(&self) -> TypeVariant {
        match self {
            Term::Array(items) => {
                let items = Arc::clone(items);
                let items = items.lock().unwrap();
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
            Term::Variant(_) => TypeVariant::Primitive(PrimitiveType::Variant),
            Term::Object(_) => TypeVariant::Primitive(PrimitiveType::Object),
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, right: &Term) -> bool {
        match self {
            Term::Num(left) => {
                if let Term::Num(right) = right {
                    left == right
                } else {
                    false
                }
            }
            _ => todo!(),
        }
    }

    fn ne(&self, _right: &Term) -> bool {
        todo!()
    }
}

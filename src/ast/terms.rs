use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use crate::errors::NalaRuntimeError;

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
    Func(Option<Params>, Box<Block>),
    Variant(String, String, Option<Box<Term>>),
    Num(f32),
    Object(Arc<Mutex<HashMap<String, Term>>>),
    Pattern(Pattern),
    String(String),
    Type(TypeVariant),

    Break(Box<Expr>),
    Void,
}

impl Term {
    pub fn unwrap_variant(&self) -> Result<(String, String, Option<Box<Term>>), NalaRuntimeError> {
        if let Term::Variant(enum_name, variant_name, data) = self {
            Ok((enum_name.to_owned(), variant_name.to_owned(), data.clone()))
        } else {
            todo!()
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Array(a) => {
                let a = Arc::clone(a);
                let a = a.lock().unwrap();
                write!(f, "<Array[{}]>", a.len())
            }
            Term::Bool(b) => write!(f, "{}", b),
            Term::Break(_) => write!(f, "<Break>"),
            Term::Func(_, _) => write!(f, "[{}]", self.get_type()),
            Term::Num(n) => write!(f, "{}", n),
            Term::Object(_) => write!(f, "<Object>"),
            Term::Pattern(_) => write!(f, "<Pattern>"),
            Term::String(t) => write!(f, "{}", t),
            Term::Type(type_kind) => write!(f, "{}", type_kind),
            Term::Variant(e, v, d) => {
                let d = if let Some(d) = d {
                    format!("({0})", d)
                } else {
                    "".to_string()
                };

                write!(f, "{0}::{1}{2}", e, v, d)
            }
            Term::Void => write!(f, "<Void>"),
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
                    TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Unknown))
                };

                let elem_type = TypeVariants::TypeVariant(elem_type);
                TypeVariant::Nested(
                    Type::PrimitiveType(PrimitiveType::Array),
                    Box::new(elem_type),
                )
            }
            Term::Bool(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Bool)),
            Term::Break(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Break)),
            Term::Func(params, _) => {
                let params = if let Some(params) = params {
                    params.to_vec()
                } else {
                    vec![]
                };

                if params.len() > 0 {
                    let param_types: Vec<TypeVariant> =
                        params.iter().map(|p| p.clone().param_type).collect();
                    let param_types = TypeVariants::from_vec(param_types);
                    TypeVariant::Nested(
                        Type::PrimitiveType(PrimitiveType::Func),
                        Box::new(param_types),
                    )
                } else {
                    TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Func))
                }
            }

            Term::Num(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
            Term::Object(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Object)),
            Term::Pattern(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Pattern)),
            Term::String(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::String)),
            Term::Type(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Enum)),
            Term::Variant(enum_name, _, _) => {
                TypeVariant::Type(Type::UserDefined(enum_name.to_owned()))
            }
            Term::Void => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Void)),
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
            Term::String(left) => {
                if let Term::String(right) = right {
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

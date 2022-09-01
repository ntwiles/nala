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
    Identifier(String),
    Value(Value),
    Index(Index),
}

#[derive(Debug, Clone)]
pub enum Value {
    Array(Arc<Mutex<Vec<Value>>>),
    Bool(bool),
    Func(Vec<Param>, Box<Block>),
    Variant(String, String, Option<Box<Value>>),
    Num(f32),
    Object(Arc<Mutex<HashMap<String, Value>>>),
    String(String),
    Type(TypeVariant),

    Break(Box<Expr>),
    Void,
}

impl Value {
    pub fn unwrap_string(&self) -> String {
        if let Value::String(string) = self {
            string.to_owned()
        } else {
            panic!("Term `{}` is not a String!", self);
        }
    }

    pub fn unwrap_variant(&self) -> (String, String, Option<Box<Value>>) {
        if let Value::Variant(enum_name, variant_name, data) = self {
            (enum_name.to_owned(), variant_name.to_owned(), data.clone())
        } else {
            panic!("Term `{}` is not a Variant!", self);
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Array(a) => {
                let a = Arc::clone(a);
                let a = a.lock().unwrap();
                write!(f, "<Array[{}]>", a.len())
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::Break(_) => write!(f, "<Break>"),
            Value::Func(params, _) => {
                let params = params
                    .to_vec()
                    .iter()
                    .map(|p| p.param_type.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                write!(f, "Func<{}>", params)
            }
            Value::Num(n) => write!(f, "{}", n),
            // TODO: Do we really want to just print <Object> here?
            Value::Object(_) => write!(f, "<Object>"),
            Value::String(t) => write!(f, "{}", t),
            Value::Type(type_kind) => write!(f, "{}", type_kind),
            Value::Variant(e, v, d) => {
                let d = if let Some(d) = d {
                    format!("({0})", d)
                } else {
                    "".to_string()
                };

                write!(f, "{0}::{1}{2}", e, v, d)
            }
            Value::Void => write!(f, "<Void>"),
        }
    }
}

impl Value {
    pub fn get_type(&self) -> TypeVariant {
        match self {
            Value::Array(items) => {
                let items = Arc::clone(items);
                let items = items.lock().unwrap();
                let elem_type = if items.len() > 0 {
                    items.first().unwrap().get_type()
                } else {
                    todo!("Handle the case where trying to get the type of an empty array.")
                };

                TypeVariant::Nested(Type::PrimitiveType(PrimitiveType::Array), vec![elem_type])
            }
            Value::Bool(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Bool)),
            Value::Break(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Break)),
            Value::Func(params, _) => {
                if params.len() > 0 {
                    let param_types: Vec<TypeVariant> =
                        params.iter().map(|p| p.clone().param_type).collect();
                    TypeVariant::Nested(Type::PrimitiveType(PrimitiveType::Func), param_types)
                } else {
                    TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Func))
                }
            }
            Value::Type(_) => todo!("What is this?"),
            Value::Num(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
            Value::Object(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Object)),
            Value::String(_) => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::String)),
            Value::Variant(enum_name, _, _) => {
                TypeVariant::Type(Type::UserDefined(enum_name.to_owned()))
            }
            Value::Void => TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Void)),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, right: &Value) -> bool {
        match self {
            Value::Num(left) => {
                if let Value::Num(right) = right {
                    left == right
                } else {
                    false
                }
            }
            Value::String(left) => {
                if let Value::String(right) = right {
                    left == right
                } else {
                    false
                }
            }
            Value::Bool(left) => {
                if let Value::Bool(right) = right {
                    left == right
                } else {
                    false
                }
            }
            _ => todo!(),
        }
    }

    fn ne(&self, _right: &Value) -> bool {
        todo!()
    }
}

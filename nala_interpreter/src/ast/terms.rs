use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use super::*;

use crate::types::type_variant::TypeVariant;

#[derive(Debug, Clone)]
pub enum SymbolOrTerm {
    Symbol(String),
    Term(Term),
}

#[derive(Debug, Clone)]
pub enum Term {
    Identifier(String),
    Value(Value),
}

// TODO: This should not be part of the AST because it user TypeVariant instead of TypeLiteralVariant.
#[derive(Debug, Clone)]
pub struct FuncValue {
    pub block: Box<FuncVariant>,
    pub params: Vec<Param>,
    pub return_type: TypeVariant,
    pub type_params: Option<String>,
    pub closure_scope: usize,
}

// TODO: This should not be part of the AST because it user TypeVariant instead of TypeLiteralVariant.
#[derive(Debug, Clone)]
pub struct Param {
    pub ident: String,
    pub param_type: TypeVariant,
}

#[derive(Debug, Clone)]
pub struct EnumVariantValue {
    pub enum_ident: String,
    pub variant_ident: String,
    pub data: Option<Box<Value>>,
}

#[derive(Clone)]
pub enum Value {
    Array(Arc<Mutex<Vec<Value>>>),
    Bool(bool),
    Func(FuncValue),
    Variant(EnumVariantValue),
    Num(f32),
    Object(Arc<Mutex<HashMap<String, Value>>>),
    String(String),
    Break(Box<Value>),
    Void,
}

impl Value {
    pub fn is_bool(&self) -> bool {
        if let Value::Bool(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_number(&self) -> bool {
        if let Value::Num(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let Value::String(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_void(&self) -> bool {
        if let Value::Void = self {
            true
        } else {
            false
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let Value::String(string) = self {
            Some(string.to_owned())
        } else {
            None
        }
    }

    pub fn as_variant(&self) -> Option<EnumVariantValue> {
        if let Value::Variant(variant) = self {
            Some(variant.clone())
        } else {
            None
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Array(a) => {
                let items = a
                    .clone()
                    .lock()
                    .unwrap()
                    .iter()
                    .fold(String::new(), |acc, curr| format!("{acc}{0:?}, ", &curr));

                write!(f, "[{items}]")
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::Func(FuncValue {
                params,
                return_type,
                ..
            }) => {
                let mut params: Vec<String> = params
                    .to_vec()
                    .iter()
                    .map(|p| p.param_type.to_string())
                    .collect();

                params.push(return_type.to_string());

                write!(f, "Func<{}>", params.join(", "))
            }
            Value::Num(n) => write!(f, "{}", n),
            Value::Object(fields) => {
                write!(f, "{{ ")?;

                write!(
                    f,
                    "{}",
                    fields
                        .lock()
                        .unwrap()
                        .iter()
                        .map(|(key, value)| format!("{}: {:?}", key, value))
                        .fold(String::new(), |a, b| a + &b + ", ")
                )?;

                write!(f, "}}")
            }
            Value::String(s) => write!(f, "'{}'", s),
            Value::Variant(EnumVariantValue {
                variant_ident,
                data,
                ..
            }) => {
                let data = if let Some(data) = data {
                    format!("({0:?})", data)
                } else {
                    "".to_string()
                };

                write!(f, "{variant_ident}{data}")
            }
            Value::Void => write!(f, "<Void>"),
            value => todo!("Implement Debug for Value {value}"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Array(a) => {
                let items = a
                    .clone()
                    .lock()
                    .unwrap()
                    .iter()
                    .fold(String::new(), |acc, curr| format!("{acc}{0:?}, ", &curr));

                write!(f, "[{items}]")
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::Break(_) => write!(f, "<Break>"),
            Value::Func(FuncValue {
                params,
                return_type,
                ..
            }) => {
                let mut params: Vec<String> = params
                    .to_vec()
                    .iter()
                    .map(|p| p.param_type.to_string())
                    .collect();

                params.push(return_type.to_string());

                write!(f, "Func<{}>", params.join(", "))
            }
            Value::Num(n) => write!(f, "{}", n),
            Value::Object(fields) => {
                let fields = fields
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value))
                    .fold(String::new(), |a, b| a + &b + ", ");

                write!(f, "{{ {fields} }}")
            }
            Value::String(t) => write!(f, "{}", t),
            Value::Variant(EnumVariantValue {
                variant_ident,
                data,
                ..
            }) => {
                let data = if let Some(data) = data {
                    format!("({0:?})", data)
                } else {
                    "".to_string()
                };

                write!(f, "{variant_ident}{data}")
            }
            Value::Void => write!(f, "<Void>"),
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

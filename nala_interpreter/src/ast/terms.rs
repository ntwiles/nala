use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use crate::{
    scope::{ScopeId, Scopes},
    types::{struct_field::StructField, type_variant::TypeVariant, NalaType},
};

use super::{types::primitive_type::PrimitiveType, *};

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

#[derive(Debug, Clone)]
pub enum Value {
    Array(Arc<Mutex<Vec<Value>>>),
    Bool(bool),
    Func(Vec<Param>, TypeLiteralVariant, Box<Block>),
    Variant(String, String, Option<Box<Value>>),
    Num(f32),
    Object(Arc<Mutex<HashMap<String, Value>>>),
    String(String),
    Type(TypeLiteralVariant),

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
            Value::Func(params, _return_type, _block) => {
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
            Value::Void => write!(f, "Void"),
        }
    }
}

impl Value {
    pub fn get_type(&self, scopes: &mut Scopes, current_scope: ScopeId) -> TypeVariant {
        match self {
            Value::Array(items) => {
                let items = Arc::clone(items);
                let items = items.lock().unwrap();
                let elem_type = if items.len() > 0 {
                    items.first().unwrap().get_type(scopes, current_scope)
                } else {
                    todo!("Handle the case where trying to get the type of an empty array.")
                };

                TypeVariant::Nested(
                    NalaType::PrimitiveType(PrimitiveType::Array),
                    vec![elem_type],
                )
            }
            Value::Bool(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool)),
            Value::Break(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Break)),
            Value::Func(params, return_type, _block) => {
                let mut param_types: Vec<TypeVariant> = params
                    .into_iter()
                    .map(|p| TypeVariant::from_literal(p.clone().param_type, scopes, current_scope)) // TODO: Why do we need this clone?
                    .collect();

                param_types.push(TypeVariant::from_literal(
                    return_type.clone(),
                    scopes,
                    current_scope,
                ));

                TypeVariant::Nested(NalaType::PrimitiveType(PrimitiveType::Func), param_types)
            }
            Value::Type(_) => todo!("What is this?"),
            Value::Num(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
            Value::Object(fields) => {
                let fields = fields
                    .lock()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(ident, v)| StructField {
                        ident,
                        field_type: v.get_type(scopes, current_scope),
                    })
                    .collect();

                TypeVariant::Type(NalaType::Struct(fields))
            }
            Value::String(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
            Value::Variant(_, _, _) => todo!(),
            Value::Void => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Void)),
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

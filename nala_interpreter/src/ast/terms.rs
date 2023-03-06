use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use crate::{
    errors::RuntimeError,
    scopes::Scopes,
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

// TODO: Find a home for this
#[derive(Debug, Clone)]
pub struct StoredFunc {
    pub params: Vec<Param>,
    pub return_type: TypeLiteralVariant,
    pub block: Box<Block>,
    pub closure_scope: usize,
}

// TODO: Find a home for this
#[derive(Debug, Clone)]
pub struct EnumVariantValue {
    pub enum_ident: String,
    pub variant_ident: String,
    pub data: Option<Box<Value>>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Array(Arc<Mutex<Vec<Value>>>),
    Bool(bool),
    Func(StoredFunc),
    Variant(EnumVariantValue),
    Num(f32),
    Object(Arc<Mutex<HashMap<String, Value>>>),
    String(String),
    Type(TypeLiteralVariant),
    Break(Box<Value>),
    Void,
}

impl Value {
    pub fn as_string(&self) -> Result<String, RuntimeError> {
        if let Value::String(string) = self {
            Ok(string.to_owned())
        } else {
            Err(RuntimeError::new(&format!(
                "Term `{self}` is not a String."
            )))
        }
    }

    pub fn as_variant(&self) -> Result<EnumVariantValue, RuntimeError> {
        if let Value::Variant(variant) = self {
            Ok(variant.clone())
        } else {
            Err(RuntimeError::new(&format!(
                "Term `{}` is not a Variant!",
                self
            )))
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
            Value::Func(StoredFunc {
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
                        .map(|(key, value)| format!("{}: {}", key, value))
                        .fold(String::new(), |a, b| a + &b + ", ")
                )?;

                write!(f, "}}")?;

                Ok(())
            }
            Value::String(t) => write!(f, "{}", t),
            Value::Type(type_kind) => write!(f, "{}", type_kind),
            Value::Variant(EnumVariantValue {
                enum_ident,
                variant_ident,
                data,
            }) => {
                let data = if let Some(data) = data {
                    format!("({0})", data)
                } else {
                    "".to_string()
                };

                write!(f, "{0}::{1}{2}", enum_ident, variant_ident, data)
            }
            Value::Void => write!(f, "Void"),
        }
    }
}

impl Value {
    pub fn get_type(
        &self,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<TypeVariant, RuntimeError> {
        let result = match self {
            Value::Array(items) => {
                let items = Arc::clone(items);
                let items = items.lock().unwrap();
                let elem_type = if items.len() > 0 {
                    items.first().unwrap().get_type(scopes, current_scope)?
                } else {
                    todo!("Handle the case where trying to get the type of an empty array.")
                };

                TypeVariant::Generic(
                    NalaType::PrimitiveType(PrimitiveType::Array),
                    vec![elem_type],
                )
            }
            Value::Bool(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool)),
            Value::Break(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Break)),
            Value::Func(StoredFunc {
                params,
                return_type,
                ..
            }) => {
                let mut param_types: Vec<TypeVariant> = params
                    .into_iter()
                    .map(|p| {
                        TypeVariant::from_literal(p.clone().param_type, scopes, current_scope)
                            // TODO: Remove this unwrap().
                            .unwrap()
                    }) // TODO: Why do we need this clone?
                    .collect();

                param_types.push(TypeVariant::from_literal(
                    return_type.clone(),
                    scopes,
                    current_scope,
                )?);

                TypeVariant::Generic(NalaType::PrimitiveType(PrimitiveType::Func), param_types)
            }
            Value::Type(_) => todo!("What is this?"),
            Value::Num(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
            Value::Object(fields) => {
                let fields = fields
                    .lock()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|(ident, v)| {
                        let field_type = v.get_type(scopes, current_scope).unwrap();
                        StructField { ident, field_type }
                    })
                    .collect();

                TypeVariant::Type(NalaType::Struct(fields))
            }
            Value::String(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
            Value::Variant(EnumVariantValue { enum_ident, .. }) => {
                let variants = scopes
                    .get_type(enum_ident, current_scope)?
                    .as_enum()
                    .unwrap();

                TypeVariant::Type(NalaType::Enum(enum_ident.to_owned(), variants))
            }
            Value::Void => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Void)),
        };

        Ok(result)
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

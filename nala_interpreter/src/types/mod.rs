pub mod fit;
pub mod inference;
pub mod struct_field;
pub mod type_variant;

use std::fmt;

use crate::{
    ast::types::{
        primitive_type::PrimitiveType, type_literal::TypeLiteral, variant_declare::VariantDeclare,
    },
    errors::RuntimeError,
    scopes::{type_binding::TypeBinding, Scopes},
};

use self::struct_field::StructField;

#[derive(Eq, Debug, Clone)]
pub enum NalaType {
    Enum(String, Vec<VariantDeclare>),
    PrimitiveType(PrimitiveType),
    Struct(Vec<StructField>), // PERFORMANCE: Regardless of how this is parsed, shouldn't we
                              // operate on it as a hashmap instead of a vec?
}

impl NalaType {
    pub fn from_literal(
        literal: TypeLiteral,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match literal {
            TypeLiteral::PrimitiveType(t) => Ok(NalaType::PrimitiveType(t)),
            TypeLiteral::UserDefined(ident) => match scopes.get_type(&ident, current_scope)? {
                TypeBinding::Enum(variants, _type_args) => Ok(NalaType::Enum(ident, variants)),
                TypeBinding::Struct(fields) => Ok(NalaType::Struct(fields)),
            },
        }
    }

    pub fn is_any(&self) -> bool {
        if let NalaType::PrimitiveType(PrimitiveType::Any) = self {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for NalaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NalaType::Enum(enum_ident, _variant_ident) => write!(f, "{enum_ident}"),
            NalaType::PrimitiveType(primitive) => write!(f, "{}", primitive),
            NalaType::Struct(fields) => {
                write!(f, "{{ ")?;

                write!(
                    f,
                    "{}",
                    fields
                        .iter()
                        .map(|field| format!("{}: {}", field.ident, field.value_type.to_string()))
                        .fold(String::new(), |a, b| a + &b + ", ")
                )?;

                write!(f, "}}")?;

                Ok(())
            }
        }
    }
}

impl PartialEq for NalaType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            NalaType::Enum(enum_ident, variant_ident) => {
                if let NalaType::Enum(oei, ovi) = other {
                    enum_ident == oei && variant_ident == ovi
                } else {
                    false
                }
            }
            NalaType::PrimitiveType(sp) => {
                if let NalaType::PrimitiveType(op) = other {
                    sp == op
                } else {
                    false
                }
            }
            NalaType::Struct(fields) => {
                if let NalaType::Struct(of) = other {
                    !fields.iter().any(|field| {
                        of.iter()
                            .find(|f| f.ident == field.ident && f.value_type == field.value_type)
                            .is_none()
                    })
                } else {
                    false
                }
            }
        }
    }
}

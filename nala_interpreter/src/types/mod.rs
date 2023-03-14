mod enum_variants;
pub mod fit;
pub mod inference;
pub mod struct_field;
pub mod type_variant;

use std::fmt;

use crate::{
    ast::types::{primitive_type::PrimitiveType, type_literal::TypeLiteral},
    errors::RuntimeError,
    scopes::{type_binding::TypeBinding, Scopes},
    utils::accept_results,
};

use self::{enum_variants::EnumVariant, struct_field::StructField};

#[derive(Eq, Debug, Clone)]
pub enum NalaType {
    // TODO: I think using VariantDeclare is wrong here. This is for resolved types, but VariantDeclare
    // comprises TypeLiteralVariant. We likely need a new type.
    Enum(String, Vec<EnumVariant>),
    PrimitiveType(PrimitiveType),
    Struct(Vec<StructField>),
    Generic(String),
}

impl NalaType {
    pub fn from_literal(
        literal: TypeLiteral,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match literal {
            TypeLiteral::PrimitiveType(t) => Ok(Self::PrimitiveType(t)),
            TypeLiteral::UserDefined(ident) => match scopes.get_type(&ident, current_scope)? {
                TypeBinding::Enum(binding) => {
                    let variants = binding
                        .variants
                        .iter()
                        .map(|v| {
                            EnumVariant::from_variant_declare(
                                v.clone(), // TODO: Find a way to avoid this clone.
                                scopes,
                                binding.closure_scope,
                            )
                        })
                        .collect();

                    let variants = accept_results(variants)?;

                    Ok(Self::Enum(ident, variants))
                }
                TypeBinding::Struct(fields) => Ok(Self::Struct(fields)),
                TypeBinding::Generic(ident) => Ok(Self::Generic(ident)),
            },
        }
    }

    pub fn get_generic_ident(&self) -> Option<String> {
        match self {
            Self::Enum(_, _) => None,
            Self::PrimitiveType(_) => None,
            Self::Struct(fields) => {
                for field in fields {
                    if let Some(ident) = field.value_type.get_generic_ident() {
                        return Some(ident);
                    }
                }

                None
            }
            Self::Generic(ident) => Some(ident.clone()),
        }
    }

    pub fn is_any(&self) -> bool {
        if let Self::PrimitiveType(PrimitiveType::Any) = self {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for NalaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Enum(enum_ident, _variant_ident) => write!(f, "{enum_ident}"),
            Self::PrimitiveType(primitive) => write!(f, "{}", primitive),
            Self::Struct(fields) => {
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
            Self::Generic(ident) => write!(f, "{ident}"),
        }
    }
}

impl PartialEq for NalaType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Enum(enum_ident, variant_ident) => {
                if let Self::Enum(oei, ovi) = other {
                    enum_ident == oei && variant_ident == ovi
                } else {
                    false
                }
            }
            Self::PrimitiveType(sp) => {
                if let Self::PrimitiveType(op) = other {
                    sp == op
                } else {
                    false
                }
            }
            Self::Struct(fields) => {
                if let Self::Struct(of) = other {
                    !fields.iter().any(|field| {
                        of.iter()
                            .find(|f| f.ident == field.ident && f.value_type == field.value_type)
                            .is_none()
                    })
                } else {
                    false
                }
            }
            Self::Generic(_ident) => todo!(),
        }
    }
}

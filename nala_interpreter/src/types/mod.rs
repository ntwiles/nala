pub mod composite_type;
pub mod fit;
pub mod inference;
pub mod type_variant;

use std::fmt;

use crate::{
    ast::types::{primitive_type::PrimitiveType, type_literal::TypeLiteral},
    errors::RuntimeError,
    resolved::{enum_variants::EnumVariant, from_literal::FromLiteral, struct_field::StructField},
    scopes::{type_binding_variant::TypeBindingVariant, Scopes},
};

use self::type_variant::TypeVariant;

#[derive(Eq, Debug, Clone)]
pub enum NalaType {
    Enum(String, Vec<EnumVariant>),
    PrimitiveType(PrimitiveType),
    Struct(Vec<StructField>),
    Generic(String),
}

impl NalaType {
    pub fn make_concrete(self, generic_ident: &str, concrete_type: &TypeVariant) -> Self {
        match self {
            Self::Generic(_) => unreachable!(),
            Self::Enum(_, _) => todo!(),
            Self::PrimitiveType(prim) => Self::PrimitiveType(prim),
            Self::Struct(fields) => Self::Struct(
                fields
                    .into_iter()
                    .map(|StructField { ident, value_type }| StructField {
                        ident,
                        value_type: value_type.make_concrete(generic_ident, concrete_type),
                    })
                    .collect(),
            ),
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
}

impl FromLiteral<TypeLiteral> for NalaType {
    fn from_literal(
        literal: TypeLiteral,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match literal {
            TypeLiteral::PrimitiveType(t) => Ok(Self::PrimitiveType(t)),
            TypeLiteral::UserDefined(ident) => {
                match scopes.get_type(&ident, current_scope)?.variant {
                    TypeBindingVariant::Enum(binding) => Ok(Self::Enum(ident, binding.variants)),
                    TypeBindingVariant::Struct(fields) => Ok(Self::Struct(fields)),
                    TypeBindingVariant::Generic(ident) => Ok(Self::Generic(ident)),
                    TypeBindingVariant::PrimitiveType(primitive) => {
                        Ok(Self::PrimitiveType(primitive))
                    }
                }
            }
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

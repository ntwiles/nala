pub mod composite_type;
pub mod fit;
pub mod inference;
pub mod type_variant;

use std::fmt;

use crate::{
    ast::types::{primitive_type::PrimitiveType, type_literal::TypeLiteral},
    errors::RuntimeError,
    resolved::{enum_variants::EnumVariant, from_literal::FromLiteral, struct_field::StructField},
    scopes::{type_binding::TypeBinding, Scopes},
};

use self::type_variant::TypeVariant;

#[derive(Eq, Debug, Clone)]
pub enum NalaType {
    Enum(String, Vec<EnumVariant>, Option<String>),
    PrimitiveType(PrimitiveType, Option<String>),
    Struct(Vec<StructField>, Option<String>),
    Generic(String),
}

impl NalaType {
    pub fn make_concrete(self, generic_ident: &str, concrete_type: &TypeVariant) -> Self {
        match self {
            Self::Enum(enum_ident, variants, type_param) => {
                if Some(generic_ident.to_string()) == type_param {
                    Self::Enum(
                        enum_ident,
                        variants
                            .into_iter()
                            .map(|variant| match variant {
                                EnumVariant::Empty(ident) => EnumVariant::Empty(ident),
                                EnumVariant::Data(ident, data_type) => EnumVariant::Data(
                                    ident,
                                    data_type.make_concrete(generic_ident, concrete_type),
                                ),
                            })
                            .collect(),
                        type_param,
                    )
                } else {
                    Self::Enum(enum_ident, variants, type_param)
                }
            }
            Self::Struct(fields, type_param) => Self::Struct(
                if Some(generic_ident.to_string()) == type_param {
                    fields
                        .into_iter()
                        .map(|StructField { ident, value_type }| StructField {
                            ident,
                            value_type: value_type.make_concrete(generic_ident, concrete_type),
                        })
                        .collect()
                } else {
                    fields
                },
                None,
            ),
            other => other,
        }
    }

    pub fn get_type_param(&self) -> Option<String> {
        match self {
            Self::Enum(_ident, _variants, type_param) => type_param.clone(),
            Self::PrimitiveType(_, type_param) => type_param.clone(),
            Self::Struct(_fields, type_param) => type_param.clone(),
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
            TypeLiteral::PrimitiveType(t) => Ok(Self::PrimitiveType(t, None)),
            TypeLiteral::UserDefined(ident) => match scopes.get_type(&ident, current_scope)? {
                TypeBinding::Enum(binding, type_param) => {
                    Ok(Self::Enum(ident, binding.variants, type_param))
                }
                TypeBinding::Struct(fields, type_param) => Ok(Self::Struct(fields, type_param)),
                TypeBinding::Generic(ident) => Ok(Self::Generic(ident)),
                TypeBinding::PrimitiveType(primitive, type_param) => {
                    Ok(Self::PrimitiveType(primitive, type_param))
                }
            },
        }
    }
}

impl fmt::Display for NalaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Enum(enum_ident, _variant_ident, _type_param) => write!(f, "{enum_ident}"),
            Self::PrimitiveType(primitive, _type_param) => write!(f, "{}", primitive),
            Self::Struct(fields, _type_param) => {
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
            Self::Enum(enum_ident, variant_ident, _type_param) => {
                if let Self::Enum(oei, ovi, _otp) = other {
                    enum_ident == oei && variant_ident == ovi
                } else {
                    false
                }
            }
            Self::PrimitiveType(sp, _stp) => {
                if let Self::PrimitiveType(op, _otp) = other {
                    sp == op
                } else {
                    false
                }
            }
            Self::Struct(fields, _type_param) => {
                if let Self::Struct(of, _otp) = other {
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

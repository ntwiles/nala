use std::fmt;

use crate::{
    ast::types::primitive_type::PrimitiveType,
    resolved::{enum_variants::EnumVariant, struct_field::StructField},
};

use super::type_variant::TypeVariant;

#[derive(Eq, Debug, Clone)]
pub enum NalaType {
    Enum(String, Vec<EnumVariant>),
    PrimitiveType(PrimitiveType),
    Struct(Vec<StructField>),
    Generic(String),
}

impl NalaType {
    pub fn make_concrete(self, generic_ident: Option<String>, concrete_type: &TypeVariant) -> Self {
        match self {
            Self::Enum(enum_ident, variants) => Self::Enum(
                enum_ident,
                variants
                    .into_iter()
                    .map(|variant| match variant {
                        EnumVariant::Empty(ident) => EnumVariant::Empty(ident),
                        EnumVariant::Data(ident, data_type) => EnumVariant::Data(
                            ident,
                            data_type.make_concrete(generic_ident.clone(), concrete_type),
                        ),
                    })
                    .collect(),
            ),
            Self::Struct(fields) => Self::Struct(
                fields
                    .into_iter()
                    .map(|StructField { ident, value_type }| StructField {
                        ident,
                        value_type: value_type.make_concrete(generic_ident.clone(), concrete_type),
                    })
                    .collect(),
            ),

            other => other,
        }
    }

    pub fn find_generic_type_param(&self) -> Option<String> {
        match self {
            Self::Enum(_, variants) => variants
                .iter()
                .map(|variant| match variant {
                    EnumVariant::Empty(_) => None,
                    EnumVariant::Data(_, data_type) => data_type.find_generic_type_param(),
                })
                .find(|x| x.is_some())
                .flatten(),

            Self::Struct(fields) => fields
                .iter()
                .map(|field| field.value_type.find_generic_type_param())
                .find(|x| x.is_some())
                .flatten(),

            Self::Generic(ident) => Some(ident.clone()),
            Self::PrimitiveType(_) => None,
        }
    }
}

impl fmt::Display for NalaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Enum(enum_ident, _variant_ident) => write!(f, "{enum_ident}"),
            Self::PrimitiveType(primitive) => write!(f, "{}", primitive),
            Self::Struct(fields) => {
                let mut fields = fields.clone();
                fields.sort_by(|a, b| a.cmp(&b));

                println!("Object entries: {:?}", fields);
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
            Self::Generic(ident) => {
                if let Self::Generic(oi) = other {
                    ident == oi
                } else {
                    false
                }
            }
        }
    }
}

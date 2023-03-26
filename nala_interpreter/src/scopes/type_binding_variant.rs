use crate::{
    ast::types::primitive_type::PrimitiveType, errors::RuntimeError,
    resolved::struct_field::StructField, types::type_variant::TypeVariant,
};

use super::enum_binding::EnumBinding;

#[derive(Clone, Debug)]
pub enum TypeBindingVariant {
    Enum(EnumBinding),
    Struct(Vec<StructField>),
    Generic(String),
    PrimitiveType(PrimitiveType),
}

impl TypeBindingVariant {
    pub fn from_type(type_variant: TypeVariant, generic_ident: Option<String>) -> Self {
        match type_variant {
            TypeVariant::Composite(_composite) => todo!(),
            TypeVariant::Type(the_type) => match the_type {
                crate::types::NalaType::Enum(_ident, variants) => Self::Enum(EnumBinding {
                    variants,
                    generic_ident,
                }),
                crate::types::NalaType::Struct(fields) => Self::Struct(fields),
                crate::types::NalaType::Generic(ident) => Self::Generic(ident),
                crate::types::NalaType::PrimitiveType(primitive) => Self::PrimitiveType(primitive),
            },
        }
    }

    pub fn as_enum(&self) -> Result<EnumBinding, RuntimeError> {
        match self {
            Self::Enum(binding) => Ok(binding.clone()),
            _ => Err(RuntimeError::new("Expected an enum type.")),
        }
    }

    pub fn as_struct(&self) -> Result<Vec<StructField>, RuntimeError> {
        match self {
            Self::Struct(fields) => Ok(fields.clone()),
            _ => Err(RuntimeError::new("Expected a struct type.")),
        }
    }

    pub fn get_generic_ident(&self) -> Option<String> {
        match self {
            Self::Enum(binding) => binding.generic_ident.clone(),
            _ => None,
        }
    }
}

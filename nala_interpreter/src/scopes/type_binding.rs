use crate::{
    ast::types::primitive_type::PrimitiveType, errors::RuntimeError,
    types::type_variant::TypeVariant,
};

use super::{enum_binding::EnumBinding, struct_binding::StructBinding};

#[derive(Clone, Debug)]
pub enum TypeBinding {
    Enum(EnumBinding),
    Struct(StructBinding),
    Generic(String),
    PrimitiveType(PrimitiveType),
}

impl TypeBinding {
    pub fn from_type(type_variant: TypeVariant) -> Self {
        match type_variant {
            TypeVariant::Composite(_outer, _inner) => todo!(),
            TypeVariant::Type(the_type) => match the_type {
                crate::types::NalaType::Enum(_ident, _variants) => todo!(), // We're missing fields like closure_scope.
                crate::types::NalaType::Struct(_fields) => todo!(), // We're missing fields like closure_scope
                crate::types::NalaType::Generic(ident) => Self::Generic(ident),
                crate::types::NalaType::PrimitiveType(primitive) => Self::PrimitiveType(primitive),
            },
        }
    }

    pub fn as_enum(&self) -> Result<EnumBinding, RuntimeError> {
        match self {
            TypeBinding::Enum(binding) => Ok(binding.clone()),
            _ => Err(RuntimeError::new("Expected an enum type.")),
        }
    }

    pub fn as_struct(&self) -> Result<StructBinding, RuntimeError> {
        match self {
            TypeBinding::Struct(binding) => Ok(binding.clone()),
            _ => Err(RuntimeError::new("Expected a struct type.")),
        }
    }

    pub fn get_generic_ident(&self) -> Option<String> {
        match self {
            TypeBinding::Enum(binding) => binding.generic_ident.clone(),
            _ => None,
        }
    }
}

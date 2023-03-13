use crate::{errors::RuntimeError, types::struct_field::StructField};

use super::enum_binding::EnumBinding;

#[derive(Clone, Debug)]
pub enum TypeBinding {
    Enum(EnumBinding),
    Struct(Vec<StructField>),
    Generic(String),
}

impl TypeBinding {
    pub fn as_enum(&self) -> Result<EnumBinding, RuntimeError> {
        match self {
            TypeBinding::Enum(binding) => Ok(binding.clone()),
            _ => Err(RuntimeError::new("Expected an enum type.")),
        }
    }

    pub fn as_struct(&self) -> Result<Vec<StructField>, RuntimeError> {
        match self {
            TypeBinding::Struct(fields) => Ok(fields.clone()),
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

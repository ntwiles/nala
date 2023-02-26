use crate::{ast::VariantDeclare, errors::RuntimeError, types::struct_field::StructField};

#[derive(Clone, Debug)]
pub enum TypeBinding {
    Enum(Vec<VariantDeclare>),
    Struct(Vec<StructField>),
}

impl TypeBinding {
    pub fn as_enum(&self) -> Result<Vec<VariantDeclare>, RuntimeError> {
        match self {
            TypeBinding::Enum(variants) => Ok(variants.clone()),
            _ => Err(RuntimeError::new("Expected an enum type.")),
        }
    }

    pub fn as_struct(&self) -> Result<Vec<StructField>, RuntimeError> {
        match self {
            TypeBinding::Struct(fields) => Ok(fields.clone()),
            _ => Err(RuntimeError::new("Expected a struct type.")),
        }
    }
}

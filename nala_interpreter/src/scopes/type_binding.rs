use crate::{
    ast::{types::TypeArgs, VariantDeclare},
    errors::RuntimeError,
    types::struct_field::StructField,
};

#[derive(Clone, Debug)]
pub enum TypeBinding {
    Enum(Vec<VariantDeclare>, Option<TypeArgs>),
    Struct(Vec<StructField>),
}

impl TypeBinding {
    pub fn as_enum(&self) -> Result<(Vec<VariantDeclare>, Option<TypeArgs>), RuntimeError> {
        match self {
            TypeBinding::Enum(variants, args) => Ok((variants.clone(), args.clone())),
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

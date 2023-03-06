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
            TypeBinding::Enum(variants, type_args) => Ok((variants.clone(), type_args.clone())),
            _ => Err(RuntimeError::new("Expected an enum type.")),
        }
    }

    pub fn as_struct(&self) -> Result<Vec<StructField>, RuntimeError> {
        match self {
            TypeBinding::Struct(fields) => Ok(fields.clone()),
            _ => Err(RuntimeError::new("Expected a struct type.")),
        }
    }

    pub fn is_generic(&self) -> bool {
        match self {
            TypeBinding::Enum(_, type_args) => type_args.is_some(),
            TypeBinding::Struct(_) => false,
        }
    }
}

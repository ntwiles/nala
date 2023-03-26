use crate::{
    ast::types::primitive_type::PrimitiveType, errors::RuntimeError,
    resolved::struct_field::StructField, types::type_variant::TypeVariant,
};

use super::enum_binding::EnumBinding;

#[derive(Clone, Debug)]
pub enum TypeBinding {
    Enum(EnumBinding, Option<String>),
    Struct(Vec<StructField>, Option<String>),
    Generic(String),
    PrimitiveType(PrimitiveType, Option<String>),
}

impl TypeBinding {
    pub fn from_type(type_variant: TypeVariant) -> Self {
        match type_variant {
            TypeVariant::Composite(_composite) => todo!(),
            TypeVariant::Type(the_type) => match the_type {
                crate::types::NalaType::Enum(_ident, variants, type_param) => {
                    Self::Enum(EnumBinding { variants }, type_param)
                }
                crate::types::NalaType::Struct(fields, type_param) => {
                    Self::Struct(fields, type_param)
                }
                crate::types::NalaType::Generic(ident) => Self::Generic(ident),
                crate::types::NalaType::PrimitiveType(primitive, type_param) => {
                    Self::PrimitiveType(primitive, type_param)
                }
            },
        }
    }

    pub fn as_enum(&self) -> Result<(EnumBinding, Option<String>), RuntimeError> {
        match self {
            Self::Enum(binding, type_param) => Ok((binding.clone(), type_param.clone())),
            _ => Err(RuntimeError::new("Expected an enum type.")),
        }
    }

    pub fn as_struct(&self) -> Result<(Vec<StructField>, Option<String>), RuntimeError> {
        match self {
            Self::Struct(fields, type_param) => Ok((fields.clone(), type_param.clone())),
            _ => Err(RuntimeError::new("Expected a struct type.")),
        }
    }

    pub fn get_type_param(&self) -> Result<Option<String>, RuntimeError> {
        match self {
            Self::Enum(_, type_param) => Ok(type_param.clone()),
            Self::Struct(_, type_param) => Ok(type_param.clone()),
            Self::Generic(_) => Ok(None),
            Self::PrimitiveType(_, type_param) => Ok(type_param.clone()),
        }
    }
}

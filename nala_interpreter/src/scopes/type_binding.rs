use crate::{
    errors::RuntimeError, resolved::struct_field::StructField, types::type_variant::TypeVariant,
};

use super::{enum_binding::EnumBinding, type_binding_variant::TypeBindingVariant};

#[derive(Debug, Clone)]
pub struct TypeBinding {
    pub type_param: Option<String>,
    pub variant: TypeBindingVariant,
}

impl TypeBinding {
    pub fn from_type(variant: TypeVariant, type_param: Option<String>) -> Self {
        let variant = TypeBindingVariant::from_type(variant.clone());

        Self {
            type_param,
            variant,
        }
    }

    pub fn as_enum(&self) -> Result<(Option<String>, EnumBinding), RuntimeError> {
        Ok((self.type_param.clone(), self.variant.as_enum()?))
    }

    pub fn as_struct(&self) -> Result<Vec<StructField>, RuntimeError> {
        self.variant.as_struct()
    }

    pub fn get_type_param(&self) -> Option<String> {
        self.type_param.clone()
    }
}

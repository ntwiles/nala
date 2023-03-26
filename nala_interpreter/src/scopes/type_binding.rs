use crate::{
    errors::RuntimeError, resolved::struct_field::StructField, types::type_variant::TypeVariant,
};

use super::{enum_binding::EnumBinding, type_binding_variant::TypeBindingVariant};

#[derive(Debug, Clone)]
pub struct TypeBinding {
    pub variant: TypeBindingVariant,
}

impl TypeBinding {
    pub fn from_type(type_variant: TypeVariant) -> Self {
        let variant = TypeBindingVariant::from_type(type_variant.clone(), None);

        Self { variant }
    }

    pub fn as_enum(&self) -> Result<EnumBinding, RuntimeError> {
        self.variant.as_enum()
    }

    pub fn as_struct(&self) -> Result<Vec<StructField>, RuntimeError> {
        self.variant.as_struct()
    }

    pub fn get_generic_ident(&self) -> Option<String> {
        self.variant.get_generic_ident()
    }
}

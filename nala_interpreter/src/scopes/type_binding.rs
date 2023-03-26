use crate::{
    errors::RuntimeError, resolved::struct_field::StructField, types::type_variant::TypeVariant,
};

use super::{enum_binding::EnumBinding, type_binding_variant::TypeBindingVariant};

#[derive(Debug, Clone)]
pub struct TypeBinding {
    pub variant: TypeBindingVariant,
    pub generic_ident: Option<String>, // TODO: Rename type_param.
}

impl TypeBinding {
    pub fn from_type(type_variant: TypeVariant, generic_ident: Option<String>) -> Self {
        let variant = TypeBindingVariant::from_type(type_variant.clone());

        Self {
            variant,
            generic_ident,
        }
    }

    pub fn as_enum(&self) -> Result<(Option<String>, EnumBinding), RuntimeError> {
        Ok((self.generic_ident.clone(), self.variant.as_enum()?))
    }

    pub fn as_struct(&self) -> Result<Vec<StructField>, RuntimeError> {
        self.variant.as_struct()
    }

    pub fn get_generic_ident(&self) -> Option<String> {
        self.generic_ident.clone()
    }
}

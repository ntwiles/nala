use std::cmp::Ordering;

use crate::{ast::types::StructLiteralField, errors::RuntimeError, scopes::Scopes};

use super::type_variant::TypeVariant;

#[derive(Eq, Debug, Clone)]
pub struct StructField {
    pub ident: String,
    pub value: TypeVariant,
}

impl StructField {
    pub fn from_literal(
        field: StructLiteralField,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        Ok(Self {
            ident: field.ident,
            value: TypeVariant::from_struct_literal_field(field.value, scopes, current_scope)?,
        })
    }
}

impl PartialEq for StructField {
    fn eq(self: &Self, other: &StructField) -> bool {
        self.ident == other.ident && self.value == other.value
    }
}

impl PartialOrd for StructField {
    fn partial_cmp(&self, other: &StructField) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StructField {
    fn cmp(&self, other: &StructField) -> Ordering {
        self.ident.cmp(&other.ident)
    }
}

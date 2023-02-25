use std::cmp::Ordering;

use crate::{ast::types::StructLiteralField, scopes::Scopes};

use super::type_variant::TypeVariant;

#[derive(Eq, Debug, Clone)]
pub struct StructField {
    pub ident: String,
    pub field_type: TypeVariant,
}

impl StructField {
    pub fn from_literal(
        field: StructLiteralField,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Self {
        Self {
            ident: field.ident,
            field_type: TypeVariant::from_literal(field.field_type, scopes, current_scope),
        }
    }
}

impl PartialEq for StructField {
    fn eq(self: &Self, other: &StructField) -> bool {
        self.ident == other.ident && self.field_type == other.field_type
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

use crate::{
    ast::types::StructLiteralField,
    scope::{ScopeId, Scopes},
};

use super::type_variant::TypeVariant;

#[derive(Debug, Clone)]
pub struct StructField {
    pub ident: String,
    pub field_type: TypeVariant,
}

impl StructField {
    pub fn from_literal(
        field: StructLiteralField,
        scopes: &mut Scopes,
        current_scope: ScopeId,
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

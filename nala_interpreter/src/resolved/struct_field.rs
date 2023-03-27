use std::cmp::Ordering;

use crate::{
    ast::types::{StructLiteralField, StructLiteralFieldValue},
    errors::RuntimeError,
    scopes::Scopes,
    types::{nala_type::NalaType, type_variant::TypeVariant},
};

#[derive(Eq, Debug, Clone)]
pub struct StructField {
    pub ident: String,
    pub value_type: TypeVariant,
}

impl StructField {
    pub fn from_literal(
        field: StructLiteralField,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        Ok(Self {
            ident: field.ident,
            value_type: type_from_field(field.value, scopes, current_scope)?,
        })
    }
}

impl PartialEq for StructField {
    fn eq(self: &Self, other: &StructField) -> bool {
        self.ident == other.ident && self.value_type == other.value_type
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

pub fn type_from_field(
    literal: StructLiteralFieldValue,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<TypeVariant, RuntimeError> {
    match literal {
        StructLiteralFieldValue::Nested(fields) => {
            let fields: Vec<StructField> = fields
                .into_iter()
                .map(|field| StructField {
                    ident: field.ident,
                    value_type: type_from_field(field.value, scopes, current_scope).unwrap(),
                })
                .collect();

            Ok(TypeVariant::Type(NalaType::Struct(fields)))
        }
        StructLiteralFieldValue::Type(t) => TypeVariant::from_literal(t, scopes, current_scope),
    }
}

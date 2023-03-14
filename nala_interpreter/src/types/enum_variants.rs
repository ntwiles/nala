use crate::{ast::types::variant_declare::VariantDeclare, errors::RuntimeError, scopes::Scopes};

use super::type_variant::TypeVariant;

#[derive(Eq, Debug, Clone, PartialEq)]
pub enum EnumVariant {
    Empty(String),
    Data(String, TypeVariant), // Use something other than TypeLiteralVariant here.
}

impl EnumVariant {
    pub fn from_variant_declare(
        declare: VariantDeclare,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError> {
        match declare {
            VariantDeclare::Empty(ident) => Ok(Self::Empty(ident)),
            VariantDeclare::Data(ident, t) => Ok(Self::Data(
                ident,
                TypeVariant::from_literal(t, scopes, current_scope)?,
            )),
        }
    }
}

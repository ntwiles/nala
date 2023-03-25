use crate::{
    ast::types::variant_declare::VariantDeclare, errors::RuntimeError, scopes::Scopes,
    types::type_variant::TypeVariant,
};

use super::from_literal::FromLiteral;

#[derive(Eq, Debug, Clone, PartialEq)]
pub enum EnumVariant {
    Empty(String),
    Data(String, TypeVariant),
}

impl FromLiteral<VariantDeclare> for EnumVariant {
    fn from_literal(
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

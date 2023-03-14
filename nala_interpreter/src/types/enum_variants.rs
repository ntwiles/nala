use crate::ast::types::{
    type_literal_variant::TypeLiteralVariant, variant_declare::VariantDeclare,
};

#[derive(Eq, Debug, Clone, PartialEq)]
pub enum EnumVariant {
    Empty(String),
    Data(String, TypeLiteralVariant), // Use something other than TypeLiteralVariant here.
}

impl EnumVariant {
    pub fn from_variant_declare(declare: VariantDeclare) -> Self {
        match declare {
            VariantDeclare::Empty(ident) => Self::Empty(ident),
            VariantDeclare::Data(ident, t) => Self::Data(ident, t),
        }
    }
}

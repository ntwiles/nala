use std::fmt;

use super::type_literal_variant::TypeLiteralVariant;

#[derive(Eq, Debug, Clone)]
pub enum VariantDeclare {
    Empty(String),
    Data(String, TypeLiteralVariant),
}

impl fmt::Display for VariantDeclare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantDeclare::Data(variant, data_type) => write!(f, "{0}({1})", variant, data_type),
            VariantDeclare::Empty(variant) => write!(f, "{}", variant),
        }
    }
}

impl PartialEq for VariantDeclare {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                VariantDeclare::Data(variant, data_type),
                VariantDeclare::Data(other_variant, other_data_type),
            ) => variant == other_variant && data_type == other_data_type,
            (VariantDeclare::Empty(variant), VariantDeclare::Empty(other_variant)) => {
                variant == other_variant
            }
            _ => false,
        }
    }
}

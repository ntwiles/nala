use std::fmt::{Display, Formatter, Result};

use crate::types::type_variant::TypeVariant;

use self::type_literal_variant::TypeLiteralVariant;

pub mod enum_variant;
pub mod primitive_type;
pub mod type_literal;
pub mod type_literal_variant;

#[derive(Debug, Clone)]
pub struct StructLiteralField {
    pub ident: String,
    pub field_type: TypeLiteralVariant,
}

impl StructLiteralField {
    pub fn new(ident: &str, field_type: TypeLiteralVariant) -> Self {
        Self {
            ident: ident.to_owned(),
            field_type,
        }
    }
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum TypeArgs {
    Generic(String),
    Concrete(Box<TypeVariant>),
}

impl Display for TypeArgs {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TypeArgs::Generic(s) => write!(f, "{}", s),
            TypeArgs::Concrete(i) => write!(f, "{}", i),
        }
    }
}

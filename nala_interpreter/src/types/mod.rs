use std::fmt;

use crate::{
    ast::types::{primitive_type::PrimitiveType, type_literal::TypeLiteral},
    scope::{ScopeId, Scopes},
};

use self::struct_field::StructField;

pub mod struct_field;
pub mod type_variant;

#[derive(Debug, Clone)]
pub enum NalaType {
    PrimitiveType(PrimitiveType),
    Struct(Vec<StructField>),
}

impl NalaType {
    pub fn from_literal(literal: TypeLiteral, scopes: &mut Scopes, current_scope: ScopeId) -> Self {
        match literal {
            TypeLiteral::PrimitiveType(t) => NalaType::PrimitiveType(t),
            TypeLiteral::UserDefined(ident) => {
                NalaType::Struct(scopes.get_struct(&ident, current_scope).unwrap())
            }
        }
    }

    pub fn is_assignable_to(&self, other: &Self) -> bool {
        match self {
            NalaType::PrimitiveType(sp) => {
                if let NalaType::PrimitiveType(op) = other {
                    sp.is_assignable_to(op)
                } else {
                    false
                }
            }
            NalaType::Struct(fields) => {
                if let NalaType::Struct(ot) = other {
                    fields == ot
                } else {
                    false
                }
            }
        }
    }
}

impl fmt::Display for NalaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NalaType::PrimitiveType(primitive) => write!(f, "{}", primitive),
            NalaType::Struct(fields) => write!(f, "{:?}", fields), // TODO: This is not human readable.
        }
    }
}

impl PartialEq for NalaType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            NalaType::PrimitiveType(sp) => {
                if let NalaType::PrimitiveType(op) = other {
                    sp == op
                } else {
                    false
                }
            }
            NalaType::Struct(_) => todo!(),
        }
    }
}

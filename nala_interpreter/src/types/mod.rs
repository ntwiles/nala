use std::fmt;

use crate::ast::types::{primitive_type::PrimitiveType, type_literal::TypeLiteral, StructField};

pub mod type_variant;

#[derive(Debug, Clone)]
pub enum NalaType {
    PrimitiveType(PrimitiveType),
    Struct(Vec<StructField>),
}

impl NalaType {
    pub fn from_literal(literal: TypeLiteral) -> Self {
        match literal {
            TypeLiteral::PrimitiveType(t) => NalaType::PrimitiveType(t),
            TypeLiteral::UserDefined(_) => todo!(),
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
                    todo!()
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
            NalaType::Struct(the_type) => todo!(),
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
            NalaType::Struct(sp) => todo!(),
        }
    }
}

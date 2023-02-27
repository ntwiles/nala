use std::fmt;

use super::primitive_type::PrimitiveType;

#[derive(Debug, Clone)]
pub enum TypeLiteral {
    PrimitiveType(PrimitiveType),
    UserDefined(String),
}

impl fmt::Display for TypeLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeLiteral::PrimitiveType(primitive) => write!(f, "{}", primitive),
            TypeLiteral::UserDefined(the_type) => write!(f, "{}", the_type),
        }
    }
}

impl PartialEq for TypeLiteral {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TypeLiteral::PrimitiveType(sp) => {
                if let TypeLiteral::PrimitiveType(op) = other {
                    sp == op
                } else {
                    false
                }
            }
            TypeLiteral::UserDefined(sp) => {
                if let TypeLiteral::UserDefined(op) = other {
                    sp == op
                } else {
                    false
                }
            }
        }
    }
}

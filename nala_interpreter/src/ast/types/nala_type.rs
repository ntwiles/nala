use std::fmt;

use super::primitive_type::PrimitiveType;

#[derive(Debug, Clone)]
pub enum NalaType {
    PrimitiveType(PrimitiveType),
    UserDefined(String),
}

impl NalaType {
    pub fn is_assignable_to(&self, other: &Self) -> bool {
        match self {
            NalaType::PrimitiveType(sp) => {
                if let NalaType::PrimitiveType(op) = other {
                    sp.is_assignable_to(op)
                } else {
                    false
                }
            }
            NalaType::UserDefined(st) => {
                if let NalaType::UserDefined(ot) = other {
                    st == ot
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
            NalaType::UserDefined(the_type) => write!(f, "{}", the_type),
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
            NalaType::UserDefined(sp) => {
                if let NalaType::UserDefined(op) = other {
                    sp == op
                } else {
                    false
                }
            }
        }
    }
}

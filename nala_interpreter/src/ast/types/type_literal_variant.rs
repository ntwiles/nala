use std::fmt;

use super::type_literal::TypeLiteral;

#[derive(Eq, Debug, Clone)]
pub enum TypeVariantLiteral {
    Composite(TypeLiteral, Vec<Self>),
    Type(TypeLiteral),
}

impl fmt::Display for TypeVariantLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Composite(v, vv) => {
                let children = vv
                    .iter()
                    .map(|vv| vv.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "{0}<{1}>", v, children)
            }
            Self::Type(t) => write!(f, "{}", t),
        }
    }
}

impl PartialEq for TypeVariantLiteral {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Composite(mv, mg) => {
                if let Self::Composite(ov, og) = other {
                    mv == ov && mg == og
                } else {
                    false
                }
            }
            Self::Type(me) => {
                if let Self::Type(other) = other {
                    me == other
                } else {
                    false
                }
            }
        }
    }
}

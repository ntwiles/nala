use std::fmt;

use super::type_literal::TypeLiteral;

#[derive(Debug, Clone)]
pub enum TypeLiteralVariant {
    Nested(TypeLiteral, Vec<TypeLiteralVariant>),
    Type(TypeLiteral),
}

impl fmt::Display for TypeLiteralVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeLiteralVariant::Nested(v, vv) => {
                let children = vv
                    .iter()
                    .map(|vv| vv.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "{0}<{1}>", v, children)
            }
            TypeLiteralVariant::Type(t) => write!(f, "{}", t),
        }
    }
}

impl PartialEq for TypeLiteralVariant {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TypeLiteralVariant::Nested(mv, mg) => {
                if let TypeLiteralVariant::Nested(ov, og) = other {
                    mv == ov && mg == og
                } else {
                    false
                }
            }
            TypeLiteralVariant::Type(me) => {
                if let TypeLiteralVariant::Type(other) = other {
                    me == other
                } else {
                    false
                }
            }
        }
    }
}
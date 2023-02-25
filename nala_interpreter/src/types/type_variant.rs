use std::fmt;

use crate::{ast::types::type_literal_variant::TypeLiteralVariant, scopes::Scopes};

use super::NalaType;

#[derive(Eq, Debug, Clone)]
pub enum TypeVariant {
    Composite(NalaType, Vec<TypeVariant>),
    Type(NalaType),
}

impl TypeVariant {
    pub fn from_literal(
        literal: TypeLiteralVariant,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Self {
        match literal {
            TypeLiteralVariant::Nested(p, c) => TypeVariant::Composite(
                NalaType::from_literal(p, scopes, current_scope),
                c.into_iter()
                    .map(|l| TypeVariant::from_literal(l, scopes, current_scope))
                    .collect(),
            ),
            TypeLiteralVariant::Type(t) => {
                TypeVariant::Type(NalaType::from_literal(t, scopes, current_scope))
            }
        }
    }

    pub fn is_assignable_to(&self, other: &Self) -> bool {
        match self {
            TypeVariant::Composite(sv, svv) => {
                if let TypeVariant::Composite(ov, ovv) = other {
                    if !sv.is_assignable_to(ov) {
                        return false;
                    }

                    for (i, si) in svv.iter().enumerate() {
                        let oi = &ovv[i];
                        if !si.is_assignable_to(&oi) {
                            return false;
                        }
                    }

                    true
                } else {
                    false
                }
            }
            TypeVariant::Type(st) => match other {
                TypeVariant::Type(ot) => st.is_assignable_to(ot),
                _ => false,
            },
        }
    }
}

impl fmt::Display for TypeVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeVariant::Composite(v, vv) => {
                let children = vv
                    .iter()
                    .map(|vv| vv.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "{0}<{1}>", v, children)
            }
            TypeVariant::Type(t) => write!(f, "{}", t),
        }
    }
}

impl PartialEq for TypeVariant {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TypeVariant::Composite(mv, mg) => {
                if let TypeVariant::Composite(ov, og) = other {
                    mv == ov && mg == og
                } else {
                    false
                }
            }
            TypeVariant::Type(me) => {
                if let TypeVariant::Type(other) = other {
                    me == other
                } else {
                    false
                }
            }
        }
    }
}

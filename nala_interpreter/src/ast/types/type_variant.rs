use std::fmt;

use super::nala_type::NalaType;

#[derive(Debug, Clone)]
pub enum TypeVariant {
    Nested(NalaType, Vec<TypeVariant>),
    Type(NalaType),
}

impl TypeVariant {
    pub fn is_assignable_to(&self, other: &Self) -> bool {
        match self {
            TypeVariant::Nested(sv, svv) => {
                if let TypeVariant::Nested(ov, ovv) = other {
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
            TypeVariant::Nested(v, vv) => {
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
            TypeVariant::Nested(mv, mg) => {
                if let TypeVariant::Nested(ov, og) = other {
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

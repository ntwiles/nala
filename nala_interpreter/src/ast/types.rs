use std::fmt;

#[derive(Debug, Clone)]
pub enum TypeVariant {
    Nested(Type, Vec<TypeVariant>),
    Type(Type),
}

#[derive(Debug, Clone)]
pub enum Type {
    PrimitiveType(PrimitiveType),
    UserDefined(String),
}

impl Type {
    pub fn is_assignable_to(&self, other: &Self) -> bool {
        match self {
            Type::PrimitiveType(sp) => {
                if let Type::PrimitiveType(op) = other {
                    sp.is_assignable_to(op)
                } else {
                    false
                }
            }
            Type::UserDefined(st) => {
                if let Type::UserDefined(ot) = other {
                    st == ot
                } else {
                    false
                }
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::PrimitiveType(primitive) => write!(f, "{}", primitive),
            Type::UserDefined(the_type) => write!(f, "{}", the_type),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Type::PrimitiveType(sp) => {
                if let Type::PrimitiveType(op) = other {
                    sp == op
                } else {
                    false
                }
            }
            Type::UserDefined(sp) => {
                if let Type::UserDefined(op) = other {
                    sp == op
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Array,
    Bool,
    Break,
    Exception,
    Func,
    Number,
    String,
    Symbol,
    Void,
    Object,
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

impl PrimitiveType {
    pub fn is_assignable_to(&self, param: &PrimitiveType) -> bool {
        // TODO: Not a good way to compare types.
        self.to_string() == param.to_string()
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = match self {
            PrimitiveType::Array => "Array",
            PrimitiveType::Bool => "Bool",
            PrimitiveType::Break => "<Break>",
            PrimitiveType::Exception => "<Exception>", // TODO: What is this??
            PrimitiveType::Func => "Func",
            PrimitiveType::Number => "Number",
            PrimitiveType::Object => "<Object>",
            PrimitiveType::String => "String",
            PrimitiveType::Symbol => "<Symbol>",
            PrimitiveType::Void => "<Void>",
        };

        write!(f, "{}", type_name)
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

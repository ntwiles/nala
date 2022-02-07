use super::*;

use crate::types::get_interfaces_for_primitive_type;

#[derive(Debug, Clone)]
pub enum Types {
    Types(Box<Types>, TypeVariant),
    Type(TypeVariant),
}

#[derive(Debug, Clone)]
pub enum TypeVariant {
    Nested(PrimitiveType, Box<Types>),
    Enum(String, Box<KindsDeclare>),
    Primitive(PrimitiveType),
    Interface(PrimitiveInterface),
}

impl TypeVariant {
    pub fn implements_interface(&self, interface: PrimitiveInterface) -> bool {
        let interfaces = match self {
            TypeVariant::Primitive(primitive) => {
                get_interfaces_for_primitive_type(primitive.clone())
            }
            TypeVariant::Nested(outer, _inner) => get_interfaces_for_primitive_type(outer.clone()),
            _ => todo!(),
        };

        interfaces.contains(&interface)
    }
}

#[derive(Debug, Clone)]
pub enum PrimitiveInterface {
    IAdd,
    ICompare,
    IDivide,
    IEqual,
    IMultiply,
    IPrint,
    ISubtract,
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Array,
    Bool,
    Break,
    Func,
    Number,
    String,
    Symbol,
    Void,
    Enum,
    Kind,
    Unknown,
}

impl Types {
    pub fn are_assignable_to(&self, other: &Self) -> bool {
        match self {
            Types::Types(svv, sv) => {
                if let Types::Types(ovv, ov) = other {
                    sv.is_assignable_to(ov) && svv.are_assignable_to(ovv)
                } else {
                    false
                }
            }
            Types::Type(sv) => {
                if let Types::Type(ov) = other {
                    sv.is_assignable_to(ov)
                } else {
                    false
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Types::Type(s) => s.to_string(),
            Types::Types(ss, s) => format!("{0}, {1}", ss.to_string(), s.to_string()),
        }
    }

    pub fn from_vec(types: Vec<TypeVariant>) -> Types {
        match types.len() {
            1 => Types::Type(types.first().unwrap().clone()),
            _ => {
                let last = types.last().unwrap();
                let remaining = Types::from_vec(types[..types.len() - 1].to_owned());
                Types::Types(Box::new(remaining), last.clone())
            }
        }
    }
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Types::Types(mvv, mv) => {
                if let Types::Types(ovv, ov) = other {
                    ovv == mvv && ov == mv
                } else {
                    false
                }
            }
            Types::Type(mv) => {
                if let Types::Type(ov) = other {
                    mv == ov
                } else {
                    false
                }
            }
        };
    }
}

impl TypeVariant {
    pub fn is_assignable_to(&self, other: &Self) -> bool {
        if let TypeVariant::Interface(i) = other {
            return self.implements_interface(i.clone());
        };

        match self {
            TypeVariant::Nested(sv, svv) => {
                if let TypeVariant::Nested(ov, ovv) = other {
                    sv.is_assignable_to(ov) && svv.are_assignable_to(ovv)
                } else {
                    false
                }
            }
            TypeVariant::Primitive(sv) => {
                if let TypeVariant::Primitive(ov) = other {
                    sv.is_assignable_to(ov)
                } else {
                    false
                }
            }
            TypeVariant::Enum(_, _) => todo!(),
            TypeVariant::Interface(i) => {
                if let TypeVariant::Interface(other) = other {
                    i == other
                } else {
                    false
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TypeVariant::Nested(v, vv) => format!("{0}<{1}>", v.to_string(), vv.to_string()),
            TypeVariant::Primitive(v) => v.to_string(),
            TypeVariant::Enum(_, _) => todo!(),
            TypeVariant::Interface(i) => i.to_string(),
        }
    }
}

impl PartialEq for TypeVariant {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TypeVariant::Nested(mv, mg) => {
                if let TypeVariant::Nested(ov, og) = other {
                    return mv == ov && mg == og;
                } else {
                    panic!(
                        "Cannot compare between types `{0}` and `{1}`.",
                        self.to_string(),
                        other.to_string()
                    )
                }
            }
            TypeVariant::Primitive(me) => {
                if let TypeVariant::Primitive(other) = other {
                    return me == other;
                } else {
                    panic!(
                        "Cannot compare between types `{0}` and `{1}`.",
                        self.to_string(),
                        other.to_string()
                    )
                }
            }
            TypeVariant::Enum(_, _) => {
                todo!()
            }
            TypeVariant::Interface(_) => {
                todo!()
            }
        }
    }
}

impl PrimitiveType {
    pub fn is_assignable_to(&self, param: &Self) -> bool {
        self.to_string() == param.to_string()
    }

    pub fn to_string(&self) -> String {
        let type_name = match self {
            PrimitiveType::Array => "Array",
            PrimitiveType::Bool => "Bool",
            PrimitiveType::Break => "<Break>",
            PrimitiveType::Func => "Func",
            PrimitiveType::Number => "Number",
            PrimitiveType::String => "String",
            PrimitiveType::Symbol => "<Symbol>",
            PrimitiveType::Void => "<Void>",
            PrimitiveType::Enum => "<Enum>",
            PrimitiveType::Kind => "Kind",
            PrimitiveType::Unknown => "<Unknown>",
        };

        String::from(type_name)
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl PrimitiveInterface {
    pub fn to_string(&self) -> String {
        // TODO: Can we get the interface name from the enum variant dynamically?
        let type_name = match self {
            PrimitiveInterface::IAdd => "IAdd",
            PrimitiveInterface::ICompare => "ICompare",
            PrimitiveInterface::IDivide => "IDivide",
            PrimitiveInterface::IEqual => "IEqual",
            PrimitiveInterface::IMultiply => "IMultiply",
            PrimitiveInterface::IPrint => "IPrint",
            PrimitiveInterface::ISubtract => "ISubtract",
        };

        String::from(type_name)
    }
}

impl PartialEq for PrimitiveInterface {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

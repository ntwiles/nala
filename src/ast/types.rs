use std::fmt;

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
    Enum(String, Box<VariantsDeclare>),
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
    Exception,
    Func,
    Number,
    Pattern,
    String,
    Symbol,
    Void,
    Enum,
    Variant,
    Unknown,
    Object,
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

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Types::Type(s) => write!(f, "{}", s),
            Types::Types(ss, s) => write!(f, "{0}, {1}", ss, s),
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
}

impl fmt::Display for TypeVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeVariant::Nested(v, vv) => write!(f, "{0}<{1}>", v, vv),
            TypeVariant::Primitive(v) => write!(f, "{}", v),
            TypeVariant::Enum(_, _) => todo!(),
            TypeVariant::Interface(i) => write!(f, "{}", i),
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
                    panic!("Cannot compare between types `{0}` and `{1}`.", self, other)
                }
            }
            TypeVariant::Primitive(me) => {
                if let TypeVariant::Primitive(other) = other {
                    return me == other;
                } else {
                    panic!("Cannot compare between types `{0}` and `{1}`.", self, other)
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
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = match self {
            PrimitiveType::Array => "Array",
            PrimitiveType::Bool => "Bool",
            PrimitiveType::Break => "<Break>",
            PrimitiveType::Enum => "<Enum>",
            PrimitiveType::Exception => "<Exception>",
            PrimitiveType::Func => "Func",
            PrimitiveType::Number => "Number",
            PrimitiveType::Object => "<Object>",
            PrimitiveType::Pattern => "Pattern",
            PrimitiveType::String => "String",
            PrimitiveType::Symbol => "<Symbol>",
            PrimitiveType::Variant => "Variant",
            PrimitiveType::Void => "<Void>",
            PrimitiveType::Unknown => "<Unknown>",
        };

        write!(f, "{}", type_name)
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for PrimitiveInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl PartialEq for PrimitiveInterface {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

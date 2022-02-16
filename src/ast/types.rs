use std::fmt;

use super::*;

use crate::types::get_interfaces_for_primitive_type;

#[derive(Debug, Clone)]
pub enum TypeVariants {
    TypeVariants(Box<TypeVariants>, TypeVariant),
    TypeVariant(TypeVariant),
}

#[derive(Debug, Clone)]
pub enum TypeVariant {
    Nested(PrimitiveType, Box<TypeVariants>),
    Enum(String, Box<VariantsDeclare>),
    Primitive(PrimitiveType),
    Interface(PrimitiveInterface),
    Symbol(String),
}

impl TypeVariant {
    pub fn implements_interface(&self, interface: PrimitiveInterface) -> bool {
        let interfaces = match self {
            TypeVariant::Primitive(primitive) => {
                get_interfaces_for_primitive_type(primitive.clone())
            }
            TypeVariant::Nested(outer, _inner) => get_interfaces_for_primitive_type(outer.clone()),
            TypeVariant::Enum(_enum_name, _variants) => todo!(),
            TypeVariant::Interface(_interface) => todo!(),
            TypeVariant::Symbol(_ident) => todo!(),
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

impl TypeVariants {
    pub fn are_assignable_to(&self, other: &Self) -> bool {
        match self {
            TypeVariants::TypeVariants(svv, sv) => {
                if let TypeVariants::TypeVariants(ovv, ov) = other {
                    sv.is_assignable_to(ov) && svv.are_assignable_to(ovv)
                } else {
                    false
                }
            }
            TypeVariants::TypeVariant(sv) => {
                if let TypeVariants::TypeVariant(ov) = other {
                    sv.is_assignable_to(ov)
                } else {
                    false
                }
            }
        }
    }

    pub fn from_vec(types: Vec<TypeVariant>) -> TypeVariants {
        match types.len() {
            1 => TypeVariants::TypeVariant(types.first().unwrap().clone()),
            _ => {
                let last = types.last().unwrap();
                let remaining = TypeVariants::from_vec(types[..types.len() - 1].to_owned());
                TypeVariants::TypeVariants(Box::new(remaining), last.clone())
            }
        }
    }
}

impl fmt::Display for TypeVariants {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeVariants::TypeVariant(s) => write!(f, "{}", s),
            TypeVariants::TypeVariants(ss, s) => write!(f, "{0}, {1}", ss, s),
        }
    }
}

impl PartialEq for TypeVariants {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            TypeVariants::TypeVariants(mvv, mv) => {
                if let TypeVariants::TypeVariants(ovv, ov) = other {
                    ovv == mvv && ov == mv
                } else {
                    false
                }
            }
            TypeVariants::TypeVariant(mv) => {
                if let TypeVariants::TypeVariant(ov) = other {
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
            TypeVariant::Symbol(_) => todo!(),
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
            TypeVariant::Symbol(i) => write!(f, "{}", i),
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
            TypeVariant::Enum(_, _) => todo!(),
            TypeVariant::Interface(_) => todo!(),
            TypeVariant::Symbol(_) => todo!(),
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

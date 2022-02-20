use std::fmt;

use super::*;

use crate::types::get_interfaces_for_primitive_type;

// TODO: Implement this as a Vec<TypeVariant> instead of a linked list.
// This should remain as a linked list in the grammar.
#[derive(Debug, Clone)]
pub enum TypeVariants {
    TypeVariants(Box<TypeVariants>, TypeVariant),
    TypeVariant(TypeVariant),
}

#[derive(Debug, Clone)]
pub enum TypeVariant {
    Nested(Type, Box<TypeVariants>),
    Enum(String, Box<VariantsDeclare>),
    Type(Type),
    Interface(PrimitiveInterface),
}

impl TypeVariant {
    pub fn implements_interface(&self, interface: PrimitiveInterface) -> bool {
        match self {
            TypeVariant::Type(the_type) => match the_type {
                Type::PrimitiveType(primitive) => {
                    get_interfaces_for_primitive_type(primitive.clone()).contains(&interface)
                }
                Type::UserDefined(_name) => {
                    if let PrimitiveInterface::IPrint = interface {
                        true
                    } else {
                        false
                    }
                }
            },
            TypeVariant::Nested(outer, _inner) => match outer {
                Type::PrimitiveType(outer) => {
                    get_interfaces_for_primitive_type(outer.clone()).contains(&interface)
                }
                Type::UserDefined(_name) => false,
            },
            TypeVariant::Enum(_enum_name, _variants) => todo!(),
            TypeVariant::Interface(_interface) => todo!(),
        }
    }
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
            TypeVariant::Type(st) => match other {
                TypeVariant::Type(ot) => st.is_assignable_to(ot),
                TypeVariant::Enum(_other_enum_name, _other_variants) => {
                    // if let PrimitiveType::Variant = sv {
                    //     println!("{}", sv);
                    //     false
                    // } else {
                    //     false
                    // }
                    false
                }
                _ => false,
            },
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
            TypeVariant::Type(t) => write!(f, "{}", t),
            TypeVariant::Enum(v, vv) => write!(f, "{0}::{1}", v, vv),
            TypeVariant::Interface(i) => write!(f, "{}", i),
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
            TypeVariant::Enum(mn, _) => {
                if let TypeVariant::Enum(on, _) = other {
                    mn == on
                } else {
                    false
                }
            }
            TypeVariant::Interface(_) => todo!(),
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
            PrimitiveType::Enum => "<Enum>",
            PrimitiveType::Exception => "<Exception>",
            PrimitiveType::Func => "Func",
            PrimitiveType::Number => "Number",
            PrimitiveType::Object => "<Object>",
            PrimitiveType::Pattern => "Pattern",
            PrimitiveType::String => "String",
            PrimitiveType::Symbol => "<Symbol>",
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

use std::fmt;

use super::*;

use crate::types::get_interfaces_for_primitive_type;

#[derive(Debug, Clone)]
pub enum TypeVariant {
    Nested(Type, Vec<TypeVariant>),
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

impl TypeVariant {
    pub fn is_assignable_to(&self, other: &Self) -> bool {
        if let TypeVariant::Interface(i) = other {
            return self.implements_interface(i.clone());
        };

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
            TypeVariant::Nested(v, vv) => {
                let children = vv
                    .iter()
                    .map(|vv| vv.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "{0}<{1}>", v, children)
            }
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

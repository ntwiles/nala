use super::*;

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

#[derive(Debug, Clone)]
pub enum PrimitiveInterface {
    ICompare,
    IEqual,
    IPrint,
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
    Any,
    Enum,
    Kind,
    Unknown,
}

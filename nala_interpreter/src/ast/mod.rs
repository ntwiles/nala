pub mod arrays;
pub mod branching;
pub mod funcs;
pub mod math;
pub mod objects;
pub mod patterns;
pub mod terms;
pub mod types;

use std::fmt;

use terms::Literal;

use crate::builtins::BuiltinFunc;

use self::arrays::*;
use self::branching::IfElseChain;
use self::branching::Match;
use self::funcs::*;
use self::math::*;
use self::objects::*;
use self::types::enum_variant::EnumVariantOrAddition;
use self::types::type_literal_variant::TypeVariantLiteral;
use self::types::variant_declare::VariantDeclare;
use self::types::StructLiteralField;

#[derive(Debug)]
pub enum Program {
    Block(Vec<Line>),
    Lines(Vec<Line>),
}

#[derive(Clone)]
pub enum FuncVariant {
    Nala(Vec<Line>),
    Builtin(BuiltinFunc),
}

impl fmt::Debug for FuncVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<Func>")
    }
}

#[derive(Debug, Clone)]
pub enum Line {
    Assign(PlaceExpression, Expr),
    Break(Expr),
    Declare(String, Expr, Option<TypeVariantLiteral>, bool),
    Enum(String, Option<String>, Vec<VariantDeclare>),
    Expr(Expr),
    For(String, Expr, Vec<Self>),
    Func(FuncDeclare),
    IfElseChain(Box<IfElseChain>),
    Wiles(Expr, Vec<Self>),
    Struct(String, Option<String>, Vec<StructLiteralField>),
    Match(Match),
}

#[derive(Debug, Clone)]
pub enum Expr {
    EnumVariant(EnumVariantOrAddition),
    Eq(Box<Expr>, EnumVariantOrAddition),
    Gt(Box<Expr>, Addition),
    Lt(Box<Expr>, Addition),
}

#[derive(Debug, Clone)]
pub enum Unary {
    Primary(Primary),
}

#[derive(Debug, Clone)]
pub enum Primary {
    Call(Call),
    Literal(Literal),
    Array(Array),
    Object(Object),
}

#[derive(Debug, Clone)]
pub enum PlaceExpression {
    Identifier(String),
    Index(Box<PlaceExpression>, Box<Expr>),
    MemberAccess(Box<PlaceExpression>, String),
}

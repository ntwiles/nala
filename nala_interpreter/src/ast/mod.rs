pub mod arrays;
pub mod branching;
pub mod funcs;
pub mod math;
pub mod objects;
pub mod patterns;
pub mod terms;
pub mod types;

use std::fmt;

use crate::builtins::BuiltinFunc;

use self::arrays::*;
use self::branching::IfElseChain;
use self::branching::Match;
use self::funcs::*;
use self::math::*;
use self::objects::*;
use self::types::enum_variant::EnumVariantOrAddend;
use self::types::type_literal_variant::TypeVariantLiteral;
use self::types::variant_declare::VariantDeclare;
use self::types::StructLiteralField;

#[derive(Debug)]
pub enum Program {
    Block(Vec<Stmt>),
    Stmts(Vec<Stmt>),
}

#[derive(Clone)]
pub enum FuncVariant {
    Nala(Vec<Stmt>),
    Builtin(BuiltinFunc),
}

impl fmt::Debug for FuncVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<Func>")
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(PlaceExpression, Expr),
    Break(Expr),
    Declare(String, Expr, Option<TypeVariantLiteral>, bool),
    Enum(String, Option<String>, Vec<VariantDeclare>),
    Expr(Expr),
    For(String, Expr, Vec<Stmt>),
    Func(FuncDeclare),
    IfElseChain(Box<IfElseChain>),
    Wiles(Expr, Vec<Stmt>),
    Struct(String, Option<String>, Vec<StructLiteralField>),
    Match(Match),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Array(Array),
    EnumVariant(EnumVariantOrAddend),
    Eq(Box<Expr>, EnumVariantOrAddend),
    Gt(Box<Expr>, Addend),
    Lt(Box<Expr>, Addend),
    Object(Object),
}

#[derive(Debug, Clone)]
pub enum PlaceExpression {
    Symbol(String),
    Index(Box<PlaceExpression>, Box<Expr>),
    MemberAccess(Box<PlaceExpression>, String),
}

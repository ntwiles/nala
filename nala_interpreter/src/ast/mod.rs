pub mod arrays;
pub mod funcs;
pub mod math;
pub mod objects;
pub mod patterns;
pub mod terms;
pub mod types;

use std::fmt;

use crate::builtins::BuiltinFunc;

use self::arrays::*;
use self::funcs::*;
use self::math::*;
use self::objects::*;
use self::patterns::*;
use self::types::*;

#[derive(Debug)]
pub enum Program {
    Block(Block),
    Stmts(Stmts),
}

#[derive(Clone)]
pub enum Block {
    NalaBlock(Stmts),
    RustBlock(BuiltinFunc),
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Block::NalaBlock(_) => write!(f, "<NalaBlock>"),
            Block::RustBlock(_) => write!(f, "<RustBlock>"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(PlaceExpression, Expr),
    Break(Expr),
    Declare(String, Expr, bool),
    PatternDeclare(String, Pattern),
    //TODO: Rename this to EnumDeclare
    Enum(String, Box<VariantsDeclare>),
    Expr(Expr),
    For(String, Expr, Box<Block>),
    Func(Func),
    If(Expr, Box<Block>),
    Wiles(Expr, Box<Block>),
}

#[derive(Debug, Clone)]
pub enum Stmts {
    Stmts(Box<Stmts>, Stmt),
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Eq(Box<Expr>, Addend),
    IsPattern(IsPattern),
    Gt(Box<Expr>, Addend),
    Lt(Box<Expr>, Addend),
    VariantValue(VariantValue),
    Array(Array),
    Object(Object),
    Unwrap(Unwrap),
}

#[derive(Debug, Clone)]
pub enum VariantsDeclare {
    Variants(Box<VariantsDeclare>, VariantDeclare),
    Variant(VariantDeclare),
}

impl fmt::Display for VariantsDeclare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantsDeclare::Variants(_variants, _variant) => todo!(),
            VariantsDeclare::Variant(variant) => write!(f, "{}", variant),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariantDeclare {
    Empty(String),
    Data(String, TypeVariant),
}

impl fmt::Display for VariantDeclare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantDeclare::Data(variant, data_type) => write!(f, "{0}({1})", variant, data_type),
            VariantDeclare::Empty(variant) => write!(f, "{}", variant),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariantValue {
    VariantValue(String, String),
    VariantValueWithData(String, String, Box<Expr>),
    Addend(Addend),
}

#[derive(Debug, Clone)]
pub enum PlaceExpression {
    Symbol(String),
    Index(String, Box<Expr>),
    MemberAccess(MemberAccess),
}
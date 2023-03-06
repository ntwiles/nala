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
use self::types::type_literal_variant::TypeLiteralVariant;
use self::types::StructLiteralField;
use self::types::TypeArgs;

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
    Enum(String, Option<TypeArgs>, Vec<VariantDeclare>),
    Expr(Expr),
    For(String, Expr, Box<Block>),
    Func(Func),
    IfElseChain(Box<IfElseChain>),
    Wiles(Expr, Box<Block>),
    Struct(String, Option<TypeArgs>, Vec<StructLiteralField>),
    Match(Match),
}

// TODO: Implement this as a Vec<Stmt> instead of a linked list.
// This should remain as a linked list in the grammar.
#[derive(Debug, Clone)]
pub enum Stmts {
    Stmts(Box<Stmts>, Stmt),
    Stmt(Stmt),
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

#[derive(Eq, Debug, Clone)]
pub enum VariantDeclare {
    Empty(String),
    Data(String, TypeLiteralVariant),
}

impl fmt::Display for VariantDeclare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantDeclare::Data(variant, data_type) => write!(f, "{0}({1})", variant, data_type),
            VariantDeclare::Empty(variant) => write!(f, "{}", variant),
        }
    }
}

impl PartialEq for VariantDeclare {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                VariantDeclare::Data(variant, data_type),
                VariantDeclare::Data(other_variant, other_data_type),
            ) => variant == other_variant && data_type == other_data_type,
            (VariantDeclare::Empty(variant), VariantDeclare::Empty(other_variant)) => {
                variant == other_variant
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlaceExpression {
    Symbol(String),
    Index(Box<PlaceExpression>, Box<Expr>),
    MemberAccess(Box<MemberAccess>),
}

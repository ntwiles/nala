pub mod arrays;
pub mod funcs;
pub mod math;
pub mod objects;
pub mod terms;
pub mod types;

use std::fmt::{Debug, Error, Formatter};

use crate::builtins::BuiltinFunc;

use arrays::*;
use funcs::*;
use math::*;
use objects::*;
use terms::*;
use types::*;

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

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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
    Eq(Box<Expr>, VariantValue),
    Gt(Box<Expr>, Addend),
    Lt(Box<Expr>, Addend),
    VariantValue(VariantValue),
    Array(Array),
    Object(Object),
}

#[derive(Debug, Clone)]
pub enum VariantsDeclare {
    Variants(Box<VariantsDeclare>, VariantDeclare),
    Variant(VariantDeclare),
}

#[derive(Debug, Clone)]
pub enum VariantDeclare {
    Empty(String),
}

#[derive(Debug, Clone)]
pub enum VariantValue {
    VariantValue(String, String),
    Addend(Addend),
}

#[derive(Debug, Clone)]
pub enum PlaceExpression {
    Symbol(String),
    Index(String, Box<Expr>),
    MemberAccess(MemberAccess),
}

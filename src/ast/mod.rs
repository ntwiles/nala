pub mod types;
pub mod terms;

use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

use crate::{
    io_context::IoContext,
    scope::{ScopeId, Scopes},
    types::get_interfaces_for_primitive_type,
};

use types::*;
use terms::*;

#[derive(Debug)]
pub enum Program {
    Block(Block),
    Stmts(Stmts),
}

#[derive(Clone)]
pub enum Block {
    NalaBlock(Stmts),
    // RustBlock is used for builtin functions.
    RustBlock(Params, BuiltinFunc),
}

impl Debug for Block {
    fn fmt(&self, _formatter: &mut Formatter) -> Result<(), Error> {
        // TODO: Implement this properly.
        Ok(())
    }
}

pub type BuiltinFunc = fn(HashMap<String, Term>, &mut Scopes, ScopeId, &mut dyn IoContext) -> Term;

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(SymbolOrIndex, Expr),
    Break(Expr),
    Declare(String, Expr, bool),
    Enum(String, Box<KindsDeclare>),
    Expr(Expr),
    For(String, Expr, Box<Block>),
    Func(String, Box<Params>, Box<Block>),
    If(Expr, Box<Block>),
    Wiles(Expr, Box<Block>),
}

#[derive(Debug, Clone)]
pub enum Stmts {
    Stmts(Box<Stmts>, Stmt),
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub enum KindsDeclare {
    Kinds(Box<KindsDeclare>, KindDeclare),
    Kind(KindDeclare),
}

#[derive(Debug, Clone)]
pub enum KindDeclare {
    Empty(String),
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elems: Box<Elems>,
}

#[derive(Debug, Clone)]
pub enum Elems {
    Elems(Box<Elems>, Expr),
    Expr(Expr),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Params {
    Params(Box<Params>, Param),
    Param(Param),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ident: String,
    pub param_type: TypeVariant,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Eq(Box<Expr>, KindValue),
    Gt(Box<Expr>, Addend),
    Lt(Box<Expr>, Addend),
    KindValue(KindValue),
    Array(Array),
}

#[derive(Debug, Clone)]
pub enum KindValue {
    KindValue(String, String),
    Addend(Addend),
}

#[derive(Debug, Clone)]
pub enum Addend {
    Add(Box<Addend>, Factor),
    Sub(Box<Addend>, Factor),
    Factor(Factor),
}

#[derive(Debug, Clone)]
pub enum Factor {
    Mult(Box<Factor>, Term),
    Div(Box<Factor>, Term),
    Call(Call),
}

#[derive(Debug, Clone)]
pub enum Call {
    Call(String, Box<Elems>),
    Index(Index),
}

#[derive(Debug, Clone)]
pub enum SymbolOrIndex {
    Symbol(String),
    Index(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Index {
    Index(String, Box<Expr>),
    Term(Term),
}

#[derive(Debug)]
pub enum OpKind {
    Add,
    Sub,
    Mult,
    Div,
}

impl Params {
    pub fn from_vec(params: Vec<Param>) -> Params {
        match params.len() {
            0 => Params::Empty,
            1 => Params::Param(params.first().unwrap().clone()),
            _ => {
                let last = params.last().unwrap();
                let remaining = Params::from_vec(params[..params.len() - 1].to_owned());
                Params::Params(Box::new(remaining), last.clone())
            }
        }
    }

    pub fn to_vec(&self) -> Vec<Param> {
        match self {
            Params::Params(params, param) => {
                let mut params = params.to_vec();
                params.push(param.to_owned());
                params
            }
            Params::Param(param) => vec![param.to_owned()],
            Params::Empty => vec![],
        }
    }
}

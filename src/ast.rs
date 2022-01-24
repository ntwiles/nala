use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

use crate::{
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

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
    Declare(String, Expr, bool),
    Assign(String, Expr),
    If(Expr, Box<Block>),
    For(String, Expr, Box<Block>),
    Func(String, Box<Params>, Box<Block>),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Stmts {
    Stmts(Box<Stmts>, Stmt),
    Stmt(Stmt),
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
    Params(Box<Params>, String),
    Param(String),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Eq(Box<Expr>, Addend),
    Gt(Box<Expr>, Addend),
    Lt(Box<Expr>, Addend),
    Addend(Addend),
    Array(Array),
}

#[derive(Debug, Clone)]
pub enum Builtin {
    Term(Term),
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
pub enum Index {
    Index(String, Box<Expr>),
    Builtin(Builtin),
}

#[derive(Debug, Clone)]
pub enum Term {
    Bool(bool),
    Symbol(String),
    String(String),
    Num(f32),
    Array(Vec<Term>),
    Func(Box<Params>, Box<Block>),
    Void,
}

#[derive(Debug)]
pub enum OpKind {
    Add,
    Sub,
    Mult,
    Div,
}

impl Term {
    pub fn to_string(&self) -> String {
        match self {
            Term::Symbol(_) => {
                panic!("Cannot know string representation of un-evaluated symbol.")
            }
            Term::String(t) => t.to_owned(),
            Term::Num(n) => n.to_string(),
            Term::Bool(b) => b.to_string(),
            Term::Array(a) => String::from(format!("[{}]", a.len())),
            Term::Func(_, _) => String::from("<Func>"),
            Term::Void => String::from("<Void>"),
        }
    }
}

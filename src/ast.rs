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
    Assign(SymbolOrIndex, Expr),
    Break(Expr),
    Declare(String, Expr, bool),
    Enum(String, Box<Kinds>),
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
pub enum Kinds {
    Kinds(Box<Kinds>, Kind),
    Kind(Kind),
}

#[derive(Debug, Clone)]
pub enum Kind {
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
    Params(Box<Params>, (String, GenericType)),
    Param(String, GenericType),
    Empty,
}

#[derive(Debug, Clone)]
pub enum GenericType {
    Generic(ValueType, Box<GenericType>),
    Primitive(ValueType),
}

#[derive(Debug, Clone)]
pub enum ValueType {
    Array,
    Bool,
    Break,
    Func,
    Number,
    String,
    Symbol,
    Void,
    Any,
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

#[derive(Debug, Clone)]
pub enum Term {
    Bool(bool),
    Symbol(String),
    String(String),
    Num(f32),
    Array(Vec<Term>),
    Func(Box<Params>, Box<Block>),
    Void,
    Break(Box<Expr>),
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
            Term::Break(_) => String::from("<Break>"),
        }
    }

    pub fn get_type(&self) -> GenericType {
        match self {
            Term::Array(items) => {
                let first = items[0].get_type();
                GenericType::Generic(ValueType::Array, Box::new(first))
            }
            Term::Bool(_) => GenericType::Primitive(ValueType::Bool),
            Term::Break(_) => GenericType::Primitive(ValueType::Break),
            Term::Func(_, _) => GenericType::Primitive(ValueType::Func),
            Term::Num(_) => GenericType::Primitive(ValueType::Number),
            Term::String(_) => GenericType::Primitive(ValueType::String),
            Term::Symbol(_) => GenericType::Primitive(ValueType::Symbol),
            Term::Void => GenericType::Primitive(ValueType::Void),
        }
    }
}

impl GenericType {
    pub fn is_assignable_to(&self, other: &Self) -> bool {
        match self {
            GenericType::Generic(sv, svv) => {
                if let GenericType::Generic(ov, ovv) = other {
                    sv.is_assignable_to(ov) && svv.is_assignable_to(ovv)
                } else {
                    false
                }
            }
            GenericType::Primitive(sv) => {
                if let GenericType::Primitive(ov) = other {
                    sv.is_assignable_to(ov)
                } else {
                    false
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            GenericType::Generic(v, vv) => format!("{0}<{1}>", v.to_string(), vv.to_string()),
            GenericType::Primitive(v) => v.to_string(),
        }
    }
}

impl PartialEq for GenericType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            GenericType::Generic(mv, mg) => {
                if let GenericType::Generic(ov, og) = other {
                    return mv == ov && mg == og;
                } else {
                    panic!("Cannot compare between generic and primitive types.")
                }
            }
            GenericType::Primitive(me) => {
                if let GenericType::Primitive(other) = other {
                    return me == other;
                } else {
                    panic!("Cannot compare between generic and primitive types.");
                }
            }
        }
    }
}

impl ValueType {
    pub fn is_assignable_to(&self, param: &Self) -> bool {
        param == &ValueType::Any || self.to_string() == param.to_string()
    }

    pub fn to_string(&self) -> String {
        let type_name = match self {
            ValueType::Array => "Array",
            ValueType::Bool => "Bool",
            ValueType::Break => "<Break>",
            ValueType::Func => "Func",
            ValueType::Number => "Number",
            ValueType::String => "String",
            ValueType::Symbol => "<Symbol>",
            ValueType::Void => "<Void>",
            ValueType::Any => "<Any>",
        };

        String::from(type_name)
    }
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

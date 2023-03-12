mod arrays;
pub mod basic;
mod branching;
mod enums;
mod functions;
mod objects;
mod operations;
mod types;
mod variables;

use crate::{
    ast::{terms::*, *},
    builtins::*,
    errors::RuntimeError,
    io_context::IoContext,
    scopes::*,
};

use self::{functions::*, variables::*};
use basic::*;

pub fn eval_tree(program: Program, ctx: &mut impl IoContext) -> Result<Value, RuntimeError> {
    let mut scopes = Scopes::new();

    let top_scope = scopes.new_scope(None);

    // Builtin functions.
    for func in get_builtins(&mut scopes, top_scope)?.into_iter() {
        if let Err(e) = eval_func(func, &mut scopes, top_scope) {
            panic!("Error loading Nala builtins: {0}", e.message)
        }
    }

    // Builtin constants.
    for (ident, value) in get_constants().iter() {
        let expr = Expr::from_value(value.clone());

        if let Err(e) = eval_declare(ident, &expr, None, false, &mut scopes, top_scope, ctx) {
            panic!("Error loading Nala constants: {0}", e.message)
        }
    }

    // TODO: What is the difference between these two?
    match program {
        Program::Block(stmts) => eval_stmts(&stmts, &mut scopes, top_scope, ctx),
        Program::Stmts(stmts) => eval_stmts(&stmts, &mut scopes, top_scope, ctx),
    }
}

pub fn eval_term(
    term: Term,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    match term {
        Term::Identifier(ident) => Ok(scopes.get_value(&ident, current_scope)?),
        Term::Value(value) => Ok(value),
    }
}

fn get_constants() -> Vec<(String, Value)> {
    let constants = vec![
        (String::from("true"), Value::Bool(true)),
        (String::from("false"), Value::Bool(false)),
    ];
    constants
}

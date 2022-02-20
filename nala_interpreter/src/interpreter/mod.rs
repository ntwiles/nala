mod arrays;
pub mod basic;
mod branching;
mod enums;
mod functions;
mod objects;
mod operations;
mod patterns;
mod variables;

use crate::{
    ast::{terms::*, *},
    builtins::*,
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::*,
};

use self::functions::*;
use self::variables::*;
use basic::*;

pub fn interpret_tree(
    program: Program,
    context: &mut impl IoContext,
) -> Result<Value, NalaRuntimeError> {
    let mut scopes = Scopes::new();

    let top_scope = scopes.new_scope(None);

    // Builtin functions.
    for func in get_builtins().iter() {
        if let Err(e) = interpret_func(&func, &mut scopes, top_scope) {
            panic!("Error loading Nala builtins: {0}", e.message)
        }
    }

    // Builtin constants.
    for (ident, value) in get_constants().iter() {
        if let Err(e) = interpret_declare(ident, &value, &mut scopes, top_scope, false) {
            panic!("Error loading Nala constants: {0}", e.message)
        }
    }

    match program {
        Program::Block(block) => interpret_block(&block, &mut scopes, top_scope, context),
        Program::Stmts(stmts) => interpret_stmts(&stmts, &mut scopes, top_scope, context),
    }
}

pub fn evaluate_term(
    term: Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match term {
        Term::Identifier(ident) => Ok(scopes.get_value(&ident, current_scope, context)?),
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

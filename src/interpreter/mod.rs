mod arrays;
pub mod basic;
mod branching;
mod enums;
mod functions;
mod objects;
mod operations;
mod variables;

use crate::{
    ast::{terms::*, *},
    builtins::*,
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::*,
};

use basic::*;
use functions::*;
use variables::*;

pub fn interpret_tree(
    program: Program,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let mut scopes = Scopes::new();

    let top_scope = scopes.new_scope(None);

    // Builtin functions.
    for func in get_builtins().iter() {
        if let Err(e) = interpret_func(&func, &mut scopes, top_scope) {
            panic!("Error loading Nala builtins: {0}", e.message)
        }
    }

    // Builtin constants.
    for (ident, term) in get_constants().iter() {
        if let Err(e) = interpret_declare(ident, &term, &mut scopes, top_scope, false) {
            panic!("Error loading Nala constants: {0}", e.message)
        }
    }

    match program {
        Program::Block(block) => interpret_block(&block, &mut scopes, top_scope, context),
        Program::Stmts(stmts) => interpret_stmts(&stmts, &mut scopes, top_scope, context),
    }
}

pub fn evaluate_if_symbol(
    sot: SymbolOrTerm,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Term, NalaRuntimeError> {
    match sot {
        SymbolOrTerm::Symbol(ident) => Ok(scopes.get_value(&ident, current_scope, context)?),
        SymbolOrTerm::Term(term) => Ok(term.clone()),
    }
}

fn get_constants() -> Vec<(String, Term)> {
    let constants = vec![
        (String::from("true"), Term::Bool(true)),
        (String::from("false"), Term::Bool(false)),
    ];
    constants
}

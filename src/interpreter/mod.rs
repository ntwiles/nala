mod arrays;
pub mod basic;
mod branching;
mod enums;
mod functions;
mod operations;
mod variables;

use crate::{
    ast::{terms::*, *},
    builtins::*,
    io_context::IoContext,
    scope::*,
};

use basic::*;
use functions::*;
use variables::*;

pub fn interpret_tree(program: Program, context: &mut impl IoContext) {
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

    let result = match program {
        Program::Block(block) => interpret_block(&block, &mut scopes, top_scope, context),
        Program::Stmts(stmts) => interpret_stmts(&stmts, &mut scopes, top_scope, context),
    };

    match result {
        Ok(_) => println!("Execution completed."),
        Err(e) => println!("Nala Runtime Error: {0}", e.message),
        _ => panic!("Passed a non-Exception value to Result::Err."),
    }
}

pub fn evaluate_if_symbol(
    sot: SymbolOrTerm,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Term {
    match sot {
        SymbolOrTerm::Symbol(ident) => scopes.get_value(&ident, current_scope, context),
        SymbolOrTerm::Term(term) => term.clone(),
    }
}

fn get_constants() -> Vec<(String, Term)> {
    let constants = vec![
        (String::from("true"), Term::Bool(true)),
        (String::from("false"), Term::Bool(false)),
    ];
    constants
}

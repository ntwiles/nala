mod arrays;
pub mod basic;
mod branching;
mod builtins;
mod enums;
mod functions;
mod operations;
mod variables;

use crate::{ast::*, io_context::IoContext, scope::Scopes};

use basic::*;
use builtins::*;
use functions::*;
use variables::*;

pub fn interpret_tree(program: Program, context: &mut impl IoContext) {
    let mut scopes = Scopes::new();

    let top_scope = scopes.new_scope(None);


    // Builtin functions.
    for (ident, block) in get_builtins().iter() {
        if let Block::RustBlock(params, _block) = block.clone() {
            interpret_func(ident, &params, &block, &mut scopes, top_scope);
        }
    }

    // Builtin constants.
    for (ident, term) in get_constants().iter() {
        interpret_declare(ident, &term, &mut scopes, top_scope, false);
    }

    match program {
        Program::Block(block) => interpret_block(&block, &mut scopes, top_scope, context),
        Program::Stmts(stmts) => interpret_stmts(&stmts, &mut scopes, top_scope, context),
    };
}

fn get_constants() -> Vec<(String, Term)> {
    let constants = vec![
        (String::from("true"), Term::Bool(true)),
        (String::from("false"), Term::Bool(false)),
    ];
    constants
}

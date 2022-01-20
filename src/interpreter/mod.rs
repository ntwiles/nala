mod arrays;
pub mod basic;
mod branching;
mod builtins;
mod functions;
mod io;
mod operations;
mod variables;

use crate::{ast::*, io_context::IoContext, scope::Scopes};

use basic::*;

pub fn interpret_tree(program: Program, context: &mut impl IoContext) {
    let mut scopes = Scopes::new();
    let top_scope = scopes.new_scope(None);
    match program {
        Program::Block(block) => interpret_block(&block, &mut scopes, top_scope, context),
        Program::Stmts(stmts) => interpret_stmts(&stmts, &mut scopes, top_scope, context),
    };
}

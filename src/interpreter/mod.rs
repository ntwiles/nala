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
use builtins::*;
use functions::*;

pub fn interpret_tree(program: Program, context: &mut impl IoContext) {
    let mut scopes = Scopes::new();

    let top_scope = scopes.new_scope(None);

    let builtins = get_builtins();

    for builtin in builtins.iter() {
        let (identifier, block) = builtin;
        if let Block::RustBlock(params, _block) = block.clone() {
            interpret_func(identifier, &params, &block, &mut scopes, top_scope);
        }
    }

    match program {
        Program::Block(block) => interpret_block(&block, &mut scopes, top_scope, context),
        Program::Stmts(stmts) => interpret_stmts(&stmts, &mut scopes, top_scope, context),
    };
}

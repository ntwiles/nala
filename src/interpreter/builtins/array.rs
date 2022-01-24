use std::collections::HashMap;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn get_len_block() -> Block {
    // TODO: Get rid of this magic string, maybe use enum?
    let params = Params::Param("array".to_string());
    Block::RustBlock(params, builtin_len)
}

fn builtin_len(
    args: HashMap<String, Term>,
    _scopes: &mut Scopes,
    _current_scope: ScopeId,
    _context: &mut dyn IoContext,
) -> Term {
    let array = args.get("array").unwrap();

    if let Term::Array(array) = array {
        Term::Num(array.len() as f32)
    } else {
        panic!("Can only pass values of type Array into len().");
    }
}

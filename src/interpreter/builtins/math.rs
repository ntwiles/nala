use std::collections::HashMap;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn get_floor_block() -> Block {
    // TODO: Get rid of this magic string, maybe use enum?
    let params = Params::Param(String::from("num"), ValueType::Number);
    Block::RustBlock(params, builtin_floor)
}
fn builtin_floor(
    args: HashMap<String, Term>,
    _scopes: &mut Scopes,
    _current_scope: ScopeId,
    _context: &mut dyn IoContext,
) -> Term {
    // TODO: Get rid of this magic string, maybe use enum?
    let num = args.get("num").unwrap();

    if let Term::Num(num) = num {
        Term::Num(num.floor())
    } else {
        panic!("Can only pass values of type Num into floor().");
    }
}

use std::collections::HashMap;

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn get_floor_block() -> Block {
    let num_param = Param {
        ident: String::from("num"),
        param_type: TypeVariant::Primitive(PrimitiveType::Number),
    };

    let params = Params::Param(num_param);
    Block::RustBlock(params, builtin_floor)
}
fn builtin_floor(
    args: HashMap<String, Term>,
    _scopes: &mut Scopes,
    _current_scope: ScopeId,
    _context: &mut dyn IoContext,
) -> Term {
    let num = args.get("num").unwrap();

    if let Term::Num(num) = num {
        Term::Num(num.floor())
    } else {
        panic!("Can only pass values of type Num into floor().");
    }
}

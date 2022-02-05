mod array;
mod io;
mod math;

use std::collections::HashMap;

use crate::{
    ast::{funcs::*, terms::*},
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use array::*;
use io::*;
use math::*;

pub type BuiltinFunc = fn(HashMap<String, Term>, &mut Scopes, ScopeId, &mut dyn IoContext) -> Term;

pub fn get_builtins() -> Vec<Func> {
    vec![
        get_floor_block(),
        get_len_block(),
        get_print_block(),
        get_read_block(),
        get_readnum_block(),
        get_slice_block(),
    ]
}

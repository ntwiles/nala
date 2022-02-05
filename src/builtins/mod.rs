mod array;
mod io;
mod math;

use std::collections::HashMap;

use crate::{
    ast::{terms::*, *},
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use array::*;
use io::*;
use math::*;

pub type BuiltinFunc = fn(HashMap<String, Term>, &mut Scopes, ScopeId, &mut dyn IoContext) -> Term;

pub fn get_builtins() -> Vec<(String, Block)> {
    vec![
        (String::from("floor"), get_floor_block()),
        (String::from("len"), get_len_block()),
        (String::from("print"), get_print_block()),
        (String::from("read"), get_read_block()),
        (String::from("readnum"), get_readnum_block()),
        (String::from("slice"), get_slice_block()),
    ]
}

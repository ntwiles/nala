mod array;
mod http;
mod io;
mod math;

use std::collections::HashMap;

use crate::{
    ast::{funcs::*, terms::*},
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
};

use self::array::*;
use self::http::*;
use self::io::*;
use self::math::*;

pub type BuiltinFunc =
    fn(HashMap<String, Value>, &mut dyn IoContext) -> Result<Value, RuntimeError>;

pub fn get_builtins(scopes: &mut Scopes, scope: usize) -> Result<Vec<Func>, RuntimeError> {
    Ok(vec![
        get_floor_block(),
        get_len_block(),
        get_print_block(),
        get_read_block(),
        get_readnum_block(),
        get_slice_block(),
        get_http_block(scopes, scope)?,
    ])
}

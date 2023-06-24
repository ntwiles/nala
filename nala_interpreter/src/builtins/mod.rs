mod array;
mod http;
mod io;
mod math;
mod util;
mod void;

use std::collections::HashMap;

use crate::resolved::func_value::FuncValue;
use crate::resolved::value::Value;
use crate::{errors::RuntimeError, io_context::IoContext};

use self::http::*;
use self::io::*;
use self::math::*;
use self::{array::*, void::get_void_block};

pub type BuiltinFunc =
    fn(HashMap<String, Value>, &mut dyn IoContext) -> Result<Value, RuntimeError>;

pub fn get_builtins() -> Vec<(String, FuncValue)> {
    vec![
        (String::from("floor"), get_floor_block()),
        (String::from("http"), get_http_block()),
        (String::from("len"), get_len_block()),
        (String::from("print"), get_print_block()),
        (String::from("read"), get_read_block()),
        (String::from("readnum"), get_readnum_block()),
        (String::from("slice"), get_slice_block()),
        (String::from("void"), get_void_block()),
    ]
}

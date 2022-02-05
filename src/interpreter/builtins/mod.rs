mod array;
mod io;
mod math;

use std::collections::HashMap;

use super::functions::*;

use crate::{
    ast::{terms::*, *},
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use array::*;
use io::*;
use math::*;

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

pub fn invoke_builtin(
    func: BuiltinFunc,
    params: &Params,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    // TODO: We already get params and args in evaluate_call, do we have to do this work again?
    let params: Vec<Param> = evaluate_params(params, scopes, current_scope, context);
    let args: HashMap<String, Term> = params
        .into_iter()
        .map(|Param { ident, .. }| (ident.clone(), scopes.get_value(&ident, current_scope)))
        .collect();

    func(args, scopes, current_scope, context)
}

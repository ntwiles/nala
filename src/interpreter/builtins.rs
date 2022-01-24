use std::collections::HashMap;

use super::functions::*;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn get_builtins() -> Vec<(String, Block)> {
    vec![
        (String::from("floor"), get_floor_block()),
        (String::from("len"), get_len_block()),
        (String::from("print"), get_print_block()),
        (String::from("read"), get_read_block()),
        (String::from("readnum"), get_readnum_block()),
    ]
}

// TODO: Once builtins are are 'natural' (using invoke_builtin) we can delete this function.
pub fn evaluate_builtin(
    builtin: &Builtin,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    _context: &mut impl IoContext,
) -> Term {
    match builtin {
        Builtin::Term(term) => {
            if let Term::Symbol(ident) = term {
                scopes.get_value(ident, current_scope)
            } else {
                term.clone()
            }
        }
    }
}

pub fn invoke_builtin(
    func: BuiltinFunc,
    params: &Params,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    // TODO: We already get params and args in evaluate_call, do we have to do this work again?
    let params: Vec<String> = evaluate_params(params, scopes, current_scope, context);
    let args: HashMap<String, Term> = params
        .into_iter()
        .map(|param| (param.clone(), scopes.get_value(&param, current_scope)))
        .collect();

    func(args, scopes, current_scope, context)
}

fn get_floor_block() -> Block {
    // TODO: Get rid of this magic string, maybe use enum?
    let params = Params::Param("num".to_string());
    Block::RustBlock(params, builtin_floor)
}

fn get_len_block() -> Block {
    // TODO: Get rid of this magic string, maybe use enum?
    let params = Params::Param("array".to_string());
    Block::RustBlock(params, builtin_len)
}

fn get_print_block() -> Block {
    // TODO: Get rid of this magic string, maybe use enum?
    let params = Params::Param("message".to_string());
    Block::RustBlock(params, builtin_print)
}

fn get_read_block() -> Block {
    Block::RustBlock(Params::Empty, builtin_read)
}

fn get_readnum_block() -> Block {
    Block::RustBlock(Params::Empty, builtin_readnum)
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

fn builtin_print(
    args: HashMap<String, Term>,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Term {
    let message = args.get("message").unwrap();

    if let Term::Symbol(ident) = message {
        context.print(&scopes.get_value(&ident, current_scope).to_string());
    } else {
        context.print(&message.to_string());
    }

    Term::Void
}

fn builtin_read(
    _args: HashMap<String, Term>,
    _scopes: &mut Scopes,
    _current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Term {
    let input = context.read();
    Term::String(input.trim().to_string())
}

fn builtin_readnum(
    _args: HashMap<String, Term>,
    _scopes: &mut Scopes,
    _current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Term {
    let mut input = context.read();
    input = input.trim().to_string();
    let result = input.parse::<f32>();
    match result {
        Ok(num) => Term::Num(num),
        Err(_) => panic!("Could not parse input '{}' as type Num.", input),
    }
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

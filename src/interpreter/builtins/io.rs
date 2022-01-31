use std::collections::HashMap;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn get_print_block() -> Block {
    let params = Params::Param(Param {
        ident: String::from("message"),
        param_type: Type::Primitive(PrimitiveType::Any),
    });

    Block::RustBlock(params, builtin_print)
}

pub fn get_read_block() -> Block {
    Block::RustBlock(Params::Empty, builtin_read)
}

pub fn get_readnum_block() -> Block {
    Block::RustBlock(Params::Empty, builtin_readnum)
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

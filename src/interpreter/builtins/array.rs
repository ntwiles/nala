use std::collections::HashMap;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::build_params;

pub fn get_len_block() -> Block {
    // TODO: Get rid of this magic string, maybe use enum?
    let params = Params::Param(Param {
        ident: String::from("array"),
        param_type: Type::Nested(
            PrimitiveType::Array,
            Box::new(Type::Primitive(PrimitiveType::Number)),
        ),
    });

    Block::RustBlock(params, builtin_len)
}

pub fn get_slice_block() -> Block {
    // TODO: Get rid of these magic strings, maybe use enum?
    let params = build_params(vec![
        Param {
            ident: String::from("array"),
            param_type: Type::Nested(
                PrimitiveType::Array,
                Box::new(Type::Primitive(PrimitiveType::Number)),
            ),
        },
        Param {
            ident: String::from("start"),
            param_type: Type::Primitive(PrimitiveType::Number),
        },
        Param {
            ident: String::from("end"),
            param_type: Type::Primitive(PrimitiveType::Number),
        },
    ]);

    Block::RustBlock(params, builtin_slice)
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
        // TODO: Centralize these kinds of errors.
        panic!("Can only pass values of type Array into len().");
    }
}

fn builtin_slice(
    args: HashMap<String, Term>,
    _scopes: &mut Scopes,
    _current_scope: ScopeId,
    _context: &mut dyn IoContext,
) -> Term {
    let array = if let Term::Array(array) = args.get("array").unwrap() {
        array
    } else {
        // TODO: Centralize these kinds of errors.
        panic!("Can only pass values of type Array into slice().");
    };

    let start = if let Term::Num(start) = args.get("start").unwrap() {
        *start as usize
    } else {
        // TODO: Centralize these kinds of errors.
        panic!("Can only pass values of type Array into slice().");
    };

    let end = if let Term::Num(end) = args.get("end").unwrap() {
        *end as usize
    } else {
        // TODO: Centralize these kinds of errors.
        panic!("Can only pass values of type Array into slice().");
    };

    Term::Array(array[start..end].to_owned())
}

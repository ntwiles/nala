use std::{collections::HashMap};

use crate::{
    ast::{types::*, terms::*, *},
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn get_len_block() -> Block {
    let inner_type = Types::Type(
        TypeVariant::Primitive(PrimitiveType::Number)
    );

    let outer_type =TypeVariant::Nested(
        PrimitiveType::Array,
        Box::new(inner_type)
    );

    let params = Params::Param(Param {
        ident: String::from("array"),
        param_type: outer_type
    });

    Block::RustBlock(params, builtin_len)
}

pub fn get_slice_block() -> Block {
    let array_param = Param {
        ident: String::from("array"),
        param_type: TypeVariant::Nested(
            PrimitiveType::Array,
            Box::new(Types::Type(TypeVariant::Primitive(PrimitiveType::Number)))
        )};

    let start_param = Param {
        ident: String::from("start"),
        param_type:  TypeVariant::Primitive(PrimitiveType::Number),
    };

    let end_param = Param {
        ident: String::from("end"),
        param_type: TypeVariant::Primitive(PrimitiveType::Number)
    };

    let params = Params::from_vec(vec![
        array_param,
        start_param,
        end_param
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

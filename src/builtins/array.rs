use std::collections::HashMap;

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    io_context::IoContext,
};

use super::*;

pub fn get_len_block() -> Func {
    let inner_type = Types::Type(TypeVariant::Primitive(PrimitiveType::Number));

    let outer_type = TypeVariant::Nested(PrimitiveType::Array, Box::new(inner_type));

    let params = Params::Param(Param {
        ident: String::from("array"),
        param_type: outer_type,
    });

    Func {
        ident: "len".to_string(),
        params: Box::new(params),
        block: Box::new(Block::RustBlock(builtin_len)),
    }
}

pub fn get_slice_block() -> Func {
    let array_param = Param {
        ident: String::from("array"),
        param_type: TypeVariant::Nested(
            PrimitiveType::Array,
            Box::new(Types::Type(TypeVariant::Primitive(PrimitiveType::Number))),
        ),
    };

    let start_param = Param {
        ident: String::from("start"),
        param_type: TypeVariant::Primitive(PrimitiveType::Number),
    };

    let end_param = Param {
        ident: String::from("end"),
        param_type: TypeVariant::Primitive(PrimitiveType::Number),
    };

    let params = Params::from_vec(vec![array_param, start_param, end_param]);

    Func {
        ident: "slice".to_string(),
        params: Box::new(params),
        block: Box::new(Block::RustBlock(builtin_slice)),
    }
}

fn builtin_len(args: HashMap<String, Term>, _context: &mut dyn IoContext) -> Term {
    let array = args.get("array").unwrap();

    if let Term::Array(array) = array {
        Term::Num(array.len() as f32)
    } else {
        panic_bad_args("len");
    }
}

fn builtin_slice(args: HashMap<String, Term>, _context: &mut dyn IoContext) -> Term {
    let array = if let Term::Array(array) = args.get("array").unwrap() {
        array
    } else {
        panic_bad_args("slice")
    };

    let start = if let Term::Num(start) = args.get("start").unwrap() {
        *start as usize
    } else {
        panic_bad_args("slice")
    };

    let end = if let Term::Num(end) = args.get("end").unwrap() {
        *end as usize
    } else {
        panic_bad_args("slice")
    };

    Term::Array(array[start..end].to_owned())
}
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    io_context::IoContext,
};

pub fn get_len_block() -> Func {
    let inner_type = TypeVariants::TypeVariant(TypeVariant::Type(Type::PrimitiveType(
        PrimitiveType::Number,
    )));

    let outer_type = TypeVariant::Nested(
        Type::PrimitiveType(PrimitiveType::Array),
        Box::new(inner_type),
    );

    let params = Params::Param(Param {
        ident: String::from("array"),
        param_type: outer_type,
    });

    Func {
        ident: "len".to_string(),
        params: Some(params),
        block: Box::new(Block::RustBlock(builtin_len)),
    }
}

pub fn get_slice_block() -> Func {
    let array_param = Param {
        ident: String::from("array"),
        param_type: TypeVariant::Nested(
            Type::PrimitiveType(PrimitiveType::Array),
            Box::new(TypeVariants::TypeVariant(TypeVariant::Type(
                Type::PrimitiveType(PrimitiveType::Number),
            ))),
        ),
    };

    let start_param = Param {
        ident: String::from("start"),
        param_type: TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
    };

    let end_param = Param {
        ident: String::from("end"),
        param_type: TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
    };

    let params = Params::from_vec(vec![array_param, start_param, end_param]);

    Func {
        ident: "slice".to_string(),
        params: Some(params),
        block: Box::new(Block::RustBlock(builtin_slice)),
    }
}

fn builtin_len(args: HashMap<String, Value>, _context: &mut dyn IoContext) -> Value {
    let array = args.get("array").unwrap();

    if let Value::Array(array) = array {
        let array = Arc::clone(array);
        let array = array.lock().unwrap();
        Value::Num(array.len() as f32)
    } else {
        unreachable!()
    }
}

fn builtin_slice(args: HashMap<String, Value>, _context: &mut dyn IoContext) -> Value {
    let array = if let Value::Array(array) = args.get("array").unwrap() {
        array
    } else {
        unreachable!()
    };

    let start = if let Value::Num(start) = args.get("start").unwrap() {
        *start as usize
    } else {
        unreachable!()
    };

    let end = if let Value::Num(end) = args.get("end").unwrap() {
        *end as usize
    } else {
        unreachable!()
    };

    let array = Arc::clone(array);
    let array = array.lock().unwrap();

    Value::Array(Arc::new(Mutex::new(array[start..end].to_owned())))
}

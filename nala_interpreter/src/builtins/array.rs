use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    ast::{
        funcs::*,
        terms::*,
        types::{
            primitive_type::PrimitiveType, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant,
        },
        *,
    },
    errors::RuntimeError,
    io_context::IoContext,
};

pub fn get_len_block() -> Func {
    let inner_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Any));

    let outer_type = TypeLiteralVariant::Composite(
        TypeLiteral::PrimitiveType(PrimitiveType::Array),
        vec![inner_type],
    );

    let params = vec![Param {
        ident: String::from("array"),
        param_type: outer_type,
    }];

    let return_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Number));

    Func {
        ident: "len".to_string(),
        params,
        return_type,
        block: Box::new(FuncVariant::Builtin(builtin_len)),
    }
}

pub fn get_slice_block() -> Func {
    let inner_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Any));

    let outer_type = TypeLiteralVariant::Composite(
        TypeLiteral::PrimitiveType(PrimitiveType::Array),
        vec![inner_type],
    );

    let array_param = Param {
        ident: String::from("array"),
        param_type: outer_type,
    };

    let start_param = Param {
        ident: String::from("start"),
        param_type: TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Number)),
    };

    let end_param = Param {
        ident: String::from("end"),
        param_type: TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Number)),
    };

    let inner_return_type =
        TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Any));

    let return_type = TypeLiteralVariant::Composite(
        TypeLiteral::PrimitiveType(PrimitiveType::Array),
        vec![inner_return_type],
    );

    Func {
        ident: "slice".to_string(),
        params: vec![array_param, start_param, end_param],
        return_type,
        block: Box::new(FuncVariant::Builtin(builtin_slice)),
    }
}

fn builtin_len(
    args: HashMap<String, Value>,
    _context: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let array = args.get("array").unwrap();

    if let Value::Array(array) = array {
        let array = Arc::clone(array);
        let array = array.lock().unwrap();
        Ok(Value::Num(array.len() as f32))
    } else {
        unreachable!()
    }
}

fn builtin_slice(
    args: HashMap<String, Value>,
    _context: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
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

    Ok(Value::Array(Arc::new(Mutex::new(
        array[start..end].to_owned(),
    ))))
}

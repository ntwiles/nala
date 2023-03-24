use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    ast::{terms::*, types::primitive_type::PrimitiveType, *},
    errors::RuntimeError,
    io_context::IoContext,
    types::{type_variant::TypeVariant, NalaType},
};

pub fn get_len_block() -> FuncValue {
    let inner_type = TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Any));

    let outer_type = TypeVariant::Composite(
        NalaType::PrimitiveType(PrimitiveType::Array),
        vec![inner_type],
    );

    let params = vec![Param {
        ident: String::from("array"),
        param_type: outer_type,
    }];

    let return_type = TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number));

    FuncValue {
        params,
        return_type,
        type_params: None,
        closure_scope: 0,
        block: Box::new(FuncVariant::Builtin(builtin_len)),
    }
}

pub fn get_slice_block() -> FuncValue {
    let inner_type = TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Any));

    let outer_type = TypeVariant::Composite(
        NalaType::PrimitiveType(PrimitiveType::Array),
        vec![inner_type],
    );

    let array_param = Param {
        ident: String::from("array"),
        param_type: outer_type,
    };

    let start_param = Param {
        ident: String::from("start"),
        param_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
    };

    let end_param = Param {
        ident: String::from("end"),
        param_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
    };

    let inner_return_type = TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Any));

    let return_type = TypeVariant::Composite(
        NalaType::PrimitiveType(PrimitiveType::Array),
        vec![inner_return_type],
    );

    FuncValue {
        params: vec![array_param, start_param, end_param],
        return_type,
        type_params: None,
        closure_scope: 0,
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

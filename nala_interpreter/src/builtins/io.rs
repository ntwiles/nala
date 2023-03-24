use std::collections::HashMap;

use crate::{
    ast::{terms::*, types::primitive_type::PrimitiveType, *},
    errors::RuntimeError,
    io_context::IoContext,
    types::{type_variant::TypeVariant, NalaType},
};

pub fn get_print_block() -> FuncValue {
    let message_param = Param {
        ident: String::from("message"),
        param_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Any)),
    };

    let return_type = TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Void));

    FuncValue {
        params: vec![message_param],
        return_type,
        type_params: None,
        closure_scope: 0,
        block: Box::new(FuncVariant::Builtin(builtin_print)),
    }
}

pub fn get_read_block() -> FuncValue {
    let return_type = TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String));

    FuncValue {
        params: vec![],
        return_type,
        type_params: None,
        closure_scope: 0,
        block: Box::new(FuncVariant::Builtin(builtin_read)),
    }
}

pub fn get_readnum_block() -> FuncValue {
    let return_type = TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number));

    FuncValue {
        params: vec![],
        return_type,
        type_params: None,
        closure_scope: 0,
        block: Box::new(FuncVariant::Builtin(builtin_readnum)),
    }
}

fn builtin_print(
    args: HashMap<String, Value>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let message = args.get("message").unwrap();
    ctx.print(&message.to_string());
    Ok(Value::Void)
}

fn builtin_read(
    _args: HashMap<String, Value>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let input = ctx.read();
    Ok(Value::String(input.trim().to_string()))
}

fn builtin_readnum(
    _args: HashMap<String, Value>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let input = ctx.read();

    input
        .trim()
        .to_string()
        .parse::<f32>()
        .map(|num| Value::Num(num))
        .map_err(|_| RuntimeError::new(&format!("Could not parse input `{input}` as type Number.")))
}

use std::collections::HashMap;

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

pub fn get_print_block() -> Func {
    let message_param = Param {
        ident: String::from("message"),
        param_type: TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::String)),
    };

    let return_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Void));

    Func {
        ident: "print".to_string(),
        params: vec![message_param],
        return_type,
        block: Box::new(Block::RustBlock(builtin_print)),
    }
}

pub fn get_read_block() -> Func {
    let return_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::String));

    Func {
        ident: "read".to_string(),
        params: vec![],
        return_type,
        block: Box::new(Block::RustBlock(builtin_read)),
    }
}

pub fn get_readnum_block() -> Func {
    let return_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Number));

    Func {
        ident: "readnum".to_string(),
        params: vec![],
        return_type,
        block: Box::new(Block::RustBlock(builtin_readnum)),
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
    let mut input = ctx.read();

    input = input.trim().to_string();
    let result = input.parse::<f32>();

    // TODO: Replace panic with RuntimeError.
    match result {
        Ok(num) => Ok(Value::Num(num)),
        Err(_) => panic!("Could not parse input '{}' as type Num.", input),
    }
}

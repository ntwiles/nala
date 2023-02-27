use std::collections::HashMap;

use crate::{
    ast::{
        funcs::{Func, Param},
        terms::Value,
        types::{
            primitive_type::PrimitiveType, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant,
        },
        Block,
    },
    errors::RuntimeError,
    io_context::IoContext,
};

// TODO: Write tests for void function.
pub fn get_void_block() -> Func {
    let param = Param {
        ident: String::from("_"),
        param_type: TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Number)),
    };

    let return_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Void));

    Func {
        ident: "void".to_string(),
        params: vec![param],
        return_type,
        block: Box::new(Block::RustBlock(builtin_void)),
    }
}

fn builtin_void(
    _args: HashMap<String, Value>,
    _ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    Ok(Value::Void)
}

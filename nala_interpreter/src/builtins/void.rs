use std::collections::HashMap;

use crate::{
    ast::{
        funcs::{Func, Param},
        terms::Value,
        types::{
            primitive_type::PrimitiveType, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant,
        },
        FuncVariant,
    },
    errors::RuntimeError,
    io_context::IoContext,
};

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
        block: Box::new(FuncVariant::Builtin(builtin_void)),
    }
}

fn builtin_void(
    _args: HashMap<String, Value>,
    _ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    Ok(Value::Void)
}

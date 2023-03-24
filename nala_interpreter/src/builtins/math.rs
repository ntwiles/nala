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

pub fn get_floor_block() -> Func {
    let num_param = Param {
        ident: String::from("num"),
        param_type: TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Number)),
    };

    let return_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Number));

    Func {
        ident: "floor".to_string(),
        params: vec![num_param],
        return_type,
        type_params: None,
        block: Box::new(FuncVariant::Builtin(builtin_floor)),
    }
}

fn builtin_floor(
    args: HashMap<String, Value>,
    _context: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let num = args.get("num").unwrap();

    if let Value::Num(num) = num {
        Ok(Value::Num(num.floor()))
    } else {
        unreachable!()
    }
}

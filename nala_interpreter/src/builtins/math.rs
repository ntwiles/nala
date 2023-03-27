use std::collections::HashMap;

use crate::{
    ast::{types::primitive_type::PrimitiveType, *},
    errors::RuntimeError,
    io_context::IoContext,
    resolved::{
        func_value::{FuncValue, Param},
        value::Value,
    },
    types::{nala_type::NalaType, type_variant::TypeVariant},
};

pub fn get_floor_block() -> FuncValue {
    let num_param = Param {
        ident: String::from("num"),
        param_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
    };

    let return_type = TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number));

    FuncValue {
        params: vec![num_param],
        return_type,
        type_param: None,
        closure_scope: 0,
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

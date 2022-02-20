use std::collections::HashMap;

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    io_context::IoContext,
};

pub fn get_floor_block() -> Func {
    let num_param = Param {
        ident: String::from("num"),
        param_type: TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
    };

    let params = Params::Param(num_param);

    Func {
        ident: "floor".to_string(),
        params: Some(params),
        block: Box::new(Block::RustBlock(builtin_floor)),
    }
}

fn builtin_floor(args: HashMap<String, Value>, _context: &mut dyn IoContext) -> Value {
    let num = args.get("num").unwrap();

    if let Value::Num(num) = num {
        Value::Num(num.floor())
    } else {
        unreachable!()
    }
}

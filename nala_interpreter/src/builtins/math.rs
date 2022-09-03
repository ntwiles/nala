use std::collections::HashMap;

use crate::{
    ast::{
        funcs::*,
        terms::*,
        types::{nala_type::NalaType, primitive_type::PrimitiveType, type_variant::TypeVariant},
        *,
    },
    io_context::IoContext,
};

pub fn get_floor_block() -> Func {
    let num_param = Param {
        ident: String::from("num"),
        param_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
    };

    Func {
        ident: "floor".to_string(),
        params: vec![num_param],
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

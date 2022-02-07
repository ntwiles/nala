use std::collections::HashMap;

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    io_context::IoContext,
};

use super::*;

pub fn get_floor_block() -> Func {
    let num_param = Param {
        ident: String::from("num"),
        param_type: TypeVariant::Primitive(PrimitiveType::Number),
    };

    let params = Params::Param(num_param);

    Func {
        ident: "floor".to_string(),
        params: Box::new(params),
        block: Box::new(Block::RustBlock(builtin_floor)),
    }
}

fn builtin_floor(args: HashMap<String, Term>, _context: &mut dyn IoContext) -> Term {
    let num = args.get("num").unwrap();

    if let Term::Num(num) = num {
        Term::Num(num.floor())
    } else {
        panic_bad_args("floor")
    }
}

use std::collections::HashMap;

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    io_context::IoContext,
};

pub fn get_print_block() -> Func {
    let message_param = Param {
        ident: String::from("message"),
        param_type: TypeVariant::Interface(PrimitiveInterface::IPrint),
    };

    let params = Params::Param(message_param);

    Func {
        ident: "print".to_string(),
        params: Some(params),
        block: Box::new(Block::RustBlock(builtin_print)),
    }
}

pub fn get_read_block() -> Func {
    Func {
        ident: "read".to_string(),
        params: Some(Params::Empty),
        block: Box::new(Block::RustBlock(builtin_read)),
    }
}

pub fn get_readnum_block() -> Func {
    Func {
        ident: "readnum".to_string(),
        params: Some(Params::Empty),
        block: Box::new(Block::RustBlock(builtin_readnum)),
    }
}

fn builtin_print(args: HashMap<String, Term>, context: &mut dyn IoContext) -> Term {
    let message = args.get("message").unwrap();
    context.print(&message.to_string());
    Term::Void
}

fn builtin_read(_args: HashMap<String, Term>, context: &mut dyn IoContext) -> Term {
    let input = context.read();
    Term::String(input.trim().to_string())
}

fn builtin_readnum(_args: HashMap<String, Term>, context: &mut dyn IoContext) -> Term {
    let mut input = context.read();

    input = input.trim().to_string();
    let result = input.parse::<f32>();

    match result {
        Ok(num) => Term::Num(num),
        Err(_) => panic!("Could not parse input '{}' as type Num.", input),
    }
}

use crate::{ast::*, io_context::IoContext};

pub fn evaluate_read(context: &mut impl IoContext) -> Term {
    let input = context.read();
    Term::String(input.trim().to_string())
}

pub fn evaluate_readnum(context: &mut impl IoContext) -> Term {
    let mut input = context.read();
    input = input.trim().to_string();
    let result = input.parse::<f32>();
    match result {
        Ok(num) => Term::Num(num),
        Err(_) => panic!("Could not parse input '{}' as type Num.", input),
    }
}

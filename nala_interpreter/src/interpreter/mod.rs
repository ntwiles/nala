mod arrays;
pub mod basic;
mod branching;
pub mod enums;
mod functions;
mod objects;
mod operations;
mod types;
mod variables;

use crate::{
    ast::{terms::*, *},
    builtin_types::{get_builtin_enums, get_builtin_structs},
    builtins::*,
    errors::RuntimeError,
    io_context::IoContext,
    resolved::value::Value,
    scopes::*,
};

use self::{
    functions::*,
    types::{eval_enum, eval_struct},
    variables::*,
};
use basic::*;

pub fn eval_program(program: Program, ctx: &mut impl IoContext) -> Result<Value, RuntimeError> {
    let mut scopes = Scopes::new();
    let top_scope = scopes.new_scope(None);

    load_builtin_types(&mut scopes, top_scope)?;
    load_builtin_constants(&mut scopes, top_scope);
    load_builtin_functions(&mut scopes, top_scope)?;

    match program {
        Program::Block(lines) => eval_lines(&lines, &mut scopes, top_scope, ctx),
        Program::Lines(lines) => eval_lines(&lines, &mut scopes, top_scope, ctx),
    }
}

pub fn eval_term(
    term: Term,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    match term {
        Term::Identifier(ident) => Ok(scopes.get_value(&ident, current_scope)?),
        Term::Literal(value) => match value {
            Literal::Number(value) => Ok(Value::Num(value)),
            Literal::String(value) => Ok(Value::String(value)),
        },
    }
}

fn load_builtin_types(scopes: &mut Scopes, current_scope: usize) -> Result<(), RuntimeError> {
    // TODO: This is going to quickly become problematic. Even with only two builtin types,
    // HttpResult<T> is dependent on Option<T> and needs to be loaded first. In this very simple
    // case, we can fix this just by loading enums before we load structs, but this is not a general
    // solution. This will/should probably remain unsolved until implementing a package/module
    // system.

    for (ident, type_param, variants) in get_builtin_enums() {
        if let Err(e) = eval_enum(&ident, type_param, variants, scopes, current_scope) {
            panic!("Error loading builtin enums: {0}", e.message)
        }
    }

    for (ident, type_param, fields) in get_builtin_structs() {
        if let Err(e) = eval_struct(&ident, type_param, fields, scopes, current_scope) {
            panic!("Error loading builtin structs: {0}", e.message)
        }
    }

    Ok(())
}

fn load_builtin_constants(scopes: &mut Scopes, top_scope: usize) {
    for (ident, value) in vec![
        (String::from("true"), Value::Bool(true)),
        (String::from("false"), Value::Bool(false)),
    ]
    .into_iter()
    {
        if let Err(e) = eval_declare(&ident, value, None, false, scopes, top_scope) {
            panic!("Error loading builtin constants: {0}", e.message)
        }
    }
}

fn load_builtin_functions(scopes: &mut Scopes, top_scope: usize) -> Result<(), RuntimeError> {
    for (ident, func) in get_builtins().into_iter() {
        if let Err(e) = eval_builtin_declare(ident, func, scopes, top_scope) {
            panic!("Error loading builtin functions: {0}", e.message)
        }
    }

    Ok(())
}

mod arrays;
pub mod basic;
mod branching;
mod enums;
mod functions;
mod objects;
mod operations;
mod types;
mod variables;

use crate::{
    ast::{
        terms::*,
        types::{
            type_literal::TypeLiteral, type_literal_variant::TypeVariantLiteral,
            variant_declare::VariantDeclare,
        },
        *,
    },
    builtins::*,
    errors::RuntimeError,
    io_context::IoContext,
    resolved::value::Value,
    scopes::*,
};

use self::{functions::*, types::eval_enum, variables::*};
use basic::*;

pub fn eval_program(program: Program, ctx: &mut impl IoContext) -> Result<Value, RuntimeError> {
    let mut scopes = Scopes::new();
    let top_scope = scopes.new_scope(None);

    load_builtin_types(&mut scopes, top_scope)?;
    load_builtin_constants(&mut scopes, top_scope);
    load_builtin_functions(&mut scopes, top_scope)?;

    match program {
        Program::Block(stmts) => eval_stmts(&stmts, &mut scopes, top_scope, ctx),
        Program::Stmts(stmts) => eval_stmts(&stmts, &mut scopes, top_scope, ctx),
    }
}

pub fn eval_term(
    term: Term,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    match term {
        Term::Identifier(ident) => Ok(scopes.get_value(&ident, current_scope)?),
        Term::ValueLiteral(value) => match value {
            ValueLiteral::Number(value) => Ok(Value::Num(value)),
            ValueLiteral::String(value) => Ok(Value::String(value)),
        },
    }
}

fn load_builtin_types(scopes: &mut Scopes, current_scope: usize) -> Result<(), RuntimeError> {
    let type_params = Some(String::from("T"));
    let variants = vec![
        VariantDeclare::Data(
            String::from("Some"),
            TypeVariantLiteral::Type(TypeLiteral::UserDefined(String::from("T"))),
        ),
        VariantDeclare::Empty(String::from("None")),
    ];

    if let Err(e) = eval_enum("Option", type_params, variants, scopes, current_scope) {
        panic!("Error loading builtin types: {0}", e.message)
    }

    Ok(())
}

fn load_builtin_constants(scopes: &mut Scopes, top_scope: usize) {
    for (ident, value) in vec![
        (String::from("true"), Value::Bool(true)),
        (String::from("false"), Value::Bool(false)),
    ]
    .iter()
    {
        if let Err(e) = eval_declare(ident, &value, None, false, scopes, top_scope) {
            panic!("Error loading builtin constants: {0}", e.message)
        }
    }
}

fn load_builtin_functions(scopes: &mut Scopes, top_scope: usize) -> Result<(), RuntimeError> {
    for (ident, func) in get_builtins(scopes, top_scope)?.into_iter() {
        if let Err(e) = eval_builtin_declare(ident, func, scopes, top_scope) {
            panic!("Error loading builtin functions: {0}", e.message)
        }
    }

    Ok(())
}

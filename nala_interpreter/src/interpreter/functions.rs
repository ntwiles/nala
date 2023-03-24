use std::collections::HashMap;

use super::{basic::*, variables::*};

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
    errors::*,
    io_context::IoContext,
    scopes::Scopes,
    types::{fit::fits_type, inference::infer_type, type_variant::TypeVariant},
};

pub fn eval_func(
    func: Func,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let Func {
        ident,
        block,
        params,
        return_type,
        type_params: _,
    } = func;

    check_param_types(&params, scopes, current_scope)?;

    let closure_scope = scopes.new_scope(Some(current_scope));

    scopes.add_binding(
        &ident,
        Value::Func(FuncValue {
            params,
            return_type,
            block,
            closure_scope,
        }),
        None,
        current_scope,
        false,
    )
}

fn check_param_types(
    params: &Vec<Param>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<(), RuntimeError> {
    let mut results = params
        .iter()
        .map(|p| check_param_type(&p.param_type, scopes, current_scope));

    if let Some(Err(err)) = results.find(|r| r.is_err()) {
        Err(err)
    } else {
        Ok(())
    }
}

fn check_param_type(
    param_type: &TypeLiteralVariant,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<(), RuntimeError> {
    if let TypeLiteralVariant::Composite(outer, inner) = param_type {
        match outer {
            TypeLiteral::PrimitiveType(outer) => match outer {
                PrimitiveType::Array => Ok(()),
                PrimitiveType::Func => Ok(()),
                _ => Err(type_args_not_supported_error(outer.to_string(), inner)),
            },
            TypeLiteral::UserDefined(ident) => {
                let binding = scopes.get_type(&ident, current_scope)?;

                if binding.get_generic_ident().is_some() {
                    Ok(())
                } else {
                    Err(type_args_not_supported_error(outer.to_string(), inner))
                }
            }
        }
    } else {
        Ok(())
    }
}

pub fn eval_call(
    call: &Call,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match call {
        Call::Call(place, args) => {
            let block = eval_place_expr(place, scopes, current_scope, ctx)?;

            if let Value::Func(FuncValue {
                params,
                block,
                closure_scope,
                return_type: expected_return_type,
            }) = block
            {
                let args = eval_elems(&*args, scopes, current_scope, ctx)?;

                if params.len() != args.len() {
                    return Err(RuntimeError::new(&format!(
                        "Called function with wrong number of arguments: Expected {0}, got {1}.",
                        params.len(),
                        args.len()
                    )));
                }

                let call_scope = scopes.new_scope(Some(closure_scope));

                let mut param_args: HashMap<String, Value> = HashMap::new();

                for (i, param) in params.iter().enumerate() {
                    let arg = args.get(i).unwrap();

                    let param_type =
                        TypeVariant::from_literal(param.param_type.clone(), scopes, current_scope)?;

                    if !fits_type(arg, &param_type, scopes, current_scope)? {
                        return Err(wrong_arg_type_for_param_error(
                            arg,
                            infer_type(&arg, scopes, current_scope)?.to_string(),
                            param_type.to_string(),
                        ));
                    }

                    scopes.add_binding(&param.ident, arg.clone(), None, call_scope, false)?;
                    param_args.entry(param.ident.clone()).or_insert(arg.clone());
                }

                let return_value = match *block {
                    FuncVariant::Nala(stmts) => eval_stmts(&stmts, scopes, call_scope, ctx)?,
                    FuncVariant::Builtin(func) => func(param_args, ctx)?,
                };

                let expected_return_type =
                    TypeVariant::from_literal(expected_return_type, scopes, current_scope)?;

                if fits_type(&return_value, &expected_return_type, scopes, current_scope)? {
                    Ok(return_value)
                } else {
                    Err(RuntimeError::new(&format!("Tried to return value `{return_value:?}` of type `{0}` where value of type `{expected_return_type}` was expected.", infer_type(&return_value, scopes, current_scope)?)))
                }
            } else {
                Err(RuntimeError::new(&format!("Cannot invoke a non-function.")))
            }
        }
        Call::PlaceExpression(place) => eval_place_expr(place, scopes, current_scope, ctx),
        Call::Value(value) => Ok(value.clone()),
    }
}

fn type_args_not_supported_error(outer: String, inner: &Vec<TypeLiteralVariant>) -> RuntimeError {
    let inner = inner
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    RuntimeError::new(&format!(
        "Type `{outer}` does not support type arguments. Type `{outer}<{inner}>` is invalid."
    ))
}

fn wrong_arg_type_for_param_error(
    arg_value: &Value,
    arg_type: String,
    param_type: String,
) -> RuntimeError {
    RuntimeError::new(&format!(
        "Passed value `{arg_value:?}` of type `{arg_type}` to function where `{param_type}` was expected.")
    )
}

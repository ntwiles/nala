use std::collections::HashMap;

use super::{basic::*, variables::*};

use crate::{
    ast::{funcs::*, terms::*, types::type_literal_variant::TypeLiteralVariant, *},
    errors::*,
    io_context::IoContext,
    scopes::{type_binding::TypeBinding, Scopes},
    types::{fit::fits_type, inference::infer_type, type_variant::TypeVariant},
    utils::accept_results,
};

pub fn eval_func_declare(
    func: FuncDeclare,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let FuncDeclare {
        ident,
        block,
        params,
        return_type,
        type_params,
    } = func;

    let closure_scope = scopes.new_scope(Some(current_scope));

    if let Some(type_param) = &type_params {
        scopes.add_type_binding(
            &type_param,
            closure_scope,
            TypeBinding::Generic(type_param.clone()),
        )?;
    };

    let params = check_param_types(&params, scopes, closure_scope)?;
    let return_type = TypeVariant::from_literal(return_type, scopes, current_scope)?;

    scopes.add_binding(
        &ident,
        Value::Func(FuncValue {
            block,
            params,
            return_type,
            type_params,
            closure_scope,
        }),
        None,
        current_scope,
        false,
    )
}

pub fn eval_builtin_declare(
    ident: String,
    func: FuncValue,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let FuncValue {
        block,
        params,
        return_type,
        closure_scope: _,
        type_params,
    } = func;

    let closure_scope = scopes.new_scope(Some(current_scope));

    if let Some(type_param) = &type_params {
        scopes.add_type_binding(
            &type_param,
            closure_scope,
            TypeBinding::Generic(type_param.clone()),
        )?;
    };

    scopes.add_binding(
        &ident,
        Value::Func(FuncValue {
            block,
            params,
            return_type,
            type_params,
            closure_scope,
        }),
        None,
        current_scope,
        false,
    )
}

// TODO: These aren't really doing checking anymore, they're just converting the literal types to regular types.
fn check_param_types(
    params: &Vec<ParamDeclare>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Vec<Param>, RuntimeError> {
    let results = params
        .iter()
        .map(|p| check_param_type(&p.ident, &p.param_type, scopes, current_scope))
        .collect();

    return accept_results(results);
}

fn check_param_type(
    ident: &str,
    param_type: &TypeLiteralVariant,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Param, RuntimeError> {
    let param_type = TypeVariant::from_literal(param_type.clone(), scopes, current_scope)?;

    Ok(Param {
        ident: ident.to_string(),
        param_type,
    })
}

pub fn eval_call(
    call: &Call,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match call {
        Call::Call(place, type_args, args) => {
            let block = eval_place_expr(place, scopes, current_scope, ctx)?;

            if let Value::Func(FuncValue {
                params,
                block,
                closure_scope,
                type_params,
                return_type: expected_return_type,
            }) = block
            {
                let call_scope = scopes.new_scope(Some(closure_scope));

                handle_type_args(type_args, type_params, scopes, call_scope)?;
                let args = handle_args(args, params, scopes, call_scope, current_scope, ctx)?;

                let return_value = match *block {
                    FuncVariant::Nala(stmts) => eval_stmts(&stmts, scopes, call_scope, ctx)?,
                    FuncVariant::Builtin(func) => func(args, ctx)?,
                };

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

fn handle_type_args(
    type_args: &Option<TypeLiteralVariant>,
    type_params: Option<String>,
    scopes: &mut Scopes,
    call_scope: usize,
) -> Result<(), RuntimeError> {
    if let Some(type_arg) = type_args {
        if type_params.is_none() {
            Err(RuntimeError::new(&format!(
                "Tried to call function with type arguments, but function has no type parameters."
            )))?;
        }

        let type_arg = TypeVariant::from_literal(type_arg.clone(), &mut Scopes::new(), 0)?;
        let type_binding = TypeBinding::from_type(type_arg);

        scopes.add_type_binding(&type_params.unwrap(), call_scope, type_binding)?;
    }

    Ok(())
}

fn handle_args(
    args: &Vec<Expr>,
    params: Vec<Param>,
    scopes: &mut Scopes,
    call_scope: usize,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<HashMap<String, Value>, RuntimeError> {
    let args = eval_elems(&*args, scopes, current_scope, ctx)?;

    if params.len() != args.len() {
        return Err(RuntimeError::new(&format!(
            "Called function with wrong number of arguments: Expected {0}, got {1}.",
            params.len(),
            args.len()
        )));
    }

    let mut param_args: HashMap<String, Value> = HashMap::new();

    for (i, param) in params.iter().enumerate() {
        let arg = args.get(i).unwrap();

        if !fits_type(arg, &param.param_type, scopes, current_scope)? {
            return Err(wrong_arg_type_for_param_error(
                arg,
                infer_type(&arg, scopes, current_scope)?.to_string(),
                param.param_type.to_string(),
            ));
        }

        scopes.add_binding(&param.ident, arg.clone(), None, call_scope, false)?;
        param_args.entry(param.ident.clone()).or_insert(arg.clone());
    }

    Ok(param_args)
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

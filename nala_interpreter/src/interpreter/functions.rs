use std::collections::HashMap;

use super::{basic::*, variables::*};

use crate::{
    ast::{
        funcs::{Call, FuncDeclare, ParamDeclare},
        types::type_literal_variant::TypeVariantLiteral,
        *,
    },
    errors::*,
    io_context::IoContext,
    resolved::{
        func_value::{FuncValue, Param},
        value::Value,
    },
    scopes::Scopes,
    types::{
        fit::fits_type, inference::infer_type, nala_type::NalaType, type_variant::TypeVariant,
    },
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
        type_param,
    } = func;

    let closure_scope = scopes.new_scope(Some(current_scope));

    if let Some(type_param) = &type_param {
        scopes.add_type_binding(
            closure_scope,
            &type_param,
            TypeVariant::generic(type_param.clone()),
        )?;
    };

    let params = params_from_declares(&params, scopes, closure_scope)?;
    let return_type = TypeVariant::from_literal(return_type, scopes, closure_scope)?;

    scopes.add_binding(
        &ident,
        Value::Func(FuncValue {
            block,
            params,
            return_type,
            type_param,
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
        type_param,
    } = func;

    let closure_scope = scopes.new_scope(Some(current_scope));

    if let Some(type_param) = &type_param {
        scopes.add_type_binding(
            closure_scope,
            &type_param,
            TypeVariant::generic(type_param.clone()),
        )?;
    };

    scopes.add_binding(
        &ident,
        Value::Func(FuncValue {
            block,
            params,
            return_type,
            type_param,
            closure_scope,
        }),
        None,
        current_scope,
        false,
    )
}

fn params_from_declares(
    params: &Vec<ParamDeclare>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Vec<Param>, RuntimeError> {
    accept_results(
        params
            .iter()
            .map(|p| param_from_declare(&p.ident, &p.param_type, scopes, current_scope))
            .collect(),
    )
}

fn param_from_declare(
    ident: &str,
    param_type: &TypeVariantLiteral,
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
                type_param,
                return_type: expected_return_type,
            }) = block
            {
                let call_scope = scopes.new_scope(Some(closure_scope));

                handle_type_args(type_args, type_param.clone(), scopes, call_scope)?;
                let args = handle_args(args, params, scopes, call_scope, current_scope, ctx)?;

                let return_value = match *block {
                    FuncVariant::Nala(lines) => eval_lines(&lines, scopes, call_scope, ctx)?,
                    FuncVariant::Builtin(func) => func(args, ctx)?,
                };

                let expected_return_type = if let Some(type_param) = type_param {
                    let concrete_type = scopes.get_type(&type_param, call_scope)?;
                    expected_return_type.make_concrete(Some(type_param), &concrete_type)
                } else {
                    expected_return_type
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
        Call::Literal(value) => Ok(Value::from_literal(value.clone())?),
    }
}

fn handle_type_args(
    type_args: &Option<TypeVariantLiteral>,
    type_param: Option<String>,
    scopes: &mut Scopes,
    call_scope: usize,
) -> Result<(), RuntimeError> {
    if let Some(type_arg) = type_args {
        if type_param.is_none() {
            Err(RuntimeError::new(&format!(
                "Tried to call function with type arguments, but function has no type parameters."
            )))?;
        }

        let type_arg = TypeVariant::from_literal(type_arg.clone(), &mut Scopes::new(), 0)?;
        scopes.add_type_binding(call_scope, &type_param.unwrap(), type_arg)?;
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

        let arg_type = infer_type(&arg, scopes, current_scope)?;
        resolve_generics(&param.param_type, arg_type, scopes, call_scope)?;

        scopes.add_binding(&param.ident, arg.clone(), None, call_scope, false)?;
        param_args.entry(param.ident.clone()).or_insert(arg.clone());
    }

    Ok(param_args)
}

fn resolve_generics(
    param_type_variant: &TypeVariant,
    arg_type: TypeVariant,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<(), RuntimeError> {
    match param_type_variant {
        TypeVariant::Composite(c) => {
            if let Some(_) = c.generic_type_param {
                let arg_inners = arg_type.as_composite().unwrap().inner;

                for (i, param_inner) in c.inner.iter().enumerate() {
                    let arg_inner = arg_inners.get(i).unwrap();
                    resolve_generics(param_inner, arg_inner.clone(), scopes, current_scope)?;
                }
            }
        }
        TypeVariant::Type(t) => {
            if let NalaType::Generic(ident) = t {
                // TODO: This will overwrite type bindings made in previous passes, either from
                // earlier args to the given Nala call, or from earlier type params to the same arg
                // (in the case of composite types with multiple inner type args). This also won't
                // throw any kind of error if the more recently bound type doesn't match up with the
                // one bound earlier.

                scopes.update_type_binding(current_scope, ident, arg_type)?;
            }
        }
    };

    Ok(())
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

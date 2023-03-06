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
    types::{type_variant::TypeVariant, NalaType},
};

pub fn eval_func(
    func: &Func,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let Func {
        ident,
        block,
        params,
        return_type,
    } = func;

    check_param_types(&params, scopes, current_scope)?;

    scopes.add_binding(
        &ident,
        Value::Func(StoredFunc {
            params: params.clone(), // TODO: Do we need this clone? Do we need to be borrowing in the params?
            return_type: return_type.clone(),
            block: block.clone(),
            closure_scope: current_scope,
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

                if binding.is_generic() {
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

pub fn eval_invocation(
    call: &Invocation,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match call {
        Invocation::Invocation(place, args) => {
            let block = eval_place_expr(place, scopes, current_scope, enclosing_scope, ctx)?;

            if let Value::Func(StoredFunc {
                params,
                block,
                closure_scope,
                return_type,
            }) = block
            {
                let call_scope = scopes.new_scope(Some(current_scope));
                let args = eval_elems(&*args, scopes, call_scope, enclosing_scope, ctx)?;

                if params.len() != args.len() {
                    panic!(
                        "Called function with wrong number of arguments: Expected {0}, got {1}.",
                        params.len(),
                        args.len()
                    )
                }

                let mut param_args: HashMap<String, Value> = HashMap::new();

                for (i, param) in params.iter().enumerate() {
                    let arg = args.get(i).unwrap();

                    let arg_type = arg.get_type(scopes, current_scope)?;

                    let param_type =
                        TypeVariant::from_literal(param.param_type.clone(), scopes, current_scope)?;

                    /*
                     * TODO: This is where our issue is. This results in `Option<T><Number>`
                     * because we're storing generic args in two different ways;
                     * both using TypeVariant::Generic and using type_args in enum.
                     */
                    // println!("Param type: {:?}", param_type);

                    /*
                     * TODO: We're temporarily only doing type checking on NalaBlocks, so that builtins
                     * like `print()` can be called with args of any type. We should move away from this
                     * in favor of generics.
                     */
                    if let Block::NalaBlock(_) = *block {
                        if !arg_type.is_assignable_to(&param_type) {
                            return Err(wrong_arg_type_for_param_error(
                                arg.clone().to_string(),
                                arg.get_type(scopes, current_scope)?.to_string(),
                                param_type.to_string(),
                            ));
                        }
                    }

                    scopes.add_binding(&param.ident, arg.clone(), None, call_scope, false)?;
                    param_args.entry(param.ident.clone()).or_insert(arg.clone());
                }

                let return_value = match *block {
                    Block::NalaBlock(stmts) => {
                        eval_stmts(&stmts, scopes, call_scope, Some(closure_scope), ctx)?
                    }
                    Block::RustBlock(func) => Ok(func(param_args, ctx)?)?,
                };

                let return_type = TypeVariant::from_literal(return_type, scopes, current_scope)?;

                // TODO: This is a temporary bypass to support builtins until we have generics.
                if let TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Any)) = return_type
                {
                    return Ok(return_value);
                }

                if return_value
                    .get_type(scopes, current_scope)?
                    .is_assignable_to(&return_type)
                {
                    Ok(return_value)
                } else {
                    Err(RuntimeError::new(&format!("Tried to return value `{return_value}` of type `{0}` where value of type `{return_type}` was expected.", return_value.get_type(scopes, current_scope)?)))
                }
            } else {
                Err(RuntimeError::new(&format!("Cannot invoke a non-function.")))
            }
        }
        Invocation::PlaceExpression(place) => {
            eval_place_expr(place, scopes, current_scope, enclosing_scope, ctx)
        }
        Invocation::Value(value) => Ok(value.clone()),
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
    arg_value: String,
    arg_type: String,
    param_type: String,
) -> RuntimeError {
    RuntimeError::new(&format!(
        "Passed value `{arg_value}` of type `{arg_type}` to function where `{param_type}` was expected.")
    )
}

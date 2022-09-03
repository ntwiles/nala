use std::collections::HashMap;

use super::{basic::*, variables::*};

use crate::{
    ast::{
        funcs::*,
        terms::*,
        types::{nala_type::NalaType, primitive_type::PrimitiveType, type_variant::TypeVariant},
        *,
    },
    errors::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

fn wrong_arg_type_for_param_error(
    arg_value: String,
    arg_type: String,
    param_type: String,
) -> NalaRuntimeError {
    NalaRuntimeError {
        message: format!(
            "Passed value `{0}` of type `{1}` to function where `{2}` was expected.",
            arg_value, arg_type, param_type
        ),
    }
}

pub fn eval_func(
    func: &Func,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Result<Value, NalaRuntimeError> {
    let Func {
        ident,
        block,
        params,
    } = func;

    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        let block = Box::new(block.clone());
        let params = params.clone();

        let result = check_param_types(params.clone());

        if result.is_ok() {
            scopes.add_binding(&ident, current_scope, Value::Func(params, *block), false);
        } else {
            return Err(NalaRuntimeError {
                message: result.unwrap_err(),
            });
        }
    }

    Ok(Value::Void)
}

fn check_param_types(params: Vec<Param>) -> Result<(), String> {
    let mut results = params.iter().map(|p| check_param_type(&p.param_type));

    if let Some(Err(err)) = results.find(|r| r.is_err()) {
        Err(err)
    } else {
        Ok(())
    }
}

fn check_param_type(param_type: &TypeVariant) -> Result<(), String> {
    if let TypeVariant::Nested(outer, inner) = param_type {
        let outer = if let NalaType::PrimitiveType(outer) = outer {
            outer
        } else {
            let inners = inner
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            return Err(format!(
                "Type `{0}` does not support nesting. Type `{0}<{1}>` is invalid.",
                outer, inners
            ));
        };

        return match outer {
            PrimitiveType::Array => Ok(()),
            PrimitiveType::Func => Ok(()),
            _ => {
                let inners = inner
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                Err(format!(
                    "Type `{0}` does not support nesting. Type `{0}<{1}>` is invalid.",
                    outer, inners
                ))
            }
        };
    }

    Ok(())
}

pub fn eval_invocation(
    call: &Invocation,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match call {
        Invocation::Invocation(place, args) => {
            let block = eval_place_expr(place, scopes, current_scope, ctx)?;

            if let Value::Func(params, block) = block {
                let func_scope = scopes.new_scope(Some(current_scope));

                let args = eval_elems(&*args, scopes, func_scope, ctx)?;

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

                    let arg_type = arg.get_type();
                    let param_type = param.param_type.clone();

                    if !arg_type.is_assignable_to(&param_type) {
                        return Err(wrong_arg_type_for_param_error(
                            arg.clone().to_string(),
                            arg.get_type().to_string(),
                            param_type.to_string(),
                        ));
                    }

                    scopes.add_binding(&param.ident, func_scope, arg.clone(), false);
                    param_args.entry(param.ident.clone()).or_insert(arg.clone());
                }

                let block = *block;

                match block {
                    Block::NalaBlock(stmts) => eval_stmts(&stmts, scopes, func_scope, ctx),
                    Block::RustBlock(func) => Ok(func(param_args, ctx)),
                }
            } else {
                panic!("Cannot invoke a non-function.")
            }
        }
        Invocation::PlaceExpression(place) => eval_place_expr(place, scopes, current_scope, ctx),
        Invocation::Value(value) => Ok(value.clone()),
    }
}

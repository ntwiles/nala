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
    scope::Scopes,
    types::type_variant::TypeVariant,
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
    current_scope: usize,
) -> Result<Value, NalaRuntimeError> {
    let Func {
        ident,
        block,
        params,
        return_type,
    } = func;

    let result = check_param_types(&params);

    if result.is_ok() {
        scopes.add_binding(
            &ident,
            current_scope,
            Value::Func(StoredFunc {
                params: params.clone(), // TODO: Do we need this clone? Do we need to be borrowing in the params?
                return_type: return_type.clone(),
                block: block.clone(),
                closure_scope: current_scope,
            }),
            false,
        )
    } else {
        Err(NalaRuntimeError {
            message: result.unwrap_err(),
        })
    }
}

fn check_param_types(params: &Vec<Param>) -> Result<(), String> {
    let mut results = params.iter().map(|p| check_param_type(&p.param_type));

    if let Some(Err(err)) = results.find(|r| r.is_err()) {
        Err(err)
    } else {
        Ok(())
    }
}

fn check_param_type(param_type: &TypeLiteralVariant) -> Result<(), String> {
    if let TypeLiteralVariant::Nested(outer, inner) = param_type {
        let outer = if let TypeLiteral::PrimitiveType(outer) = outer {
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
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match call {
        Invocation::Invocation(place, args) => {
            let block = eval_place_expr(place, scopes, current_scope, enclosing_scope, ctx)?;

            if let Value::Func(StoredFunc {
                params,
                block,
                closure_scope,
                ..
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

                    let arg_type = arg.get_type(scopes, current_scope);

                    let param_type =
                        TypeVariant::from_literal(param.param_type.clone(), scopes, current_scope);

                    /*
                     * TODO: We're temporarily only doing type checking on NalaBlocks, so that builtins
                     * like `print()` can be called with args of any type. We should move away from this
                     * in favor of generics.
                     */
                    if let Block::NalaBlock(_) = *block {
                        if !arg_type.is_assignable_to(&param_type) {
                            return Err(wrong_arg_type_for_param_error(
                                arg.clone().to_string(),
                                arg.get_type(scopes, current_scope).to_string(),
                                param_type.to_string(),
                            ));
                        }
                    }

                    scopes.add_binding(&param.ident, call_scope, arg.clone(), false)?;
                    param_args.entry(param.ident.clone()).or_insert(arg.clone());
                }

                match *block {
                    Block::NalaBlock(stmts) => {
                        eval_stmts(&stmts, scopes, call_scope, Some(closure_scope), ctx)
                    }
                    Block::RustBlock(func) => Ok(func(param_args, ctx)),
                }
            } else {
                panic!("Cannot invoke a non-function.")
            }
        }
        Invocation::PlaceExpression(place) => {
            eval_place_expr(place, scopes, current_scope, enclosing_scope, ctx)
        }
        Invocation::Value(value) => Ok(value.clone()),
    }
}

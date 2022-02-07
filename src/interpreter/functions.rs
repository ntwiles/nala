use std::collections::HashMap;

use super::{arrays::*, basic::*};

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    errors::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub struct WrongArgTypeForParamError {
    func_ident: String,
    arg_value: String,
    arg_type: String,
    param_type: String,
}

impl NalaRuntimeError for WrongArgTypeForParamError {
    fn message(&self) -> String {
        format!(
            "Passed value `{0}` of type `{1}` to func `{2}` where `{3}` was expected.",
            self.arg_value, self.arg_type, self.func_ident, self.param_type
        )
    }
}

pub fn interpret_func(func: &Func, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    let Func {
        ident,
        block,
        params,
    } = func;

    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        let block = Box::new(block.clone());
        let params = Box::new(params.clone());

        let result = check_param_types(&*params);

        if result.is_ok() {
            scopes.add_binding(&ident, current_scope, Term::Func(*params, *block), false);
        } else {
            panic!("{}", result.unwrap_err())
        }
    }

    Term::Void
}

fn check_param_types(params: &Params) -> Result<(), String> {
    match params {
        Params::Params(params, Param { param_type, .. }) => {
            match check_param_types(params) {
                Ok(_) => (),
                Err(err) => return Err(err),
            };

            check_param_type(param_type)
        }
        Params::Param(Param { param_type, .. }) => check_param_type(param_type),
        Params::Empty => Ok(()),
    }
}

fn check_param_type(param_type: &TypeVariant) -> Result<(), String> {
    if let TypeVariant::Nested(outer, inner) = param_type {
        return match outer {
            PrimitiveType::Array => Ok(()),
            PrimitiveType::Func => Ok(()),
            _ => {
                let message = format!(
                    "Type `{0}` does not support nesting. Type `{0}<{1}>` is invalid.",
                    outer.to_string(),
                    inner.to_string()
                );
                Err(message)
            }
        };
    }

    Ok(())
}

pub fn evaluate_call(
    call: &Call,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match call {
        Call::Call(ident, args) => {
            let block = scopes.get_value(ident, current_scope, context);

            if let Term::Func(params, block) = block {
                let func_scope = scopes.new_scope(Some(current_scope));

                let params_vec = evaluate_params(&*params, scopes, func_scope, context);
                let args = evaluate_elems(&*args, scopes, func_scope, context);

                if params_vec.len() != args.len() {
                    panic!(
                        "Called function `{0}` with wrong number of arguments: Expected {1}, got {2}.",
                        ident,
                        params_vec.len(),
                        args.len()
                    )
                }

                let param_args: HashMap<String, Term> = params_vec
                    .clone()
                    .iter()
                    .enumerate()
                    .map(|(i, Param { ident, param_type })| {
                        let arg = args.get(i).unwrap();

                        let arg_type = arg.get_type();
                        let param_type = param_type.clone();

                        if !arg_type.is_assignable_to(&param_type) {
                            runtime_error(
                                context,
                                WrongArgTypeForParamError {
                                    arg_value: arg.clone().to_string(),
                                    arg_type: arg.get_type().to_string(),
                                    func_ident: ident.to_owned(),
                                    param_type: param_type.to_string(),
                                },
                            )
                        }

                        // TODO: Should function args be mutable or immutable?
                        scopes.add_binding(ident, func_scope, arg.clone(), false);

                        (ident.clone(), arg.clone())
                    })
                    .collect();

                let block = *block;

                match block {
                    Block::NalaBlock(stmts) => interpret_stmts(&stmts, scopes, func_scope, context),
                    Block::RustBlock(func) => func(param_args, context),
                }
            } else {
                panic!("Cannot invoke `{0}` because it is not a function.", ident)
            }
        }
        Call::Index(index) => evaluate_index(index, scopes, current_scope, context),
    }
}

pub fn evaluate_params(
    params: &Params,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Vec<Param> {
    match params {
        Params::Params(params, param) => {
            let mut params = evaluate_params(params, scopes, current_scope, context);
            params.push(param.to_owned());
            params
        }
        Params::Param(param) => vec![param.to_owned()],
        Params::Empty => vec![],
    }
}

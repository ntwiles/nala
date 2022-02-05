use std::collections::HashMap;

use super::{arrays::*, basic::*};

use crate::{
    ast::{funcs::*, terms::*, types::*, *},
    builtins::BuiltinFunc,
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

pub fn interpret_func(
    ident: &String,
    params: &Params,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Term {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        let block = Box::new(block.clone());
        let params = Box::new(params.clone());

        let result = check_param_types(&*params);

        if result.is_ok() {
            scopes.add_binding(&ident, current_scope, Term::Func(params, block), false);
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
            let block = scopes.get_value(ident, current_scope);
            if let Term::Func(params, block) = block {
                let func_scope = scopes.new_scope(Some(current_scope));

                let params = evaluate_params(&*params, scopes, func_scope, context);
                let args = evaluate_elems(&*args, scopes, func_scope, context);

                if params.len() != args.len() {
                    panic!(
                        "Called func `{0}` with wrong number of arguments: Expected {1}, got {2}.",
                        ident,
                        params.len(),
                        args.len()
                    )
                }

                for i in 0..params.len() {
                    let param = params.get(i).unwrap();
                    let arg = args.get(i).unwrap();

                    let arg_type = arg.get_type();
                    let param_type = param.param_type.clone();

                    if !arg_type.is_assignable_to(&param_type) {
                        runtime_error(WrongArgTypeForParamError {
                            arg_value: arg.clone().to_string(),
                            arg_type: arg.get_type().to_string(),
                            func_ident: ident.to_owned(),
                            param_type: param_type.to_string(),
                        })
                    }

                    scopes.add_binding(&param.ident, func_scope, arg.clone(), true)
                }

                interpret_block(&block, scopes, func_scope, context)
            } else {
                // This Void should never be returned, consider writing this differently and panicking?
                Term::Void
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

pub fn invoke_builtin(
    func: BuiltinFunc,
    params: &Params,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    // TODO: We already get params and args in evaluate_call, do we have to do this work again?
    let params: Vec<Param> = evaluate_params(params, scopes, current_scope, context);
    let args: HashMap<String, Term> = params
        .into_iter()
        .map(|Param { ident, .. }| (ident.clone(), scopes.get_value(&ident, current_scope)))
        .collect();

    func(args, scopes, current_scope, context)
}

use super::{arrays::*, basic::*};

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

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

fn check_param_type(param_type: &Type) -> Result<(), String> {
    if let Type::Nested(outer, inner) = param_type {
        return if let PrimitiveType::Array = outer {
            Ok(())
        } else {
            let message = format!(
                "Type `{0}` does not support nesting. Type `{0}<{1}>` is invalid.",
                outer.to_string(),
                inner.to_string()
            );
            Err(message)
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
                    panic!("Number of params does not match number of arguments.")
                }

                for i in 0..params.len() {
                    let Param {
                        ident: param_ident,
                        param_type,
                    } = params.get(i).unwrap();
                    let arg = args.get(i).unwrap();

                    let arg_type = arg.get_type();
                    let param_type = param_type.clone();

                    if !arg_type.is_assignable_to(&param_type) {
                        panic!(
                            "Passed value `{0}` of type `{1}` to func `{2}` where `{3}` was expected.",
                            arg.clone().to_string(),
                            arg.get_type().to_string(),
                            ident,
                            param_type.to_string(),
                        )
                    }

                    scopes.add_binding(param_ident, func_scope, arg.clone(), true)
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

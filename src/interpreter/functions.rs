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

        scopes.add_binding(&ident, current_scope, Term::Func(params, block), false);
    }

    Term::Void
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
                    let (param, param_type) = params.get(i).unwrap();
                    let arg = args.get(i).unwrap();

                    let arg_type = arg.value_type();
                    let param_type = param_type.clone();

                    if arg_type != param_type && param_type != ValueType::Any {
                        panic!(
                            "Passed value `{3}` of type {0} to func `{1}` where {2} was expected.",
                            arg.value_type().to_string(),
                            ident,
                            param_type.to_string(),
                            arg.clone().to_string()
                        )
                    }

                    scopes.add_binding(param, func_scope, arg.clone(), true)
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
) -> Vec<(String, ValueType)> {
    match params {
        Params::Params(params, param) => {
            let mut params = evaluate_params(params, scopes, current_scope, context);
            params.push(param.to_owned());
            params
        }
        Params::Param(param, type_name) => vec![(param.to_owned(), type_name.to_owned())],
        Params::Empty => vec![],
    }
}

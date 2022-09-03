use std::sync::Arc;

use super::{arrays::eval_index, eval_expr, objects::*};

use crate::{
    ast::{objects::*, terms::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn eval_declare(
    ident: &String,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    is_mutable: bool,
) -> Result<Value, NalaRuntimeError> {
    if scopes.binding_exists_local(&ident, current_scope) {
        return Err(NalaRuntimeError {
            message: format!("Binding for {} already exists in local scope.", ident),
        });
    } else {
        if let Value::Void = value {
            return Err(NalaRuntimeError {
                message: "Cannot declare a variable with a value of type Void.".to_string(),
            });
        }

        scopes.add_binding(&ident, current_scope, value.clone(), is_mutable);
    }

    Ok(Value::Void)
}

pub fn eval_assign(
    variable: &PlaceExpression,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match variable {
        PlaceExpression::Index(place, index_expr) => {
            match &**place {
                PlaceExpression::Index(_, _) => todo!(),
                PlaceExpression::MemberAccess(member_access) => {
                    let array =
                        eval_member_access(None, member_access, scopes, current_scope, context)?;

                    let index = if let Value::Num(index) =
                        eval_expr(index_expr, scopes, current_scope, context)?
                    {
                        index
                    } else {
                        todo!();
                    };

                    if let Value::Array(array) = array {
                        let array = Arc::clone(&array);
                        let mut array = array.lock().unwrap();
                        array[index as usize] = value.clone();
                    } else {
                        panic!("Trying to index into a non-Array.")
                    }
                }
                PlaceExpression::Symbol(ident) => {
                    if scopes.binding_exists(&ident, current_scope, context) {
                        let index_result = eval_expr(&index_expr, scopes, current_scope, context)?;

                        if let Value::Void = value {
                            panic!("Cannot assign a value of type Void.");
                        }

                        let index = if let Value::Num(index) = index_result {
                            index
                        } else {
                            panic!("Index does not resolve to a Number.");
                        };

                        let array = scopes.get_value(&ident, current_scope, context)?;

                        if let Value::Array(array) = array {
                            let array = Arc::clone(&array);
                            let mut array = array.lock().unwrap();
                            array[index as usize] = value.clone();
                            //return scopes.mutate_value(&ident, current_scope, Value::Array(array));
                        } else {
                            panic!("Trying to index into a non-Array.")
                        }
                    }
                }
            }
        }
        PlaceExpression::Symbol(ident) => {
            if scopes.binding_exists(&ident, current_scope, context) {
                if let Value::Void = value {
                    panic!("Cannot assign a value of type Void.");
                }

                let existing = scopes.get_value(&ident, current_scope, context)?;

                let existing_type = existing.get_type();
                let value_type = value.get_type();

                if existing_type == value_type {
                    return scopes.mutate_value(&ident, current_scope, value.clone());
                } else {
                    return Err(NalaRuntimeError {
                        message: format!(
                            "Cannot assign a value of type {0} where {1} is expected.",
                            value_type, existing_type
                        ),
                    });
                }
            } else {
                panic!("Unknown identifier `{}`", ident);
            }
        }
        PlaceExpression::MemberAccess(member_access) => {
            let (parent, child) = match &**member_access {
                MemberAccess::MemberAccesses(parents, child) => (
                    eval_member_access(None, &*parents, scopes, current_scope, context)?,
                    child,
                ),
                MemberAccess::MemberAccess(parent, child) => {
                    (scopes.get_value(&parent, current_scope, context)?, child)
                }
            };

            if let Value::Object(parent) = parent {
                let parent = Arc::clone(&parent);
                let mut parent = parent.lock().unwrap();
                parent.insert(child.to_string(), value.clone());
            } else {
                todo!()
            };
        }
    }

    Ok(Value::Void)
}

pub fn eval_place_expr(
    variable: &PlaceExpression,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match variable {
        PlaceExpression::Index(place, expr) => {
            let array = eval_place_expr(place, scopes, current_scope, context)?;
            eval_index(&array, expr, scopes, current_scope, context)
        }
        PlaceExpression::Symbol(ident) => {
            if scopes.binding_exists(&ident, current_scope, context) {
                scopes.get_value(ident, current_scope, context)
            } else {
                Err(NalaRuntimeError {
                    message: format!("Unknown identifier `{}`", ident),
                })
            }
        }
        PlaceExpression::MemberAccess(member_access) => {
            eval_member_access(None, member_access, scopes, current_scope, context)
        }
    }
}

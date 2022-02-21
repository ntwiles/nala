use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    ast::{objects::*, terms::Value},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::*,
};

use super::{arrays::evaluate_index, basic::evaluate_expr};

pub fn evaluate_member_access(
    member_access: &MemberAccess,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match member_access {
        MemberAccess::MemberAccesses(parents, child) => {
            let object = evaluate_member_access(parents, scopes, current_scope, context)?;

            if let Value::Object(reference) = object {
                let object = Arc::clone(&reference);
                let object = object.lock().unwrap();

                if object.contains_key(child) {
                    Ok(object[child].clone())
                } else {
                    Err(NalaRuntimeError {
                        message: format!("Member `{0}` does not exist on object.", child),
                    })
                }
            } else {
                Err(NalaRuntimeError {
                    message: format!(
                        "Cannot access member `{0}` of non-Object `{1}`.",
                        child, object
                    ),
                })
            }
        }
        MemberAccess::MemberAccess(parent, child) => {
            let object = scopes.get_value(parent, current_scope, context)?;

            if let Value::Object(reference) = object {
                let object = Arc::clone(&reference);
                let object = object.lock().unwrap();

                if object.contains_key(child) {
                    Ok(object[child].clone())
                } else {
                    Err(NalaRuntimeError {
                        message: format!(
                            "Member `{0}` does not exist on object `{1}`",
                            child, parent
                        ),
                    })
                }
            } else {
                todo!()
            }
        }
        MemberAccess::Index(index) => evaluate_index(index, scopes, current_scope, context),
    }
}

pub fn evaluate_object(
    object: &Object,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let object: HashMap<String, Value> =
        evaluate_object_entries(&object.entries, scopes, current_scope, context)?;

    Ok(Value::Object(Arc::new(Mutex::new(object))))
}

fn evaluate_object_entries(
    entries: &Vec<KeyValuePair>,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<HashMap<String, Value>, NalaRuntimeError> {
    let results: Vec<Result<(String, Value), NalaRuntimeError>> = entries
        .iter()
        .map(|kvp| evaluate_object_entry(kvp, scopes, current_scope, context))
        .collect();

    if let Some(Err(error)) = results.iter().find(|r| r.is_err()) {
        Err(error.clone())
    } else {
        let mut object = HashMap::<String, Value>::new();

        for result in results {
            let (key, value) = result.clone().unwrap();
            object.insert(key, value);
        }

        Ok(object)
    }
}

fn evaluate_object_entry(
    entry: &KeyValuePair,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<(String, Value), NalaRuntimeError> {
    let value = evaluate_expr(&*entry.value, scopes, current_scope, context)?;
    Ok((entry.key.clone(), value))
}

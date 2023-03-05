use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    ast::{objects::*, terms::Value},
    errors::RuntimeError,
    io_context::IoContext,
    scopes::*,
};

use super::basic::eval_expr;

pub fn eval_member_access(
    parent_obj: Option<Arc<Mutex<HashMap<String, Value>>>>,
    member_access: &MemberAccess,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match member_access {
        MemberAccess::MemberAccesses(parents, child) => {
            let object = eval_member_access(parent_obj, parents, scopes, current_scope, ctx)?;

            if let Value::Object(reference) = object {
                let object = Arc::clone(&reference);
                let object = object.lock().unwrap();
                if object.contains_key(child) {
                    Ok(object[child].clone())
                } else {
                    Err(RuntimeError::new(&format!(
                        "Member `{child}` does not exist on object."
                    )))
                }
            } else {
                Err(RuntimeError::new(&format!(
                    "Cannot access member `{child}` of non-Object `{object}`."
                )))
            }
        }
        MemberAccess::MemberAccess(parent, child) => {
            let object = match parent_obj {
                Some(_parent_obj) => todo!(),
                None => scopes.get_value(parent, current_scope, None)?, // TODO: Should we be ignoring enclosing scope here?
            };

            if let Value::Object(reference) = object {
                let object = Arc::clone(&reference);
                let object = object.lock().unwrap();
                if object.contains_key(child) {
                    Ok(object[child].clone())
                } else {
                    Err(RuntimeError::new(&format!(
                        "Member `{child}` does not exist on object `{parent}`"
                    )))
                }
            } else {
                todo!()
            }
        }
    }
}

pub fn eval_object(
    object: &Object,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let object: HashMap<String, Value> =
        eval_object_entries(&object.entries, scopes, current_scope, enclosing_scope, ctx)?;

    Ok(Value::Object(Arc::new(Mutex::new(object))))
}

fn eval_object_entries(
    entries: &Vec<KeyValuePair>,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<HashMap<String, Value>, RuntimeError> {
    let results: Vec<Result<(String, Value), RuntimeError>> = entries
        .iter()
        .map(|kvp| eval_object_entry(kvp, scopes, current_scope, enclosing_scope, ctx))
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

fn eval_object_entry(
    entry: &KeyValuePair,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<(String, Value), RuntimeError> {
    let value = eval_expr(&*entry.value, scopes, current_scope, enclosing_scope, ctx)?;
    Ok((entry.key.clone(), value))
}

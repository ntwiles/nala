use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    ast::objects::*, errors::RuntimeError, io_context::IoContext, resolved::value::Value, scopes::*,
};

use super::basic::eval_expr;

pub fn eval_member_access(object: &Value, field: &String) -> Result<Value, RuntimeError> {
    if let Value::Object(reference) = object {
        let object = Arc::clone(&reference);
        let object = object.lock().unwrap();
        if object.contains_key(field) {
            Ok(object[field].clone())
        } else {
            Err(RuntimeError::new(&format!(
                "Member `{field}` does not exist on parent object." // TODO: Get the identifier for the object.
            )))
        }
    } else {
        Err(RuntimeError::new(&format!(
            "Tried to access member `{field}` of non-Object `{object}`."
        )))
    }
}

pub fn eval_object(
    object: &Object,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let object: HashMap<String, Value> =
        eval_object_entries(&object.entries, scopes, current_scope, ctx)?;

    Ok(Value::Object(Arc::new(Mutex::new(object))))
}

fn eval_object_entries(
    entries: &Vec<KeyValuePair>,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<HashMap<String, Value>, RuntimeError> {
    let results: Vec<Result<(String, Value), RuntimeError>> = entries
        .iter()
        .map(|kvp| eval_object_entry(kvp, scopes, current_scope, ctx))
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
    ctx: &mut dyn IoContext,
) -> Result<(String, Value), RuntimeError> {
    let value = eval_expr(&*entry.value, scopes, current_scope, ctx)?;
    Ok((entry.key.clone(), value))
}

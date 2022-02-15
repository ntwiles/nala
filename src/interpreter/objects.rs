use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    ast::{objects::*, terms::Term},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::*,
};

use super::{arrays::evaluate_index, basic::evaluate_expr};

pub fn evaluate_member_access(
    member_access: &MemberAccess,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    match member_access {
        MemberAccess::MemberAccesses(parents, child) => {
            let object = evaluate_member_access(parents, scopes, current_scope, context)?;

            if let Term::Object(reference) = object {
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
                todo!()
            }
        }
        MemberAccess::MemberAccess(parent, child) => {
            let object = scopes.get_value(parent, current_scope, context)?;

            if let Term::Object(reference) = object {
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
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let object: HashMap<String, Term> =
        evaluate_object_entries(&mut *object.entries.clone(), scopes, current_scope, context)?;
    Ok(Term::Object(Arc::new(Mutex::new(object))))
}

fn evaluate_object_entries(
    entries: &mut KeyValuePairs,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<HashMap<String, Term>, NalaRuntimeError> {
    match entries {
        KeyValuePairs::KeyValuePairs(entries, entry) => {
            let mut entries = evaluate_object_entries(entries, scopes, current_scope, context)?;
            let entry = evaluate_object_entry(entry, scopes, current_scope, context)?;
            entries.extend(entry);
            Ok(entries)
        }
        KeyValuePairs::KeyValuePair(entry) => Ok(evaluate_object_entry(
            entry,
            scopes,
            current_scope,
            context,
        )?),
    }
}

fn evaluate_object_entry(
    entry: &KeyValuePair,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<HashMap<String, Term>, NalaRuntimeError> {
    let value = evaluate_expr(&*entry.value, scopes, current_scope, context)?;
    let mut map = HashMap::<String, Term>::new();
    map.insert(entry.key.clone(), value);
    Ok(map)
}

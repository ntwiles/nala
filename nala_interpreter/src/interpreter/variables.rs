use std::sync::Arc;

use super::{arrays::eval_index, eval_expr, objects::*};

use crate::{
    ast::{objects::*, terms::*, types::type_literal_variant::TypeLiteralVariant, *},
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
    types::{fit::fits_type, inference::infer_type, type_variant::TypeVariant},
};

pub fn eval_declare(
    ident: &String,
    expr: &Expr,
    declared_type: Option<TypeLiteralVariant>,
    is_mutable: bool,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let value = eval_expr(expr, scopes, current_scope, ctx)?;

    if let Value::Void = value {
        return Err(RuntimeError::new(
            "Cannot declare a variable with a value of type Void.",
        ));
    }

    if let Some(declared_type) = declared_type {
        let declared_type =
            TypeVariant::from_literal(declared_type.clone(), scopes, current_scope)?;

        if !fits_type(&value, &declared_type, scopes, current_scope)? {
            return Err(RuntimeError::new(&format!(
                "Tried to declare variable `{ident}` with explicit type `{declared_type}` but value does not fit that type.",
            )));
        }

        scopes.add_binding(
            &ident,
            value.clone(),
            Some(declared_type),
            current_scope,
            is_mutable,
        )
    } else {
        // Inference just done to see if there's enough information to infer type before binding.
        // PERFORMANCE: A possible optimization could be to cache this on the binding once we
        // know the type of the value.
        infer_type(&value, scopes, current_scope)?;
        scopes.add_binding(&ident, value.clone(), None, current_scope, is_mutable)
    }
}

pub fn eval_assign(
    variable: &PlaceExpression,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match variable {
        PlaceExpression::Index(place, index_expr) => match &**place {
            PlaceExpression::Index(_, _) => todo!(),
            PlaceExpression::MemberAccess(member_access) => {
                // TODO: What is going on here? This is labeled as member access
                // but it looks like an array index.
                let array = eval_member_access(None, member_access, scopes, current_scope, ctx)?;

                let index =
                    if let Value::Num(index) = eval_expr(index_expr, scopes, current_scope, ctx)? {
                        index
                    } else {
                        todo!();
                    };

                if let Value::Array(array) = array {
                    let array = Arc::clone(&array);
                    let mut array = array.lock().unwrap();
                    array[index as usize] = value.clone();
                } else {
                    // TODO: Add test case for this error.
                    Err(RuntimeError::new("Trying to index into a non-Array."))?
                }
            }
            PlaceExpression::Symbol(ident) => {
                if scopes.binding_exists(&ident, current_scope) {
                    let index_result = eval_expr(&index_expr, scopes, current_scope, ctx)?;

                    // TODO: Add test cases for the three errors in this block.
                    if let Value::Void = value {
                        Err(RuntimeError::new("Cannot assign a value of type Void."))?;
                    }

                    let index = if let Value::Num(index) = index_result {
                        index
                    } else {
                        Err(RuntimeError::new("Index does not resolve to a Number."))?
                    };

                    let array = scopes.get_value(&ident, current_scope)?;

                    if let Value::Array(array) = array {
                        let array = Arc::clone(&array);
                        let mut array = array.lock().unwrap();
                        array[index as usize] = value.clone();
                    } else {
                        Err(RuntimeError::new("Trying to index into a non-Array."))?
                    }
                }
            }
        },
        PlaceExpression::Symbol(ident) => {
            // TODO: Add test cases for the two errors in this block.
            if let Value::Void = value {
                Err(RuntimeError::new("Cannot assign a value of type Void."))?;
            }

            let existing = scopes.get_value(&ident, current_scope)?;

            let existing_type = infer_type(&existing, scopes, current_scope)?;
            let value_type = infer_type(&value, scopes, current_scope)?;

            if existing_type == value_type {
                scopes.mutate_value(&ident, current_scope, value.clone())?;
            } else {
                Err(RuntimeError::new(&format!(
                    "Cannot assign a value of type {value_type} where {existing_type} is expected.",
                )))?
            }
        }
        PlaceExpression::MemberAccess(member_access) => {
            let (parent, child) = match &**member_access {
                MemberAccess::MemberAccesses(parents, child) => (
                    eval_member_access(None, &*parents, scopes, current_scope, ctx)?,
                    child,
                ),
                MemberAccess::MemberAccess(parent, child) => {
                    (scopes.get_value(&parent, current_scope)?, child)
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
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match variable {
        PlaceExpression::Index(place, expr) => {
            let array = eval_place_expr(place, scopes, current_scope, ctx)?;
            eval_index(&array, expr, scopes, current_scope, ctx)
        }
        PlaceExpression::Symbol(ident) => scopes.get_value(ident, current_scope),
        PlaceExpression::MemberAccess(member_access) => {
            eval_member_access(None, member_access, scopes, current_scope, ctx)
        }
    }
}

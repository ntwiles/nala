use std::sync::Arc;

use super::{arrays::eval_index, eval_expr, objects::*};

use crate::{
    ast::{objects::*, terms::*, types::type_literal_variant::TypeLiteralVariant, *},
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
    types::type_variant::TypeVariant,
};

pub fn eval_declare(
    ident: &String,
    expr: &Expr,
    declared_type: Option<TypeLiteralVariant>,
    is_mutable: bool,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let value = eval_expr(expr, scopes, current_scope, enclosing_scope, ctx)?;

    if let Value::Void = value {
        return Err(RuntimeError::new(
            "Cannot declare a variable with a value of type Void.",
        ));
    }

    if let Some(declared_type) = declared_type {
        let declared_type =
            TypeVariant::from_literal(declared_type.clone(), scopes, current_scope)?;

        let actual_type = value.get_type(scopes, current_scope)?;

        if declared_type != actual_type {
            return Err(RuntimeError::new(&format!(
                "Tried to declare variable `{ident}` with explicit type `{declared_type}` but value of type `{actual_type}` was given.",
            )));
        }

        scopes.add_binding(
            &ident,
            value.clone(),
            Some(declared_type),
            current_scope,
            is_mutable,
        );
    }

    scopes.add_binding(&ident, value.clone(), None, current_scope, is_mutable)
}

pub fn eval_assign(
    variable: &PlaceExpression,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match variable {
        PlaceExpression::Index(place, index_expr) => match &**place {
            PlaceExpression::Index(_, _) => todo!(),
            PlaceExpression::MemberAccess(member_access) => {
                let array = eval_member_access(None, member_access, scopes, current_scope, ctx)?;

                let index = if let Value::Num(index) =
                    eval_expr(index_expr, scopes, current_scope, enclosing_scope, ctx)?
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
                if scopes.binding_exists(&ident, current_scope, enclosing_scope) {
                    let index_result =
                        eval_expr(&index_expr, scopes, current_scope, enclosing_scope, ctx)?;

                    if let Value::Void = value {
                        panic!("Cannot assign a value of type Void.");
                    }

                    let index = if let Value::Num(index) = index_result {
                        index
                    } else {
                        panic!("Index does not resolve to a Number.");
                    };

                    let array = scopes.get_value(&ident, current_scope, enclosing_scope)?;

                    if let Value::Array(array) = array {
                        let array = Arc::clone(&array);
                        let mut array = array.lock().unwrap();
                        array[index as usize] = value.clone();
                    } else {
                        panic!("Trying to index into a non-Array.")
                    }
                }
            }
        },
        PlaceExpression::Symbol(ident) => {
            // TODO: Do we need to call binding_exists? Can we just call get_value?
            if scopes.binding_exists(&ident, current_scope, enclosing_scope) {
                if let Value::Void = value {
                    panic!("Cannot assign a value of type Void.");
                }

                let existing = scopes.get_value(&ident, current_scope, enclosing_scope)?;

                let existing_type = existing.get_type(scopes, current_scope)?;
                let value_type = value.get_type(scopes, current_scope)?;

                if existing_type == value_type {
                    return scopes.mutate_value(&ident, current_scope, value.clone());
                } else {
                    return Err(RuntimeError::new(&format!(
                    "Cannot assign a value of type {value_type} where {existing_type} is expected.",
                )));
                }
            } else {
                panic!("Unknown identifier `{}`", ident);
            }
        }
        PlaceExpression::MemberAccess(member_access) => {
            let (parent, child) = match &**member_access {
                MemberAccess::MemberAccesses(parents, child) => (
                    eval_member_access(None, &*parents, scopes, current_scope, ctx)?,
                    child,
                ),
                MemberAccess::MemberAccess(parent, child) => (
                    scopes.get_value(&parent, current_scope, enclosing_scope)?,
                    child,
                ),
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
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match variable {
        PlaceExpression::Index(place, expr) => {
            let array = eval_place_expr(place, scopes, current_scope, enclosing_scope, ctx)?;
            eval_index(&array, expr, scopes, current_scope, enclosing_scope, ctx)
        }
        PlaceExpression::Symbol(ident) => scopes.get_value(ident, current_scope, enclosing_scope),
        PlaceExpression::MemberAccess(member_access) => {
            eval_member_access(None, member_access, scopes, current_scope, ctx)
        }
    }
}

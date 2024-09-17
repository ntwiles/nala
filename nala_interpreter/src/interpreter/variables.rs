use std::sync::Arc;

use super::{arrays::eval_index, eval_expr, objects::*};

use crate::{
    ast::{types::type_literal_variant::TypeVariantLiteral, *},
    errors::RuntimeError,
    io_context::IoContext,
    resolved::value::Value,
    scopes::Scopes,
    types::{fit::fits_type, inference::infer_type, type_variant::TypeVariant},
};

pub fn eval_declare(
    ident: &str,
    value: Value,
    declared_type: Option<TypeVariantLiteral>,
    is_mutable: bool,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    if let Value::Void = value {
        return Err(RuntimeError::new(
            "Cannot declare a variable with a value of type Void.",
        ));
    }

    if let Some(declared_type_literal) = declared_type {
        let declared_type =
            TypeVariant::from_literal(declared_type_literal.clone(), scopes, current_scope)?;

        if !fits_type(&value, &declared_type, scopes, current_scope)? {
            let value_type = infer_type(&value, scopes, current_scope)?;
            return Err(RuntimeError::new(&format!(
                "Tried to declare variable `{ident}` with explicit type `{declared_type_literal}` but value `{value}` of type `{value_type}` does not fit that type.",
            )));
        }

        scopes.add_binding(
            &ident,
            value,
            Some(declared_type),
            current_scope,
            is_mutable,
        )
    } else {
        let inferred_type = infer_type(&value, scopes, current_scope)?;

        if inferred_type.find_generic_type_param().is_some() {
            return Err(RuntimeError::new(&format!(
                "Can't assign value of type `{inferred_type}` because its concrete type cannot be determined. Try declaring the type explicitly.",
            )));
        } else {
            scopes.add_binding(&ident, value, None, current_scope, is_mutable)
        }
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
            PlaceExpression::MemberAccess(place_expression, member) => {
                let parent_obj = eval_place_expr(place_expression, scopes, current_scope, ctx)?;
                let array = eval_member_access(&parent_obj, member)?;

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
                    Err(RuntimeError::new("Trying to index into a non-Array."))?
                }
            }
            PlaceExpression::Identifier(ident) => {
                if scopes.binding_exists(&ident, current_scope) {
                    let index_result = eval_expr(&index_expr, scopes, current_scope, ctx)?;

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
        PlaceExpression::Identifier(ident) => {
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
        PlaceExpression::MemberAccess(place_expression, member) => {
            let parent = eval_place_expr(place_expression, scopes, current_scope, ctx)?;

            if let Value::Object(parent) = parent {
                let parent = Arc::clone(&parent);
                let mut parent = parent.lock().unwrap();
                parent.insert(member.to_string(), value.clone());
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
        PlaceExpression::Identifier(ident) => scopes.get_value(ident, current_scope),
        PlaceExpression::MemberAccess(place_expression, member_access) => {
            let object = eval_place_expr(place_expression, scopes, current_scope, ctx)?;
            eval_member_access(&object, member_access)
        }
    }
}

use crate::{
    ast::types::primitive_type::PrimitiveType,
    errors::*,
    interpreter::operations::errors::*,
    resolved::value::Value,
    scopes::Scopes,
    types::{inference::infer_type, type_variant::TypeVariant, NalaType},
};

pub fn eval_lt(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    match left {
        Value::Num(left) => Ok(num_lt(left, right, scopes, current_scope)?),
        Value::String(left) => Ok(string_lt(left, right, scopes, current_scope)?),
        Value::Bool(left) => Ok(bool_lt(left, right, scopes, current_scope)?),
        left => {
            let left_type = infer_type(&left, scopes, current_scope)?;
            Err(oper_not_implemented_error("<", &left_type))
        }
    }
}

fn num_lt(
    left: f32,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let result = match right {
        Value::Num(right) => Value::Bool(left < right),
        right => {
            let right_type = infer_type(&right, scopes, current_scope)?;
            Err(oper_not_implemented_for_error(
                "<",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number, None)),
                &right_type,
            ))?
        }
    };

    Ok(result)
}

fn string_lt(
    left: String,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let result = match right {
        Value::String(right) => Value::Bool(left < right),
        right => {
            let right_type = infer_type(&right, scopes, current_scope)?;
            Err(oper_not_implemented_for_error(
                "<",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String, None)),
                &right_type,
            ))?
        }
    };

    Ok(result)
}

fn bool_lt(
    left: bool,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let result = match right {
        Value::Bool(right) => Value::Bool(left < right),
        right => {
            let right_type = infer_type(&right, scopes, current_scope)?;
            Err(oper_not_implemented_for_error(
                "<",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool, None)),
                &right_type,
            ))?
        }
    };

    Ok(result)
}

use crate::{
    ast::types::primitive_type::PrimitiveType,
    errors::*,
    resolved::value::Value,
    scopes::Scopes,
    types::{inference::infer_type, nala_type::NalaType, type_variant::TypeVariant},
};

use super::errors::*;

pub fn eval_gt(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    match left {
        Value::Num(left) => match right {
            Value::Num(right) => Ok(Value::Bool(left > right)),
            right => Err(oper_not_implemented_for_error(
                ">",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
                &infer_type(&right, scopes, current_scope)?,
            )),
        },
        Value::String(left) => match right {
            Value::String(right) => Ok(Value::Bool(left > right)),
            right => Err(oper_not_implemented_for_error(
                ">",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
                &infer_type(&right, scopes, current_scope)?,
            )),
        },
        left => {
            let left_type = infer_type(&left, scopes, current_scope)?;
            Err(oper_not_implemented_error(">", &left_type))
        }
    }
}

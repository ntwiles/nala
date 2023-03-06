use crate::{
    ast::{terms::*, types::primitive_type::PrimitiveType},
    errors::*,
    scopes::Scopes,
    types::{type_variant::TypeVariant, NalaType},
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
            right => panic_oper_not_impl_for(
                ">",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
                &right.infer_type(scopes, current_scope)?,
            ),
        },
        Value::String(left) => match right {
            Value::String(right) => Ok(Value::Bool(left > right)),
            right => panic_oper_not_impl_for(
                ">",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
                &right.infer_type(scopes, current_scope)?,
            ),
        },
        left => {
            let left_type = left.infer_type(scopes, current_scope)?;
            panic_oper_not_impl(">", &left_type)
        }
    }
}

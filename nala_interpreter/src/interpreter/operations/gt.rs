use crate::{
    ast::{terms::*, types::*},
    errors::*,
};

use super::errors::*;

pub fn evaluate_gt(left: Value, right: Value) -> Result<Value, NalaRuntimeError> {
    match left {
        Value::Num(left) => match right {
            Value::Num(right) => Ok(Value::Bool(left > right)),
            right => panic_oper_not_impl_for(
                ">",
                &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
                &right.get_type(),
            ),
        },
        Value::String(left) => match right {
            Value::String(right) => Ok(Value::Bool(left > right)),
            right => panic_oper_not_impl_for(
                ">",
                &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::String)),
                &right.get_type(),
            ),
        },
        left => panic_oper_not_impl(">", &left.get_type()),
    }
}

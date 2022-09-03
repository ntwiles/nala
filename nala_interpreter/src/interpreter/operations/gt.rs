use crate::{
    ast::{
        terms::*,
        types::{nala_type::NalaType, primitive_type::PrimitiveType, type_variant::TypeVariant},
    },
    errors::*,
};

use super::errors::*;

pub fn eval_gt(left: Value, right: Value) -> Result<Value, NalaRuntimeError> {
    match left {
        Value::Num(left) => match right {
            Value::Num(right) => Ok(Value::Bool(left > right)),
            right => panic_oper_not_impl_for(
                ">",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
                &right.get_type(),
            ),
        },
        Value::String(left) => match right {
            Value::String(right) => Ok(Value::Bool(left > right)),
            right => panic_oper_not_impl_for(
                ">",
                &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
                &right.get_type(),
            ),
        },
        left => panic_oper_not_impl(">", &left.get_type()),
    }
}

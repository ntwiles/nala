use crate::{
    ast::{
        terms::*,
        types::{PrimitiveInterface::*, *},
    },
    errors::*,
    interpreter::operations::errors::*,
};

pub fn evaluate_lt(left: Value, right: Value) -> Result<Value, NalaRuntimeError> {
    check_operator_implemented_both(left.get_type(), right.get_type(), ">".to_string(), ICompare)?;

    match left {
        Value::Num(left) => Ok(num_lt(left, right)),
        Value::String(left) => Ok(string_lt(left, right)),
        Value::Bool(left) => Ok(bool_lt(left, right)),
        left => panic_oper_not_impl("<", &left.get_type()),
    }
}

fn num_lt(left: f32, right: Value) -> Value {
    match right {
        Value::Num(right) => Value::Bool(left < right),
        right => panic_oper_not_impl_for(
            "<",
            &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
            &right.get_type(),
        ),
    }
}

fn string_lt(left: String, right: Value) -> Value {
    match right {
        Value::String(right) => Value::Bool(left < right),
        right => panic_oper_not_impl_for(
            "<",
            &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::String)),
            &right.get_type(),
        ),
    }
}

fn bool_lt(left: bool, right: Value) -> Value {
    match right {
        Value::Bool(right) => Value::Bool(left < right),
        right => panic_oper_not_impl_for(
            "<",
            &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Bool)),
            &right.get_type(),
        ),
    }
}

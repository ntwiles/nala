use crate::{
    ast::{
        terms::*,
        types::{nala_type::NalaType, primitive_type::PrimitiveType, type_variant::TypeVariant},
    },
    errors::*,
    interpreter::operations::errors::*,
};

pub fn eval_lt(left: Value, right: Value) -> Result<Value, NalaRuntimeError> {
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
            &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
            &right.get_type(),
        ),
    }
}

fn string_lt(left: String, right: Value) -> Value {
    match right {
        Value::String(right) => Value::Bool(left < right),
        right => panic_oper_not_impl_for(
            "<",
            &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
            &right.get_type(),
        ),
    }
}

fn bool_lt(left: bool, right: Value) -> Value {
    match right {
        Value::Bool(right) => Value::Bool(left < right),
        right => panic_oper_not_impl_for(
            "<",
            &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool)),
            &right.get_type(),
        ),
    }
}

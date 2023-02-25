use crate::{
    ast::{terms::*, types::primitive_type::PrimitiveType},
    errors::*,
    interpreter::operations::errors::*,
    scopes::Scopes,
    types::{type_variant::TypeVariant, NalaType},
};

pub fn eval_lt(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, NalaRuntimeError> {
    match left {
        Value::Num(left) => Ok(num_lt(left, right, scopes, current_scope)),
        Value::String(left) => Ok(string_lt(left, right, scopes, current_scope)),
        Value::Bool(left) => Ok(bool_lt(left, right, scopes, current_scope)),
        left => panic_oper_not_impl("<", &left.get_type(scopes, current_scope)),
    }
}

fn num_lt(left: f32, right: Value, scopes: &mut Scopes, current_scope: usize) -> Value {
    match right {
        Value::Num(right) => Value::Bool(left < right),
        right => panic_oper_not_impl_for(
            "<",
            &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
            &right.get_type(scopes, current_scope),
        ),
    }
}

fn string_lt(left: String, right: Value, scopes: &mut Scopes, current_scope: usize) -> Value {
    match right {
        Value::String(right) => Value::Bool(left < right),
        right => panic_oper_not_impl_for(
            "<",
            &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
            &right.get_type(scopes, current_scope),
        ),
    }
}

fn bool_lt(left: bool, right: Value, scopes: &mut Scopes, current_scope: usize) -> Value {
    match right {
        Value::Bool(right) => Value::Bool(left < right),
        right => panic_oper_not_impl_for(
            "<",
            &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool)),
            &right.get_type(scopes, current_scope),
        ),
    }
}

use crate::ast::{terms::*, types::*};

use super::errors::{panic_oper_not_impl, panic_oper_not_impl_for};

pub fn evaluate_equals(left: Value, right: Value) -> Value {
    match left {
        Value::Num(left) => num_equals(left, right),
        Value::String(left) => string_equals(left, right),
        Value::Bool(left) => bool_equals(left, right),
        Value::Variant(left_enum, left_variant, data) => {
            variant_equals(left_enum, left_variant, data, right)
        }
        other => panic_oper_not_impl("==", &other.get_type()),
    }
}

fn num_equals(left: f32, right: Value) -> Value {
    if let Value::Num(right) = right {
        Value::Bool(left == right)
    } else {
        panic_oper_not_impl_for(
            "==",
            &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
            &right.get_type(),
        )
    }
}

fn string_equals(left: String, right: Value) -> Value {
    if let Value::String(right) = right {
        Value::Bool(left == right)
    } else {
        panic_oper_not_impl_for(
            "==",
            &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::String)),
            &right.get_type(),
        )
    }
}

fn bool_equals(left: bool, right: Value) -> Value {
    if let Value::Bool(right) = right {
        Value::Bool(left == right)
    } else {
        panic_oper_not_impl_for(
            "==",
            &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Bool)),
            &right.get_type(),
        )
    }
}

fn variant_equals(
    left_enum: String,
    left_variant: String,
    left_data: Option<Box<Value>>,
    right: Value,
) -> Value {
    if let Value::Variant(right_enum, right_variant, right_data) = right {
        let enums_match = left_enum == right_enum;
        let variants_match = left_variant == right_variant;

        if let Some(right_data) = right_data {
            let data_matches = if let Some(left_data) = left_data {
                left_data == right_data
            } else {
                false
            };

            Value::Bool(enums_match && variants_match && data_matches)
        } else {
            Value::Bool(enums_match && variants_match)
        }
    } else {
        // TODO: Using PrimitiveType::String as placeholder. Correct this.
        panic_oper_not_impl_for(
            "==",
            &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::String)),
            &right.get_type(),
        )
    }
}

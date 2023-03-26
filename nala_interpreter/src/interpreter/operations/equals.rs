use crate::{
    ast::types::primitive_type::PrimitiveType,
    errors::RuntimeError,
    resolved::value::{EnumVariantValue, Value},
    scopes::Scopes,
    types::{inference::infer_type, type_variant::TypeVariant, NalaType},
};

use super::errors::oper_not_implemented_for_error;

pub fn eval_equals(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    if infer_type(&left, scopes, current_scope)? != infer_type(&right, scopes, current_scope)? {
        Err(oper_not_implemented_for_error(
            "==",
            &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number, None)),
            &infer_type(&right, scopes, current_scope)?,
        ))?
    }

    let result = match left {
        Value::Variant(EnumVariantValue {
            enum_ident: left_enum,
            variant_ident: left_variant,
            data,
        }) => variant_equals(left_enum, left_variant, data, right, scopes, current_scope)?,
        _ => Value::Bool(left == right),
    };

    Ok(result)
}

fn variant_equals(
    left_enum: String,
    left_variant: String,
    left_data: Option<Box<Value>>,
    right: Value,
    _scopes: &mut Scopes,
    _current_scope: usize,
) -> Result<Value, RuntimeError> {
    if let Value::Variant(EnumVariantValue {
        enum_ident: right_enum,
        variant_ident: right_variant,
        data: right_data,
    }) = right
    {
        let enums_match = left_enum == right_enum;
        let variants_match = left_variant == right_variant;

        if let Some(right_data) = right_data {
            let data_matches = if let Some(left_data) = left_data {
                left_data == right_data
            } else {
                false
            };

            Ok(Value::Bool(enums_match && variants_match && data_matches))
        } else {
            Ok(Value::Bool(enums_match && variants_match))
        }
    } else {
        todo!("Implement this error once infer_type is working in this case.")
        // Err(oper_not_implemented_for_error(
        //     "==",
        //     &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Symbol)),
        //     &infer_type(&right, scopes, current_scope)?,
        // ))?
    }
}

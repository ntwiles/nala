use crate::{
    ast::types::primitive_type::PrimitiveType,
    errors::RuntimeError,
    resolved::{func_value::FuncValue, struct_field::StructField, value::Value},
    scopes::Scopes,
};

use super::{
    composite_type::CompositeType, inference::infer_type, nala_type::NalaType,
    type_variant::TypeVariant,
};

pub fn fits_type(
    value: &Value,
    type_variant: &TypeVariant,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    match type_variant {
        TypeVariant::Composite(CompositeType {
            outer,
            inner,
            generic_type_param: _,
        }) => match outer {
            NalaType::PrimitiveType(PrimitiveType::Array) => {
                fits_array(inner, value, scopes, current_scope)
            }
            NalaType::PrimitiveType(PrimitiveType::Break) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Func) => fits_func(inner, value),
            NalaType::Enum(_, _) => fits_enum(value, type_variant, inner, scopes, current_scope),
            NalaType::Struct(fields) => fits_struct(fields, value, scopes, current_scope),
            NalaType::Generic(_) => todo!(),
            _ => Err(RuntimeError::new(&format!(
                "Type `{outer}` does not support type arguments. Type `{outer}<{}>` is invalid.",
                inner[0] // TODO: We're just assuming there's only one type arg, this will be wrong later.
            )))?,
        },
        TypeVariant::Type(the_type) => match the_type {
            NalaType::PrimitiveType(PrimitiveType::Bool) => Ok(value.is_bool()),
            NalaType::PrimitiveType(PrimitiveType::Number) => Ok(value.is_number()),
            NalaType::PrimitiveType(PrimitiveType::String) => Ok(value.is_string()),
            NalaType::PrimitiveType(PrimitiveType::Void) => Ok(value.is_void()),
            NalaType::Struct(fields) => fits_struct(fields, value, scopes, current_scope),
            NalaType::Generic(_ident) => Ok(true),
            NalaType::Enum(_, _) => fits_enum(
                value,
                type_variant,
                &vec![type_variant.clone()],
                scopes,
                current_scope,
            ),
            _ => unreachable!(), // The remaining primitive types are composite only.
        },
    }
}

fn fits_array(
    inner: &Vec<TypeVariant>,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    if let Value::Array(items) = value {
        let items = items.clone();
        let items = items.lock().unwrap();
        let first = items.first();

        if let Some(first) = first {
            Ok(fits_type(first, &inner[0], scopes, current_scope)?)
        } else {
            // Empty array, fits any type.
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

fn fits_func(inner: &Vec<TypeVariant>, value: &Value) -> Result<bool, RuntimeError> {
    if let Value::Func(FuncValue { return_type, .. }) = value {
        Ok(return_type == &inner.last().unwrap().clone())
    } else {
        Ok(false)
    }
}

fn fits_enum(
    value: &Value,
    enum_type: &TypeVariant,
    expected_data_types: &Vec<TypeVariant>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    if let Value::Variant(variant) = value {
        let value_type = infer_type(value, scopes, current_scope)?;

        match enum_type {
            TypeVariant::Composite(enum_type) => {
                if let TypeVariant::Composite(value_type) = value_type {
                    let outer_fits = enum_outer_fits(&enum_type.outer, &value_type.outer)?;

                    let generic_params_match = value_type.generic_type_param.is_some()
                        || value_type.generic_type_param == enum_type.generic_type_param;

                    let data_fits = enum_data_fits(
                        &expected_data_types[0],
                        &variant.data,
                        scopes,
                        current_scope,
                    )?;

                    Ok(outer_fits && generic_params_match && data_fits)
                } else {
                    Ok(false)
                }
            }
            enum_type => Ok(enum_type == &value_type),
        }
    } else {
        Ok(false)
    }
}

fn fits_struct(
    expected_fields: &Vec<StructField>,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    if let Value::Object(fields) = value {
        let fields = fields.clone();
        let fields = fields.lock().unwrap();

        for (ident, value) in fields.iter() {
            if let Some(expected_field) = expected_fields.iter().find(|f| &f.ident == ident) {
                if !fits_type(value, &expected_field.value_type, scopes, current_scope)? {
                    return Ok(false);
                }
            } else {
                return Ok(true);
            }
        }

        Ok(true)
    } else {
        Ok(false)
    }
}

fn enum_data_fits(
    expected_type: &TypeVariant,
    data: &Option<Box<Value>>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    if let Some(data) = data {
        Ok(fits_type(data, &expected_type, scopes, current_scope))?
    } else {
        Ok(true)
    }
}

fn enum_outer_fits(enum_outer: &NalaType, value_outer: &NalaType) -> Result<bool, RuntimeError> {
    match enum_outer {
        NalaType::Enum(outer_ident, _) => {
            if let NalaType::Enum(value_ident, _) = value_outer {
                Ok(outer_ident == value_ident)
            } else {
                Ok(false)
            }
        }
        _ => unreachable!("Enum outer type should always be an enum."),
    }
}

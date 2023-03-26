use crate::{
    ast::types::primitive_type::PrimitiveType,
    errors::RuntimeError,
    resolved::{
        enum_variants::EnumVariant, func_value::FuncValue, struct_field::StructField, value::Value,
    },
    scopes::Scopes,
};

use super::{composite_type::CompositeType, type_variant::TypeVariant, NalaType};

pub fn fits_type(
    value: &Value,
    type_variant: &TypeVariant,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    match type_variant {
        TypeVariant::Composite(CompositeType { outer, inner, .. }) => match outer {
            NalaType::PrimitiveType(PrimitiveType::Array) => {
                fits_array(inner, value, scopes, current_scope)
            }
            NalaType::PrimitiveType(PrimitiveType::Break) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Func) => fits_func(inner, value),
            NalaType::Enum(enum_ident, variants, _type_param) => {
                fits_enum(value, inner, enum_ident, variants, scopes, current_scope)
            }
            NalaType::Struct(fields, _type_param) => {
                fits_struct(fields, value, scopes, current_scope)
            }
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
            NalaType::Struct(fields, _type_param) => {
                fits_struct(fields, value, scopes, current_scope)
            }
            NalaType::Generic(_ident) => Ok(true),
            NalaType::Enum(enum_ident, variants, _type_param) => fits_enum(
                value,
                &vec![type_variant.clone()],
                enum_ident,
                variants,
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
    expected_data_types: &Vec<TypeVariant>,
    enum_ident: &str,
    variants: &Vec<EnumVariant>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    if let Value::Variant(value) = value {
        match find_variant(&value.variant_ident, variants) {
            Some(EnumVariant::Data(_, _)) => Ok(enum_ident == value.enum_ident
                && data_fits(&expected_data_types[0], &value.data, scopes, current_scope)?),
            Some(EnumVariant::Empty(_)) => Ok(enum_ident == value.enum_ident),
            None => Ok(false),
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

fn data_fits(
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

fn find_variant<'a>(
    variant_ident: &str,
    variants: &'a Vec<EnumVariant>,
) -> Option<&'a EnumVariant> {
    variants.iter().find(|v| match v {
        EnumVariant::Data(ident, _) => ident == variant_ident,
        EnumVariant::Empty(ident) => ident == variant_ident,
    })
}

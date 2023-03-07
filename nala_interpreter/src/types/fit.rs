use crate::{
    ast::{
        terms::{StoredFunc, Value},
        types::{primitive_type::PrimitiveType, variant_declare::VariantDeclare},
    },
    errors::RuntimeError,
    scopes::Scopes,
};

use super::{inference::infer_type, type_variant::TypeVariant, NalaType};

pub fn fits_type(
    value: &Value,
    variant: &TypeVariant,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    match variant {
        TypeVariant::Generic(outer, inner) => match outer {
            NalaType::PrimitiveType(PrimitiveType::Any) => Ok(true),
            NalaType::PrimitiveType(PrimitiveType::Array) => {
                fits_array(inner, value, scopes, current_scope)
            }
            NalaType::PrimitiveType(PrimitiveType::Bool) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Break) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Func) => {
                fits_func(inner, value, scopes, current_scope)
            }
            NalaType::PrimitiveType(PrimitiveType::Number) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Object) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::String) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Symbol) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Void) => todo!(),
            NalaType::Enum(enum_ident, variants) => {
                fits_enum(inner, enum_ident, variants, value, scopes, current_scope)
            }
            NalaType::Struct(_fields) => todo!(),
        },
        TypeVariant::Type(_the_type) => Ok(&infer_type(value, scopes, current_scope)? == variant),
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
            Ok(infer_type(first, scopes, current_scope)? == inner[0])
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

fn fits_func(
    inner: &Vec<TypeVariant>,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    if let Value::Func(StoredFunc { return_type, .. }) = value {
        Ok(TypeVariant::from_literal(return_type.clone(), scopes, current_scope)? == inner[0])
    } else {
        Ok(false)
    }
}

fn fits_enum(
    inner: &Vec<TypeVariant>,
    enum_ident: &str,
    variants: &Vec<VariantDeclare>,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    if let Value::Variant(value) = value {
        match find_variant(&value.variant_ident, variants) {
            Some(VariantDeclare::Data(_, _)) => Ok(enum_ident == value.enum_ident
                && data_fits(&inner[0], &value.data, scopes, current_scope)?),
            Some(VariantDeclare::Empty(_)) => Ok(enum_ident == value.enum_ident),
            None => Ok(false),
        }
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
    println!("Got here.");
    if let Some(data) = data {
        println!("Checking data fits type: {:?} == {:?}", data, expected_type);
        Ok(fits_type(data, &expected_type, scopes, current_scope))?
    } else {
        Ok(true)
    }
}

fn find_variant<'a>(
    variant_ident: &str,
    variants: &'a Vec<VariantDeclare>,
) -> Option<&'a VariantDeclare> {
    variants.iter().find(|v| match v {
        VariantDeclare::Data(ident, _) => ident == variant_ident,
        VariantDeclare::Empty(ident) => ident == variant_ident,
    })
}

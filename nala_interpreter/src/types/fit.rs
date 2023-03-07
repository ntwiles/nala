use crate::{
    ast::{
        terms::Value,
        types::{primitive_type::PrimitiveType, variant_declare::VariantDeclare},
    },
    errors::RuntimeError,
    scopes::Scopes,
};

use super::{inference::infer_type, type_variant::TypeVariant, NalaType};

pub fn fits_type(
    value: &Value,
    type_variant: &TypeVariant,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    match type_variant {
        TypeVariant::Generic(outer, inner) => match outer {
            NalaType::PrimitiveType(PrimitiveType::Any) => Ok(true),
            NalaType::PrimitiveType(PrimitiveType::Array) => {
                fits_array(inner, value, scopes, current_scope)
            }
            NalaType::PrimitiveType(PrimitiveType::Bool) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Break) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Func) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Number) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Object) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::String) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Symbol) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Void) => todo!(),
            NalaType::Enum(enum_ident, variants) => fits_enum(enum_ident, variants, value),
            NalaType::Struct(_fields) => todo!(),
        },
        TypeVariant::Type(_the_type) => {
            Ok(&infer_type(value, scopes, current_scope)? == type_variant)
        }
    }
}

fn fits_array(
    item_types: &Vec<TypeVariant>,
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    if let Value::Array(items) = value {
        let items = items.clone();
        let items = items.lock().unwrap();
        let first = items.first();

        if let Some(first) = first {
            Ok(infer_type(first, scopes, current_scope)? == item_types[0])
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

fn fits_enum(
    enum_ident: &str,
    variants: &Vec<VariantDeclare>,
    value: &Value,
) -> Result<bool, RuntimeError> {
    if let Value::Variant(value) = value {
        Ok(
            enum_ident == value.enum_ident
                && find_variant(&value.variant_ident, variants).is_some(),
        )
    } else {
        Ok(false)
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

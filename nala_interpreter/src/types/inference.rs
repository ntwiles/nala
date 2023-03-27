use std::sync::{Arc, Mutex};

use crate::{
    ast::types::primitive_type::PrimitiveType,
    errors::RuntimeError,
    interpreter::enums::find_variant,
    resolved::{
        enum_variants::EnumVariant,
        func_value::FuncValue,
        struct_field::StructField,
        value::{EnumVariantValue, Value},
    },
    scopes::Scopes,
};

use super::{
    composite_type::CompositeType, fit::fits_type, nala_type::NalaType, type_variant::TypeVariant,
};

pub fn infer_type(
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<TypeVariant, RuntimeError> {
    let result = match value {
        Value::Array(items) => infer_array(items, scopes, current_scope)?,
        Value::Bool(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool)),
        Value::Break(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Break)),
        Value::Func(FuncValue {
            params,
            return_type,
            ..
        }) => {
            let mut param_types: Vec<TypeVariant> =
                params.into_iter().map(|p| p.clone().param_type).collect();

            param_types.push(return_type.clone());

            TypeVariant::Composite(CompositeType {
                outer: NalaType::PrimitiveType(PrimitiveType::Func),
                inner: param_types,
                generic_type_param: None,
            })
        }
        Value::Num(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
        Value::Object(fields) => {
            let fields = fields
                .lock()
                .unwrap()
                .clone()
                .iter()
                .map(|(ident, v)| StructField {
                    ident: ident.clone(),
                    value_type: infer_type(v, scopes, current_scope).unwrap(),
                })
                .collect();

            TypeVariant::Type(NalaType::Struct(fields))
        }
        Value::String(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
        Value::Variant(variant) => infer_variant(variant, scopes, current_scope)?,
        Value::Void => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Void)),
    };

    Ok(result)
}

fn infer_array(
    items: &Arc<Mutex<Vec<Value>>>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<TypeVariant, RuntimeError> {
    let items = Arc::clone(&items);
    let items = items.lock().unwrap();

    let elem_type = if items.len() > 0 {
        let first = items.first();
        infer_type(first.unwrap(), scopes, current_scope)?
    } else {
        drop(items);
        Err(RuntimeError::new(&format!(
            "Cannot infer type of an empty array."
        )))?
    };

    Ok(TypeVariant::Composite(CompositeType {
        outer: NalaType::PrimitiveType(PrimitiveType::Array),
        inner: vec![elem_type],
        generic_type_param: None,
    }))
}

pub fn infer_variant(
    variant: &EnumVariantValue,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<TypeVariant, RuntimeError> {
    let EnumVariantValue {
        enum_ident,
        variant_ident,
        data,
    } = variant;

    // TODO: This appears to be the only reason that infer_type needs to accept
    // scopes and current_scope. Another clue that maybe enum variant values should
    // contain type information.
    let (enum_type, generic_type_param) = scopes.get_type(&enum_ident, current_scope)?.as_enum()?;

    let existing_variant = find_variant(&enum_type.variants, variant_ident)?;

    match existing_variant {
        EnumVariant::Data(_ident, data_type) => {
            let data = match data {
                Some(d) => d,
                None => todo!("Expected data but none was supplied error."),
            };

            if fits_type(data, &data_type, scopes, current_scope)? {
                if let Some(generic_type_param) = generic_type_param {
                    let inner_type = if let Some(ident) = data_type.find_generic_type_param() {
                        if ident == generic_type_param {
                            infer_type(data, scopes, current_scope)?
                        } else {
                            unreachable!("This is currently unreachable because we don't support multiple generic types.")
                        }
                    } else {
                        TypeVariant::Type(NalaType::Generic(generic_type_param.clone()))
                    };

                    Ok(TypeVariant::Composite(CompositeType {
                        outer: NalaType::Enum(enum_ident.to_owned(), enum_type.variants),
                        inner: vec![inner_type],
                        generic_type_param: Some(generic_type_param),
                    }))
                } else {
                    Ok(TypeVariant::Type(NalaType::Enum(
                        enum_ident.to_owned(),
                        enum_type.variants,
                    )))
                }
            } else {
                todo!("")
            }
        }
        EnumVariant::Empty(_ident) => Ok(TypeVariant::Type(NalaType::Enum(
            enum_ident.to_owned(),
            enum_type.variants,
        ))),
    }
}

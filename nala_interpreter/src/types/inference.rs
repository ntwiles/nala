use std::sync::{Arc, Mutex};

use crate::{
    ast::types::{primitive_type::PrimitiveType, variant_declare::VariantDeclare},
    errors::RuntimeError,
    resolved::{
        func_value::FuncValue,
        value::{EnumVariantValue, Value},
    },
    scopes::Scopes,
    utils::accept_results,
};

use super::{
    enum_variants::EnumVariant, fit::fits_type, struct_field::StructField,
    type_variant::TypeVariant, NalaType,
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

            TypeVariant::Composite(NalaType::PrimitiveType(PrimitiveType::Func), param_types)
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

    Ok(TypeVariant::Composite(
        NalaType::PrimitiveType(PrimitiveType::Array),
        vec![elem_type],
    ))
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
    let enum_type = scopes
        .get_type(&enum_ident, current_scope)?
        .as_enum()
        .unwrap();

    let existing_variant = enum_type.variants.iter().find(|v| match v {
        VariantDeclare::Data(ident, _) => ident == variant_ident,
        VariantDeclare::Empty(ident) => ident == variant_ident,
    });

    let existing_variant = match existing_variant {
        Some(v) => v,
        None => todo!("Enum variant not found"),
    };

    match existing_variant {
        VariantDeclare::Data(_ident, data_type) => {
            let expected_data_type =
                TypeVariant::from_literal(data_type.clone(), scopes, enum_type.closure_scope)?;

            let data = match data {
                Some(d) => d,
                None => todo!("Expected data but none was supplied error."),
            };

            if fits_type(data, &expected_data_type, scopes, current_scope)? {
                if let Some(generic_ident) = enum_type.get_generic_ident() {
                    let inner_type = if let Some(ident) = expected_data_type.get_generic_ident() {
                        if ident == generic_ident {
                            infer_type(data, scopes, current_scope)?
                        } else {
                            unreachable!("This is currently unreachable because we don't support multiple generic types.")
                        }
                    } else {
                        TypeVariant::Type(NalaType::Generic(generic_ident))
                    };

                    let variants = enum_type
                        .variants
                        .iter()
                        .map(|v| {
                            EnumVariant::from_variant_declare(
                                v.clone(), // TODO: Find a way to avoid this clone.
                                scopes,
                                enum_type.closure_scope,
                            )
                        })
                        .collect();

                    let variants = accept_results(variants)?;

                    Ok(TypeVariant::Composite(
                        NalaType::Enum(enum_ident.to_owned(), variants),
                        vec![inner_type],
                    ))
                } else {
                    let variants = enum_type
                        .variants
                        .iter()
                        .map(|v| {
                            EnumVariant::from_variant_declare(
                                v.clone(), // TODO: Find a way to avoid this clone.
                                scopes,
                                enum_type.closure_scope,
                            )
                        })
                        .collect();

                    let variants = accept_results(variants)?;

                    Ok(TypeVariant::Type(NalaType::Enum(
                        enum_ident.to_owned(),
                        variants,
                    )))
                }
            } else {
                todo!("")
            }
        }
        VariantDeclare::Empty(_ident) => {
            if enum_type.get_generic_ident().is_some() {
                // TODO: I don't think we want to error here, we're moved towards letting inferrence
                // happen for generic types and erroring instead on assignment or call.
                Err(RuntimeError::new(&format!(
                    "Not enough information to infer type of generic enum variant."
                )))
            } else {
                let variants = enum_type
                    .variants
                    .iter()
                    .map(|v| {
                        EnumVariant::from_variant_declare(
                            v.clone(), // TODO: Find a way to avoid this clone.
                            scopes,
                            enum_type.closure_scope,
                        )
                    })
                    .collect();

                let variants = accept_results(variants)?;

                Ok(TypeVariant::Type(NalaType::Enum(
                    enum_ident.to_owned(),
                    variants,
                )))
            }
        }
    }
}

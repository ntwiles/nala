use std::sync::{Arc, Mutex};

use crate::{
    ast::{
        terms::{EnumVariantValue, FuncValue, Value},
        types::{
            primitive_type::PrimitiveType, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant, variant_declare::VariantDeclare, TypeArgs,
        },
    },
    errors::RuntimeError,
    scopes::Scopes,
    utils::accept_results,
};

use super::{struct_field::StructField, type_variant::TypeVariant, NalaType};

pub fn infer_type(
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<TypeVariant, RuntimeError> {
    let result = match value {
        Value::Array(items) => infer_array(value, items, scopes, current_scope)?,
        Value::Bool(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool)),
        Value::Break(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Break)),
        Value::Func(FuncValue {
            params,
            return_type,
            ..
        }) => {
            let param_types = params
                .into_iter()
                .map(|p| TypeVariant::from_literal(p.clone().param_type, scopes, current_scope))
                .collect();

            let mut param_types = accept_results(param_types)?;

            param_types.push(TypeVariant::from_literal(
                return_type.clone(),
                scopes,
                current_scope,
            )?);

            TypeVariant::Generic(NalaType::PrimitiveType(PrimitiveType::Func), param_types)
        }
        Value::Type(_) => todo!("What is this?"),
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
        Value::Variant(variant) => infer_variant(value, variant, scopes, current_scope)?,
        Value::Void => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Void)),
    };

    Ok(result)
}

fn infer_array(
    raw_value: &Value,
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
        Err(cannot_infer_value_error(raw_value))?
    };

    Ok(TypeVariant::Generic(
        NalaType::PrimitiveType(PrimitiveType::Array),
        vec![elem_type],
    ))
}

fn infer_variant(
    raw_value: &Value,
    variant: &EnumVariantValue,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<TypeVariant, RuntimeError> {
    let EnumVariantValue {
        enum_ident,
        variant_ident,
        data,
    } = variant;

    let (variants, type_arg) = scopes
        .get_type(&enum_ident, current_scope)?
        .as_enum()
        .unwrap();

    // TODO: Absolute mess, redo all this.
    if let Some(TypeArgs::Generic(type_arg)) = type_arg {
        if let Some(data) = data {
            let found_variant = variants.iter().find(|v| match v {
                VariantDeclare::Data(ident, _) => ident == variant_ident,
                VariantDeclare::Empty(ident) => ident == variant_ident,
            });

            if let Some(VariantDeclare::Data(
                _,
                TypeLiteralVariant::Type(TypeLiteral::UserDefined(ident)),
            )) = found_variant
            {
                if type_arg == *ident {
                    Ok(TypeVariant::Generic(
                        NalaType::Enum(enum_ident.to_owned(), variants),
                        vec![infer_type(data, scopes, current_scope)?],
                    ))
                } else {
                    Ok(TypeVariant::Generic(
                        NalaType::Enum(enum_ident.to_owned(), variants),
                        vec![TypeVariant::Type(NalaType::PrimitiveType(
                            PrimitiveType::Any,
                        ))],
                    ))
                }
            } else {
                Err(cannot_infer_value_error(raw_value))
            }
        } else {
            Err(cannot_infer_value_error(raw_value))
        }
    } else {
        Ok(TypeVariant::Type(NalaType::Enum(
            enum_ident.to_owned(),
            variants,
        )))
    }
}

fn cannot_infer_value_error(value: &Value) -> RuntimeError {
    RuntimeError::new(&format!("Cannot infer type of value `{value}`."))
}

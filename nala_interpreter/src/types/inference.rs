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

// TODO: There are most likely cases where infer_type is called on the same value multiple times, which
// is wasteful. Maybe it would be a good idea to cache the results of infer_type in the value itself?
// probably the best way to do that would be to not return a TypeVariant anymore, but instead return
// a new version of the type which has the type variant already filled in. That way, it's less likely
// that we'll miss a case where we don't update the value with the cached inferred type.
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
        // TODO: Inferring the type based on the first element isn't the right idea. Instead, we
        // should infer using the most informative element. For example, if the first element is
        // Option::None, we will infer Array<Option<T>> here, but the second element might have been
        // Option::Some(1), in which case we might have been able to infer Array<Option<Number>> had
        // we kept looking.
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

    let (enum_variants, generic_type_param) =
        scopes.get_type(&enum_ident, current_scope)?.as_enum()?;

    let existing_variant = find_variant(&enum_variants, variant_ident)?;

    match existing_variant {
        EnumVariant::Data(_ident, data_type) => {
            let data = match data {
                Some(d) => d,
                None => todo!("Expected data but none was supplied error."),
            };

            let inferred_type = if fits_type(data, &data_type, scopes, current_scope)? {
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

                    let outer = NalaType::Enum(enum_ident.to_owned(), enum_variants);
                    let outer = outer.make_concrete(Some(generic_type_param.clone()), &inner_type);

                    let generic_type_param = if inner_type.find_generic_type_param().is_some() {
                        Some(generic_type_param.clone())
                    } else {
                        None
                    };

                    TypeVariant::Composite(CompositeType {
                        outer,
                        inner: vec![inner_type],
                        generic_type_param,
                    })
                } else {
                    TypeVariant::Type(NalaType::Enum(enum_ident.to_owned(), enum_variants))
                }
            } else {
                todo!()
            };

            Ok(inferred_type)
        }
        EnumVariant::Empty(_ident) => {
            if let Some(generic_type_param) = generic_type_param {
                Ok(TypeVariant::Composite(CompositeType {
                    outer: NalaType::Enum(enum_ident.to_owned(), enum_variants),
                    inner: vec![TypeVariant::generic(generic_type_param.clone())],
                    generic_type_param: Some(generic_type_param),
                }))
            } else {
                Ok(TypeVariant::Type(NalaType::Enum(
                    enum_ident.to_owned(),
                    enum_variants,
                )))
            }
        }
    }
}

use std::sync::Arc;

use crate::{
    ast::{
        terms::{EnumVariantValue, StoredFunc, Value},
        types::{
            primitive_type::PrimitiveType, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant, variant_declare::VariantDeclare, TypeArgs,
        },
    },
    errors::RuntimeError,
    scopes::Scopes,
};

use super::{struct_field::StructField, type_variant::TypeVariant, NalaType};

pub fn infer_type(
    value: &Value,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<TypeVariant, RuntimeError> {
    let result = match value {
        Value::Array(items) => {
            let items = Arc::clone(&items);
            let items = items.lock().unwrap();
            let elem_type = if items.len() > 0 {
                infer_type(items.first().unwrap(), scopes, current_scope)?
            } else {
                todo!("Handle the case where trying to get the type of an empty array.")
            };

            TypeVariant::Generic(
                NalaType::PrimitiveType(PrimitiveType::Array),
                vec![elem_type],
            )
        }
        Value::Bool(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Bool)),
        Value::Break(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Break)),
        Value::Func(StoredFunc {
            params,
            return_type,
            ..
        }) => {
            let mut param_types: Vec<TypeVariant> = params
                .into_iter()
                .map(|p| {
                    TypeVariant::from_literal(p.clone().param_type, scopes, current_scope)
                        // TODO: Remove this unwrap().
                        .unwrap()
                }) // TODO: Why do we need this clone?
                .collect();

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
                    field_type: infer_type(v, scopes, current_scope).unwrap(),
                })
                .collect();

            TypeVariant::Type(NalaType::Struct(fields))
        }
        Value::String(_) => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::String)),
        Value::Variant(EnumVariantValue {
            enum_ident,
            data,
            variant_ident,
        }) => {
            let (variants, type_arg) = scopes
                .get_type(enum_ident, current_scope)?
                .as_enum()
                .unwrap();

            // TODO: Absolute mess, redo all this and remove use of Any.
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
                            TypeVariant::Generic(
                                NalaType::Enum(enum_ident.to_owned(), variants),
                                vec![infer_type(data, scopes, current_scope)?],
                            )
                        } else {
                            TypeVariant::Generic(
                                NalaType::Enum(enum_ident.to_owned(), variants),
                                vec![TypeVariant::Type(NalaType::PrimitiveType(
                                    PrimitiveType::Any,
                                ))],
                            )
                        }
                    } else {
                        TypeVariant::Generic(
                            NalaType::Enum(enum_ident.to_owned(), variants),
                            vec![TypeVariant::Type(NalaType::PrimitiveType(
                                PrimitiveType::Any,
                            ))],
                        )
                    }
                } else {
                    TypeVariant::Generic(
                        NalaType::Enum(enum_ident.to_owned(), variants),
                        vec![TypeVariant::Type(NalaType::PrimitiveType(
                            PrimitiveType::Any,
                        ))],
                    )
                }
            } else {
                TypeVariant::Type(NalaType::Enum(enum_ident.to_owned(), variants))
            }
        }
        Value::Void => TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Void)),
    };

    Ok(result)
}

pub fn fits_type(
    value: &Value,
    type_variant: &TypeVariant,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<bool, RuntimeError> {
    match type_variant {
        TypeVariant::Generic(outer, inner) => match outer {
            NalaType::PrimitiveType(PrimitiveType::Any) => todo!(),
            NalaType::PrimitiveType(PrimitiveType::Array) => todo!(),
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

fn fits_enum(
    enum_ident: &str,
    variants: &Vec<VariantDeclare>,
    value: &Value,
) -> Result<bool, RuntimeError> {
    let result = if let Value::Variant(value) = value {
        enum_ident == value.enum_ident && find_variant(&value.variant_ident, variants).is_some()
    } else {
        false
    };

    Ok(result)
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

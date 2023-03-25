use crate::{
    ast::types::enum_variant::EnumVariantOrAddend,
    errors::RuntimeError,
    io_context::IoContext,
    resolved::{
        enum_variants::EnumVariant,
        value::{EnumVariantValue, Value},
    },
    scopes::Scopes,
    types::{fit::fits_type, inference::infer_type},
};

use super::{basic::eval_expr, operations::eval_addend};

pub fn eval_enum_variant(
    variant: &EnumVariantOrAddend,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match variant {
        EnumVariantOrAddend::Addend(addend) => eval_addend(addend, scopes, current_scope, ctx),
        EnumVariantOrAddend::EnumVariant(enum_ident, variant_ident, data) => {
            let enum_type = scopes.get_type(enum_ident, current_scope)?.as_enum()?;

            let existing_variant = find_variant(&enum_type.variants, variant_ident)?;

            let data = if let Some(data) = data {
                let data = eval_expr(data, scopes, current_scope, ctx)?;
                let data_type = infer_type(&data, scopes, current_scope)?;

                let expected_data_type = if let EnumVariant::Data(_, expected_data_type) =
                    &existing_variant
                {
                    expected_data_type
                } else {
                    Err(RuntimeError::new(&format!(
                        "Passed data `{data:?}` of type type `{data_type}` when no data was expected.",
                    )))?
                };

                if !fits_type(&data, &expected_data_type, scopes, current_scope)? {
                    return Err(RuntimeError::new(&format!(
                        "Created variant with wrong data type. Expected `{expected_data_type}` but got `{0}`",
                        infer_type(&data, scopes, current_scope)?,
                    )));
                }

                Some(Box::new(data))
            } else {
                None
            };

            Ok(Value::Variant(EnumVariantValue {
                enum_ident: enum_ident.to_owned(),
                variant_ident: variant_ident.to_owned(),
                data,
            }))
        }
    }
}

fn compare_variant(variant: &EnumVariant, name: &str) -> bool {
    match variant {
        EnumVariant::Empty(variant) => variant == name,
        EnumVariant::Data(variant, _) => variant == name,
    }
}

pub fn find_variant(
    variants: &Vec<EnumVariant>,
    needle: &str,
) -> Result<EnumVariant, RuntimeError> {
    let result = variants.iter().find(|v| compare_variant(v, needle));
    match result {
        Some(variant) => Ok(variant.clone()),
        None => Err(RuntimeError::new(&format!(
            "Could not find variant `{needle}`.",
        ))),
    }
}

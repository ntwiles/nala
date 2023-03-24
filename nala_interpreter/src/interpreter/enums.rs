use crate::{
    ast::types::{enum_variant::EnumVariantOrAddend, variant_declare::VariantDeclare},
    errors::RuntimeError,
    io_context::IoContext,
    resolved::value::{EnumVariantValue, Value},
    scopes::Scopes,
    types::{fit::fits_type, inference::infer_type, type_variant::TypeVariant},
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

                let expected_data_type = if let VariantDeclare::Data(_, expected_data_type) =
                    &existing_variant
                {
                    TypeVariant::from_literal(
                        expected_data_type.clone(),
                        scopes,
                        enum_type.closure_scope,
                    )?
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

fn compare_variant(variant: &VariantDeclare, name: &str) -> bool {
    match variant {
        VariantDeclare::Empty(variant) => variant == name,
        VariantDeclare::Data(variant, _) => variant == name,
    }
}

fn find_variant(
    variants: &Vec<VariantDeclare>,
    needle: &str,
) -> Result<VariantDeclare, RuntimeError> {
    let result = variants.iter().find(|v| compare_variant(v, needle));
    match result {
        Some(variant) => Ok(variant.clone()),
        None => Err(RuntimeError::new(&format!(
            "Could not find variant `{needle}`.",
        ))),
    }
}

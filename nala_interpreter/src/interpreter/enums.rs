use crate::{
    ast::{
        terms::*,
        types::{
            enum_variant::EnumVariantOrAddend, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant, TypeArgs,
        },
        *,
    },
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
    types::type_variant::TypeVariant,
};

use super::{basic::eval_expr, operations::eval_addend};

pub fn eval_enum_variant(
    variant: &EnumVariantOrAddend,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match variant {
        EnumVariantOrAddend::Addend(addend) => {
            eval_addend(addend, scopes, current_scope, enclosing_scope, ctx)
        }
        EnumVariantOrAddend::EnumVariant(enum_ident, variant_ident, data) => {
            let (existing_variants, existing_type_args) =
                scopes.get_type(enum_ident, current_scope)?.as_enum()?;

            let existing_variant = find_variant(&existing_variants, variant_ident)?;

            let data = if let Some(data) = data {
                let data = eval_expr(data, scopes, current_scope, None, ctx)?; // TODO: Should we be passing None here?
                let data_type = data.get_type(scopes, current_scope)?;

                // TODO: This is a mess, clean this up.
                let expected_data_type = if let VariantDeclare::Data(_, data) = existing_variant {
                    if let Some(TypeArgs::Generic(type_args)) = existing_type_args {
                        if let TypeLiteralVariant::Type(TypeLiteral::UserDefined(data)) = data {
                            if type_args == data {
                                Some(data_type.clone())
                            } else {
                                todo!()
                            }
                        } else {
                            todo!()
                        }
                    } else {
                        Some(TypeVariant::from_literal(data, scopes, current_scope)?)
                    }
                } else {
                    None
                };

                // TODO: Merge this with the above.
                let expected_data_type = match expected_data_type {
                    Some(expected_data_type) => expected_data_type,
                    None => {
                        return Err(RuntimeError::new(&format!(
                            "Passed data type {data_type} when none was expected.",
                        )))
                    }
                };

                if !(data
                    .get_type(scopes, current_scope)?
                    .is_assignable_to(&expected_data_type))
                {
                    return Err(RuntimeError::new(&format!(
                            "Created variant with wrong data type. Expected `{expected_data_type}` but got `{0}`",
                            data.get_type(scopes, current_scope)?,
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
        None => todo!(),
    }
}

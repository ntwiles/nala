use crate::{
    ast::{
        terms::*,
        types::{
            enum_variant::EnumVariantOrAddend, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant, variant_declare::VariantDeclare, TypeArgs,
        },
    },
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
    types::{inference::infer_type, type_variant::TypeVariant},
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
            let (existing_variants, enum_type_args) =
                scopes.get_type(enum_ident, current_scope)?.as_enum()?;

            let existing_variant = find_variant(&existing_variants, variant_ident)?;

            let data = if let Some(data) = data {
                let data = eval_expr(data, scopes, current_scope, None, ctx)?; // TODO: Should we be passing None here?
                let data_type = infer_type(&data, scopes, current_scope)?;

                let expected_data_type = if let VariantDeclare::Data(_, data) = existing_variant {
                    if let Some(TypeArgs::Generic(enum_type_arg)) = enum_type_args {
                        match data {
                            // This is the case where the expected data type is generic.
                            TypeLiteralVariant::Type(TypeLiteral::UserDefined(t)) => {
                                if enum_type_arg == t {
                                    data_type.clone()
                                } else {
                                    todo!()
                                }
                            }
                            _ => TypeVariant::from_literal(data, scopes, current_scope)?,
                        }
                    } else {
                        TypeVariant::from_literal(data, scopes, current_scope)?
                    }
                } else {
                    Err(RuntimeError::new(&format!(
                        "Passed data type {data_type} when none was expected.",
                    )))?
                };

                if !(infer_type(&data, scopes, current_scope)?
                    .is_assignable_to(&expected_data_type))
                {
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

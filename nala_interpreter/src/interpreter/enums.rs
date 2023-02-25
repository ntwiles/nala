use crate::{
    ast::{terms::*, types::*, *},
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
};

use super::{basic::*, operations::*};

pub fn eval_variant(
    variant: &VariantValue,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let (enum_name, variant, data) = match variant {
        VariantValue::Addend(addend) => return eval_addend(addend, scopes, current_scope, ctx),
        VariantValue::VariantValue(enum_name, variant) => (enum_name, variant, None),
        VariantValue::VariantValueWithData(enum_name, variant, data) => {
            (enum_name, variant, Some(data))
        }
    };

    let value = scopes.get_value(enum_name, current_scope, ctx)?;

    if let Value::Type(TypeVariant::Enum(_enum_name, variants)) = value {
        let existing_variant = find_variant(&variants, variant)?;

        let expected_data_type = if let VariantDeclare::Data(_, data) = existing_variant {
            Some(data)
        } else {
            None
        };

        let data = if let Some(data) = data {
            let data = eval_expr(data, scopes, current_scope, ctx)?;

            let expected_data_type = if let Some(data_type) = expected_data_type {
                data_type
            } else {
                return Err(RuntimeError {
                    message: format!(
                        "Passed data type {0} when none was expected!",
                        data.get_type()
                    ),
                });
            };

            if !(data.get_type().is_assignable_to(&expected_data_type)) {
                return Err(RuntimeError {
                    message: format!(
                        "Created variant with wrong data type! Expected `{0}` but got `{1}`",
                        expected_data_type,
                        data.get_type()
                    ),
                });
            }

            Some(Box::new(data))
        } else {
            None
        };

        Ok(Value::Variant(
            enum_name.to_owned(),
            variant.to_owned(),
            data,
        ))
    } else {
        panic!("{} is not an Enum value.", enum_name);
    }
}

fn compare_variant(variant: &VariantDeclare, name: &String) -> bool {
    match variant {
        VariantDeclare::Empty(variant) => variant == name,
        VariantDeclare::Data(variant, _) => variant == name,
    }
}

fn find_variant(
    variants: &Vec<VariantDeclare>,
    needle: &String,
) -> Result<VariantDeclare, RuntimeError> {
    let result = variants.iter().find(|v| compare_variant(v, needle));
    match result {
        Some(variant) => Ok(variant.clone()),
        None => todo!(),
    }
}

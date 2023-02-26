use crate::{
    ast::{terms::*, *},
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
};

pub fn eval_enum_variant(
    enum_ident: &str,
    variant_ident: &str,
    scopes: &mut Scopes,
    current_scope: usize,
    _enclosing_scope: Option<usize>,
    _ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let the_enum = scopes.get_type(enum_ident, current_scope)?.as_enum()?;

    let _existing_variant = find_variant(&the_enum, variant_ident)?;

    // let expected_data_type = if let VariantDeclare::Data(_, data) = existing_variant {
    //     Some(data)
    // } else {
    //     None
    // };

    // TODO: Support data in variants.
    // let data = if let Some(data) = data {
    //     let data = eval_expr(data, scopes, current_scope, None, ctx)?;

    //     let expected_data_type = if let Some(data_type) = expected_data_type {
    //         data_type
    //     } else {
    //         return Err(RuntimeError::new(&format!(
    //             "Passed data type {0} when none was expected.",
    //             data.get_type()
    //         )));
    //     };

    //     if !(data.get_type().is_assignable_to(&expected_data_type)) {
    //         return Err(RuntimeError::new(format!(
    //                 "Created variant with wrong data type. Expected `{expected_data_type}` but got `{0}`",
    //                 data.get_type()
    //             )));
    //     }

    //     Some(Box::new(data))
    // } else {
    //     None
    // };

    Ok(Value::Variant(
        enum_ident.to_owned(),
        variant_ident.to_owned(),
        None,
    ))
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

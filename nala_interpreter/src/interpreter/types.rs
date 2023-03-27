use crate::{
    ast::types::{variant_declare::VariantDeclare, StructLiteralField},
    errors::RuntimeError,
    resolved::{enum_variants::EnumVariant, struct_field::StructField, value::Value},
    scopes::Scopes,
    types::{composite_type::CompositeType, nala_type::NalaType, type_variant::TypeVariant},
    utils::accept_results,
};

pub fn eval_struct(
    ident: &str,
    type_param: Option<String>,
    fields: Vec<StructLiteralField>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let closure_scope = scopes.new_scope(Some(current_scope));

    let binding = if let Some(type_param) = &type_param {
        scopes.add_type_binding(
            closure_scope,
            &type_param,
            TypeVariant::generic(type_param.clone()),
        )?;

        TypeVariant::Composite(CompositeType {
            outer: NalaType::Struct(fields_from_literals(fields, scopes, closure_scope)?),
            inner: vec![TypeVariant::generic(type_param.clone())],
            generic_type_param: Some(type_param.clone()),
        })
    } else {
        TypeVariant::Type(NalaType::Struct(fields_from_literals(
            fields,
            scopes,
            closure_scope,
        )?))
    };

    scopes.add_type_binding(current_scope, &ident, binding)?;

    Ok(Value::Void)
}

fn fields_from_literals(
    fields: Vec<StructLiteralField>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Vec<StructField>, RuntimeError> {
    accept_results(
        fields
            .into_iter()
            .map(|f| StructField::from_literal(f, scopes, current_scope))
            .collect(),
    )
}

pub fn eval_enum(
    ident: &str,
    type_param: Option<String>,
    variants: Vec<VariantDeclare>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let closure_scope = scopes.new_scope(Some(current_scope));

    let binding = if let Some(type_param) = &type_param {
        scopes.add_type_binding(
            closure_scope,
            &type_param,
            TypeVariant::generic(type_param.clone()),
        )?;

        TypeVariant::Composite(CompositeType {
            outer: NalaType::Enum(
                ident.to_string(),
                variants_from_literals(variants, scopes, closure_scope)?,
            ),
            inner: vec![TypeVariant::generic(type_param.clone())],
            generic_type_param: Some(type_param.clone()),
        })
    } else {
        let variants = variants_from_literals(variants, scopes, closure_scope)?;

        TypeVariant::Type(NalaType::Enum(ident.to_string(), variants))
    };

    scopes.add_type_binding(current_scope, &ident, binding)?;

    Ok(Value::Void)
}

fn variants_from_literals(
    variants: Vec<VariantDeclare>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Vec<EnumVariant>, RuntimeError> {
    accept_results(
        variants
            .into_iter()
            .map(|f| EnumVariant::from_literal(f, scopes, current_scope))
            .collect(),
    )
}

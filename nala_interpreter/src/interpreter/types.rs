use crate::{
    ast::types::{variant_declare::VariantDeclare, StructLiteralField},
    errors::RuntimeError,
    resolved::{enum_variants::EnumVariant, struct_field::StructField, value::Value},
    scopes::Scopes,
    types::{composite_type::CompositeType, nala_type::NalaType, type_variant::TypeVariant},
    utils::accept_results,
};

// TODO: This has duplicated code for generating fields because this needs to happen after binding
// generic type params. Fix this.
pub fn eval_struct(
    ident: &str,
    type_param: Option<String>,
    fields: Vec<StructLiteralField>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let closure_scope = scopes.new_scope(Some(current_scope));

    if let Some(type_param) = &type_param {
        scopes.add_type_binding(
            closure_scope,
            &type_param,
            TypeVariant::generic(type_param.clone()),
        )?;

        let fields = accept_results(
            fields
                .into_iter()
                .map(|f| StructField::from_literal(f, scopes, closure_scope))
                .collect(),
        )?;

        scopes.add_type_binding(
            current_scope,
            &ident,
            TypeVariant::Composite(CompositeType {
                outer: NalaType::Struct(fields),
                inner: vec![TypeVariant::generic(type_param.clone())],
                generic_type_param: Some(type_param.clone()),
            }),
        )?;
    } else {
        let fields = accept_results(
            fields
                .into_iter()
                .map(|f| StructField::from_literal(f, scopes, closure_scope))
                .collect(),
        )?;

        scopes.add_type_binding(
            current_scope,
            &ident,
            TypeVariant::Type(NalaType::Struct(fields)),
        )?;
    };

    Ok(Value::Void)
}

// TODO: This has duplicated code for generating fields because this needs to happen after binding
// generic type params. Fix this.
pub fn eval_enum(
    ident: &str,
    type_param: Option<String>,
    variants: Vec<VariantDeclare>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let closure_scope = scopes.new_scope(Some(current_scope));

    if let Some(type_param) = &type_param {
        scopes.add_type_binding(
            closure_scope,
            &type_param,
            TypeVariant::generic(type_param.clone()),
        )?;

        let variants = accept_results(
            variants
                .into_iter()
                .map(|f| EnumVariant::from_literal(f, scopes, closure_scope))
                .collect(),
        )?;

        scopes.add_type_binding(
            current_scope,
            &ident,
            TypeVariant::Composite(CompositeType {
                outer: NalaType::Enum(ident.to_string(), variants),
                inner: vec![TypeVariant::generic(type_param.clone())],
                generic_type_param: Some(type_param.clone()),
            }),
        )?;
    } else {
        let variants = accept_results(
            variants
                .into_iter()
                .map(|f| EnumVariant::from_literal(f, scopes, closure_scope))
                .collect(),
        )?;

        scopes.add_type_binding(
            current_scope,
            &ident,
            TypeVariant::Type(NalaType::Enum(ident.to_string(), variants)),
        )?;
    };

    Ok(Value::Void)
}

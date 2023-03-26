use crate::{
    ast::types::{variant_declare::VariantDeclare, StructLiteralField},
    errors::RuntimeError,
    resolved::{
        enum_variants::EnumVariant, from_literal::FromLiteral, struct_field::StructField,
        value::Value,
    },
    scopes::{enum_binding::EnumBinding, type_binding::TypeBinding, Scopes},
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

    if let Some(type_param) = &type_param {
        scopes.add_type_binding(
            closure_scope,
            &type_param,
            TypeBinding::Generic(type_param.clone()),
        )?;
    }

    let fields = accept_results(
        fields
            .into_iter()
            .map(|f| StructField::from_literal(f, scopes, closure_scope))
            .collect(),
    )?;

    scopes
        .add_type_binding(
            current_scope,
            &ident,
            TypeBinding::Struct(fields, type_param),
        )
        .map(|_| Value::Void)
}

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
            TypeBinding::Generic(type_param.clone()),
        )?;
    }

    let variants = accept_results(
        variants
            .into_iter()
            .map(|f| EnumVariant::from_literal(f, scopes, closure_scope))
            .collect(),
    )?;

    scopes
        .add_type_binding(
            current_scope,
            &ident,
            TypeBinding::Enum(EnumBinding { variants }, type_param),
        )
        .map(|_| Value::Void)
}

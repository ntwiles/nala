use crate::{
    ast::types::{variant_declare::VariantDeclare, StructLiteralField},
    errors::RuntimeError,
    resolved::{struct_field::StructField, value::Value},
    scopes::{
        enum_binding::EnumBinding, struct_binding::StructBinding, type_binding::TypeBinding, Scopes,
    },
    utils::accept_results,
};

pub fn eval_struct(
    ident: &str,
    type_params: Option<String>,
    fields: Vec<StructLiteralField>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let closure_scope = scopes.new_scope(Some(current_scope));

    if let Some(type_param) = &type_params {
        scopes.add_type_binding(
            &type_param,
            closure_scope,
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
            &ident,
            current_scope,
            TypeBinding::Struct(StructBinding {
                generic_ident: type_params,
                closure_scope,
                fields,
            }),
        )
        .map(|_| Value::Void)
}

pub fn eval_enum(
    ident: &str,
    type_params: Option<String>,
    variants: Vec<VariantDeclare>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let closure_scope = scopes.new_scope(Some(current_scope));

    if let Some(type_param) = &type_params {
        scopes.add_type_binding(
            &type_param,
            closure_scope,
            TypeBinding::Generic(type_param.clone()),
        )?;
    }

    scopes
        .add_type_binding(
            &ident,
            current_scope,
            TypeBinding::Enum(EnumBinding {
                variants,
                closure_scope,
                generic_ident: type_params,
            }),
        )
        .map(|_| Value::Void)
}

use crate::{
    ast::{
        terms::Value,
        types::{StructLiteralField, TypeArgs},
        VariantDeclare,
    },
    errors::RuntimeError,
    scopes::{type_binding::TypeBinding, Scopes},
    types::struct_field::StructField,
};

pub fn eval_struct(
    ident: &str,
    type_args: &Option<TypeArgs>,
    fields: Vec<StructLiteralField>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let binding = TypeBinding::Struct(
        fields
            .into_iter()
            // TODO: Remove this .unwrap().
            .map(|f| StructField::from_literal(f, scopes, current_scope).unwrap())
            .collect(),
    );

    scopes
        .add_type_binding(&ident, current_scope, binding)
        .map(|_| Value::Void)
}

pub fn eval_enum(
    ident: &str,
    type_args: &Option<TypeArgs>,
    variants: Vec<VariantDeclare>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let binding = TypeBinding::Enum(variants);

    println!("type_args: {:?}", type_args);

    scopes
        .add_type_binding(&ident, current_scope, binding)
        .map(|_| Value::Void)
}

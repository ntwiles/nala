use crate::{
    ast::{terms::*, types::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::operations::*;

pub fn interpret_enum(
    ident: &String,
    variants: &VariantsDeclare,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Result<Term, NalaRuntimeError> {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        let enum_type = TypeVariant::Enum(ident.to_owned(), Box::new(variants.clone()));
        let enum_term = Term::Type(enum_type);
        scopes.add_binding(&ident, current_scope, enum_term, false);
    }

    Ok(Term::Void)
}

pub fn evaluate_variant(
    variant: &VariantValue,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    match variant {
        VariantValue::VariantValue(enum_name, variant) => {
            let term = scopes.get_value(enum_name, current_scope, context)?;

            if let Term::Type(TypeVariant::Enum(_, variants)) = term {
                if variant_exists(&*variants, variant) {
                    Ok(Term::Variant(format!(
                        "{0}::{1}",
                        enum_name,
                        variant.to_owned()
                    )))
                } else {
                    panic!(
                        "Enum variant {0} does not exist on Enum {1}",
                        variant, enum_name
                    )
                }
            } else {
                panic!("{} is not an Enum value.", enum_name);
            }
        }
        VariantValue::Addend(addend) => evaluate_addend(addend, scopes, current_scope, context),
    }
}

fn compare_variant(variant: &VariantDeclare, name: &String) -> bool {
    match variant {
        VariantDeclare::Empty(variant_name) => variant_name == name,
    }
}

fn variant_exists(variants: &VariantsDeclare, needle: &String) -> bool {
    match variants {
        VariantsDeclare::Variants(variants, variant) => {
            compare_variant(variant, needle) || variant_exists(variants, needle)
        }
        VariantsDeclare::Variant(variant) => compare_variant(variant, needle),
    }
}

use crate::{
    ast::{terms::*, types::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::operations::*;

pub fn interpret_enum(
    ident: &String,
    kinds: &KindsDeclare,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Result<Term, NalaRuntimeError> {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        let enum_type = TypeVariant::Enum(ident.to_owned(), Box::new(kinds.clone()));
        let enum_term = Term::Type(enum_type);
        scopes.add_binding(&ident, current_scope, enum_term, false);
    }

    Ok(Term::Void)
}

pub fn evaluate_kind(
    kind: &KindValue,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    match kind {
        KindValue::KindValue(enum_name, kind) => {
            let term = scopes.get_value(enum_name, current_scope, context);

            if let Term::Type(TypeVariant::Enum(_, kinds)) = term {
                if kind_exists(&*kinds, kind) {
                    Ok(Term::Kind(format!("{0}::{1}", enum_name, kind.to_owned())))
                } else {
                    panic!("Kind {0} does not exist on Enum {1}", kind, enum_name)
                }
            } else {
                panic!("{} is not an Enum value.", enum_name);
            }
        }
        KindValue::Addend(addend) => evaluate_addend(addend, scopes, current_scope, context),
    }
}

fn compare_kind(kind: &KindDeclare, name: &String) -> bool {
    match kind {
        KindDeclare::Empty(kind_name) => kind_name == name,
    }
}

fn kind_exists(kinds: &KindsDeclare, needle: &String) -> bool {
    match kinds {
        KindsDeclare::Kinds(kinds, kind) => {
            compare_kind(kind, needle) || kind_exists(kinds, needle)
        }
        KindsDeclare::Kind(kind) => compare_kind(kind, needle),
    }
}

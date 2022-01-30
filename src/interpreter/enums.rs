use crate::{
    ast::*,
    scope::{ScopeId, Scopes},
};

pub fn interpret_enum(
    ident: &String,
    kinds: &Kinds,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Term {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        let enum_term = Term::Enum(ident.to_owned(), Box::new(kinds.clone()));
        scopes.add_binding(&ident, current_scope, enum_term, false);
    }

    Term::Void
}

pub fn evaluate_kind(
    enum_name: &String,
    kind: &String,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Term {
    let term = scopes.get_value(enum_name, current_scope);

    if let Term::Enum(_, kinds) = term {
        if kind_exists(&*kinds, kind) {
            Term::Kind(format!("{0}::{1}", enum_name, kind.to_owned()))
        } else {
            panic!("Kind {0} does not exist on Enum {1}", kind, enum_name)
        }
    } else {
        panic!("{} is not an Enum value.", enum_name);
    }
}

fn compare_kind(kind: &Kind, name: &String) -> bool {
    match kind {
        Kind::Empty(kind_name) => kind_name == name,
    }
}

fn kind_exists(kinds: &Kinds, needle: &String) -> bool {
    match kinds {
        Kinds::Kinds(kinds, kind) => compare_kind(kind, needle) || kind_exists(kinds, needle),
        Kinds::Kind(kind) => compare_kind(kind, needle),
    }
}

use std::collections::HashMap;

use crate::ast;

pub struct Scope {
    parent: Option<ScopeId>,
    bindings: HashMap<String, (ast::Term, bool)>,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>) -> Scope {
        Scope {
            parent,
            bindings: HashMap::<String, (ast::Term, bool)>::new(),
        }
    }

    pub fn add_binding(self: &mut Self, ident: &str, value: ast::Term, is_mutable: bool) {
        self.bindings.insert(ident.to_owned(), (value, is_mutable));
    }

    pub fn get_value(self: &Self, ident: &str) -> Option<ast::Term> {
        if let Some((value, _)) = self.bindings.get(ident) {
            Some(value.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct ScopeId {
    index: usize,
}

pub struct Scopes {
    scopes: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Scopes {
        Scopes { scopes: vec![] }
    }

    pub fn new_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let next_index = self.scopes.len();
        self.scopes.push(Scope::new(parent));
        ScopeId { index: next_index }
    }

    fn get_maybe_value(self: &Self, ident: &str, current_scope: ScopeId) -> Option<ast::Term> {
        let scope = self.scopes.get(current_scope.index).unwrap();

        match scope.get_value(&ident) {
            Some(value) => Some(value),
            None => match scope.parent {
                Some(parent_scope) => Some(self.get_value(ident, parent_scope)),
                None => None,
            },
        }
    }

    pub fn get_value(self: &Self, ident: &str, current_scope: ScopeId) -> ast::Term {
        match self.get_maybe_value(ident, current_scope) {
            Some(value) => value,
            None => panic!("Identifier '{}' was not found in this scope.", ident),
        }
    }

    pub fn add_binding(
        self: &mut Self,
        ident: &str,
        current_scope: ScopeId,
        value: ast::Term,
        is_mutable: bool,
    ) {
        let scope = self.scopes.get_mut(current_scope.index).unwrap();
        scope.add_binding(ident, value, is_mutable);
    }

    pub fn binding_exists_local(self: &Self, ident: &str, current_scope: ScopeId) -> bool {
        self.scopes
            .get(current_scope.index)
            .unwrap()
            .get_value(&ident)
            .is_some()
    }
}

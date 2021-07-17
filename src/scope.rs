use std::collections::HashMap;

use crate::ast;

pub struct Scope {
    parent: Option<ScopeId>,
    bindings: HashMap<String, ast::Term>,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>) -> Scope {
        Scope {
            parent,
            bindings: HashMap::<String, ast::Term>::new(),
        }
    }

    pub fn add_binding(self: &mut Self, ident: &str, value: ast::Term) {
        self.bindings.insert(ident.to_owned(), value);
    }

    pub fn get_value(self: &Self, ident: &str) -> Option<&ast::Term> {
        self.bindings.get(ident)
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

    // TODO: Better error reporting than unwrap offers.
    pub fn get_value(self: &Self, ident: &str, current_scope: ScopeId) -> ast::Term {
        self.scopes
            .get(current_scope.index)
            .unwrap()
            .get_value(&ident)
            .unwrap()
            .clone()
    }

    pub fn add_binding(self: &mut Self, ident: &str, current_scope: ScopeId, value: ast::Term) {
        let scope = self.scopes.get_mut(current_scope.index).unwrap();
        scope.add_binding(ident, value);
    }

    pub fn binding_exists_local(self: &Self, ident: &str, current_scope: ScopeId) -> bool {
        self.scopes
            .get(current_scope.index)
            .unwrap()
            .get_value(&ident)
            .is_some()
    }
}

use std::collections::HashMap;

use crate::ast;

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeId>,
    bindings: HashMap<String, (ast::Term, String, bool)>,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>) -> Scope {
        Scope {
            parent,
            bindings: HashMap::<String, (ast::Term, String, bool)>::new(),
        }
    }

    pub fn add_binding(self: &mut Self, ident: &str, value: ast::Term, is_mutable: bool) {
        let type_name = match value {
            ast::Term::Array(_) => "Array",
            ast::Term::Bool(_) => "Bool",
            ast::Term::Break(_) => "<Break>",
            ast::Term::Func(_, _) => "Func",
            ast::Term::Num(_) => "Num",
            ast::Term::String(_) => "String",
            ast::Term::Symbol(_) => "Symbol",
            ast::Term::Void => "<Void>",
        };

        self.bindings.insert(
            ident.to_owned(),
            (value, String::from(type_name), is_mutable),
        );
    }

    pub fn get_binding(self: &Self, ident: &str) -> Option<(ast::Term, String, bool)> {
        if let Some(binding) = self.bindings.get(ident) {
            Some(binding.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScopeId {
    index: usize,
}

#[derive(Debug)]
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

        match scope.get_binding(&ident) {
            Some((value, _, _)) => Some(value),
            None => match scope.parent {
                Some(parent_scope) => Some(self.get_value(ident, parent_scope)),
                None => None,
            },
        }
    }

    pub fn get_value(self: &Self, ident: &str, starting_scope: ScopeId) -> ast::Term {
        match self.get_maybe_value(ident, starting_scope) {
            Some(value) => value,
            None => panic!("Identifier '{}' was not found in this scope.", ident),
        }
    }

    fn find_scope_with_binding(
        self: &mut Self,
        ident: &str,
        current_scope_id: ScopeId,
    ) -> Option<&mut Scope> {
        if self.binding_exists_local(ident, current_scope_id) {
            Some(self.scopes.get_mut(current_scope_id.index).unwrap())
        } else {
            let parent = self.scopes.get_mut(current_scope_id.index).unwrap().parent;

            if let Some(parent) = parent {
                self.find_scope_with_binding(ident, parent)
            } else {
                None
            }
        }
    }

    pub fn mutate_value(
        self: &mut Self,
        ident: &str,
        current_scope: ScopeId,
        new_value: ast::Term,
    ) {
        let scope = self.find_scope_with_binding(ident, current_scope);

        if let Some(scope) = scope {
            let (_, _, is_mutable) = scope.get_binding(ident).unwrap();
            if is_mutable {
                scope.add_binding(ident, new_value, true)
            } else {
                panic!("Cannot re-assign to immutable binding {}", &ident)
            }
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

    pub fn binding_exists(self: &Self, ident: &str, current_scope: ScopeId) -> bool {
        self.get_maybe_value(ident, current_scope).is_some()
    }

    pub fn binding_exists_local(self: &Self, ident: &str, current_scope: ScopeId) -> bool {
        self.scopes
            .get(current_scope.index)
            .unwrap()
            .get_binding(&ident)
            .is_some()
    }
}

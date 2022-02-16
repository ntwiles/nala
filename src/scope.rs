use std::collections::HashMap;

use crate::{ast::terms::*, errors::*, io_context::IoContext};

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeId>,
    bindings: HashMap<String, (Term, String, bool)>,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>) -> Scope {
        Scope {
            parent,
            bindings: HashMap::<String, (Term, String, bool)>::new(),
        }
    }

    pub fn add_binding(self: &mut Self, ident: &str, value: Term, is_mutable: bool) {
        let type_name = value.get_type().to_string();

        self.bindings.insert(
            ident.to_owned(),
            (value, String::from(type_name), is_mutable),
        );
    }

    pub fn get_binding(self: &Self, ident: &str) -> Option<(Term, String, bool)> {
        if let Some(binding) = self.bindings.get(ident) {
            Some(binding.clone())
        } else {
            None
        }
    }
}

fn not_found_in_scope_error(ident: &str) -> NalaRuntimeError {
    NalaRuntimeError {
        message: format!("Identifier '{}' was not found in this scope.", ident),
    }
}

fn assign_immutable_binding_error(ident: &str) -> NalaRuntimeError {
    NalaRuntimeError {
        message: format!("Cannot re-assign to immutable binding `{}`.", ident),
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

    fn get_maybe_value(
        self: &Self,
        ident: &str,
        current_scope: ScopeId,
        context: &mut dyn IoContext,
    ) -> Option<Term> {
        let scope = self.scopes.get(current_scope.index).unwrap();

        match scope.get_binding(&ident) {
            Some((value, _, _)) => Some(value),
            None => match scope.parent {
                Some(parent_scope) => self.get_maybe_value(ident, parent_scope, context),
                None => None,
            },
        }
    }

    pub fn get_value(
        self: &Self,
        ident: &str,
        starting_scope: ScopeId,
        context: &mut dyn IoContext,
    ) -> Result<Term, NalaRuntimeError> {
        match self.get_maybe_value(ident, starting_scope, context) {
            Some(value) => Ok(value),
            None => Err(not_found_in_scope_error(ident)),
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
        new_value: Term,
    ) -> Result<Term, NalaRuntimeError> {
        let scope = self.find_scope_with_binding(ident, current_scope);

        if let Some(scope) = scope {
            let (_, _, is_mutable) = scope.get_binding(ident).unwrap();
            if is_mutable {
                scope.add_binding(ident, new_value, true)
            } else {
                return Err(assign_immutable_binding_error(ident));
            }
        } else {
            return Err(not_found_in_scope_error(ident));
        }

        Ok(Term::Void)
    }

    pub fn add_binding(
        self: &mut Self,
        ident: &str,
        current_scope: ScopeId,
        value: Term,
        is_mutable: bool,
    ) {
        let scope = self.scopes.get_mut(current_scope.index).unwrap();
        scope.add_binding(ident, value, is_mutable);
    }

    pub fn binding_exists(
        self: &Self,
        ident: &str,
        current_scope: ScopeId,
        context: &mut dyn IoContext,
    ) -> bool {
        self.get_maybe_value(ident, current_scope, context)
            .is_some()
    }

    pub fn binding_exists_local(self: &Self, ident: &str, current_scope: ScopeId) -> bool {
        self.scopes
            .get(current_scope.index)
            .unwrap()
            .get_binding(&ident)
            .is_some()
    }
}

use std::collections::HashMap;

use crate::{ast::terms::*, errors::*, types::struct_field::StructField};

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeId>,
    bindings: HashMap<String, (Value, bool)>,
    type_bindings: HashMap<String, Vec<StructField>>,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>) -> Scope {
        Scope {
            parent,
            bindings: HashMap::new(),
            type_bindings: HashMap::new(),
        }
    }

    pub fn add_struct_binding(self: &mut Self, ident: &str, fields: Vec<StructField>) {
        self.type_bindings.insert(ident.to_owned(), fields);
    }

    pub fn add_binding(self: &mut Self, ident: &str, value: Value, is_mutable: bool) {
        self.bindings.insert(ident.to_owned(), (value, is_mutable));
    }

    pub fn get_binding(self: &Self, ident: &str) -> Option<(Value, bool)> {
        if let Some(binding) = self.bindings.get(ident) {
            Some(binding.clone())
        } else {
            None
        }
    }

    pub fn get_struct_binding(self: &Self, ident: &str) -> Option<&Vec<StructField>> {
        if let Some(binding) = self.type_bindings.get(ident) {
            Some(binding)
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

// Why is this a struct and not just a usize?
#[derive(Debug, Clone, Copy)]
pub struct ScopeId {
    index: usize,
}

// TODO: Pull this out to its own file.
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

    fn get_maybe_value(self: &Self, ident: &str, current_scope: ScopeId) -> Option<Value> {
        let scope = self.scopes.get(current_scope.index).unwrap();

        match scope.get_binding(&ident) {
            Some((value, _)) => Some(value),
            None => match scope.parent {
                Some(parent_scope) => self.get_maybe_value(ident, parent_scope),
                None => None,
            },
        }
    }

    fn get_maybe_struct(
        self: &Self,
        ident: &str,
        current_scope: ScopeId,
    ) -> Option<&Vec<StructField>> {
        let scope = self.scopes.get(current_scope.index).unwrap();

        match scope.get_struct_binding(&ident) {
            Some(fields) => Some(fields),
            None => match scope.parent {
                Some(parent_scope) => self.get_maybe_struct(ident, parent_scope),
                None => None,
            },
        }
    }

    // TODO: There must be a way to avoid all these gross match statements.
    pub fn get_value(
        self: &Self,
        ident: &str,
        starting_scope: ScopeId,
        enclosing_scope: Option<ScopeId>,
    ) -> Result<Value, NalaRuntimeError> {
        if let Some(enclosing_scope) = enclosing_scope {
            match self.get_maybe_value(ident, enclosing_scope) {
                Some(value) => return Ok(value),
                None => match self.get_maybe_value(ident, starting_scope) {
                    Some(value) => Ok(value),
                    None => Err(not_found_in_scope_error(ident)),
                },
            }
        } else {
            match self.get_maybe_value(ident, starting_scope) {
                Some(value) => Ok(value),
                None => Err(not_found_in_scope_error(ident)),
            }
        }
    }

    pub fn get_struct(
        self: &Self,
        ident: &str,
        starting_scope: ScopeId,
    ) -> Result<Vec<StructField>, NalaRuntimeError> {
        match self.get_maybe_struct(ident, starting_scope) {
            Some(value) => Ok(value.clone()),
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
        new_value: Value,
    ) -> Result<Value, NalaRuntimeError> {
        let scope = self.find_scope_with_binding(ident, current_scope);

        if let Some(scope) = scope {
            let (_, is_mutable) = scope.get_binding(ident).unwrap();
            if is_mutable {
                scope.add_binding(ident, new_value, true)
            } else {
                return Err(assign_immutable_binding_error(ident));
            }
        } else {
            return Err(not_found_in_scope_error(ident));
        }

        Ok(Value::Void)
    }

    pub fn add_binding(
        self: &mut Self,
        ident: &str,
        current_scope: ScopeId,
        value: Value,
        is_mutable: bool,
    ) {
        let scope = self.scopes.get_mut(current_scope.index).unwrap();
        scope.add_binding(ident, value, is_mutable);
    }

    pub fn add_struct_binding(
        self: &mut Self,
        ident: &str,
        current_scope: ScopeId,
        fields: Vec<StructField>,
    ) -> Result<(), NalaRuntimeError> {
        if self
            .scopes
            .get(current_scope.index)
            .unwrap()
            .get_struct_binding(&ident)
            .is_some()
        {
            Err(NalaRuntimeError {
                message: format!(
                    "Binding for struct {} already exists in local scope.",
                    ident
                ),
            })
        } else {
            let scope = self.scopes.get_mut(current_scope.index).unwrap();

            scope.add_struct_binding(ident, fields);

            Ok(())
        }
    }

    fn binding_exists_enclosing(
        self: &Self,
        ident: &str,
        enclosing_scope: Option<ScopeId>,
    ) -> bool {
        if let Some(enclosing_scope) = enclosing_scope {
            self.get_maybe_value(ident, enclosing_scope).is_some()
        } else {
            false
        }
    }

    pub fn binding_exists(
        self: &Self,
        ident: &str,
        current_scope: ScopeId,
        enclosing_scope: Option<ScopeId>,
    ) -> bool {
        self.binding_exists_enclosing(ident, enclosing_scope)
            || self.get_maybe_value(ident, current_scope).is_some()
    }

    // TODO: Get rid of this and just have `add_binding` return a Result.
    pub fn binding_exists_local(self: &Self, ident: &str, current_scope: ScopeId) -> bool {
        self.scopes
            .get(current_scope.index)
            .unwrap()
            .get_binding(&ident)
            .is_some()
    }
}

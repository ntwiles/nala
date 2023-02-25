mod scope;

use crate::{ast::terms::*, errors::*, types::struct_field::StructField};

use self::scope::Scope;

#[derive(Debug)]
pub struct Scopes {
    scopes: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Scopes {
        Scopes { scopes: vec![] }
    }

    pub fn new_scope(&mut self, parent: Option<usize>) -> usize {
        let next_index = self.scopes.len();
        self.scopes.push(Scope::new(parent));
        next_index
    }

    fn get_maybe_value(self: &Self, ident: &str, current_scope: usize) -> Option<Value> {
        let scope = self.scopes.get(current_scope).unwrap();

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
        current_scope: usize,
    ) -> Option<&Vec<StructField>> {
        let scope = self.scopes.get(current_scope).unwrap();

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
        starting_scope: usize,
        enclosing_scope: Option<usize>,
    ) -> Result<Value, RuntimeError> {
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
        starting_scope: usize,
    ) -> Result<Vec<StructField>, RuntimeError> {
        match self.get_maybe_struct(ident, starting_scope) {
            Some(value) => Ok(value.clone()),
            None => Err(not_found_in_scope_error(ident)),
        }
    }

    fn find_scope_with_binding(
        self: &mut Self,
        ident: &str,
        current_scope_id: usize,
    ) -> Option<&mut Scope> {
        if self.binding_exists_local(ident, current_scope_id) {
            Some(self.scopes.get_mut(current_scope_id).unwrap())
        } else {
            let parent = self.scopes.get_mut(current_scope_id).unwrap().parent;

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
        current_scope: usize,
        new_value: Value,
    ) -> Result<Value, RuntimeError> {
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
        current_scope: usize,
        value: Value,
        is_mutable: bool,
    ) -> Result<Value, RuntimeError> {
        if self.binding_exists_local(ident, current_scope) {
            Err(RuntimeError::new(&format!(
                "Binding for {} already exists in local scope.",
                ident
            )))
        } else {
            let scope = self.scopes.get_mut(current_scope).unwrap();
            scope.add_binding(ident, value, is_mutable);
            Ok(Value::Void)
        }
    }

    pub fn add_struct_binding(
        self: &mut Self,
        ident: &str,
        current_scope: usize,
        fields: Vec<StructField>,
    ) -> Result<Value, RuntimeError> {
        if self.struct_binding_exists_local(ident, current_scope) {
            Err(RuntimeError::new(&format!(
                "Binding for struct {} already exists in local scope.",
                ident
            )))
        } else {
            let scope = self.scopes.get_mut(current_scope).unwrap();
            scope.add_struct_binding(ident, fields);
            Ok(Value::Void)
        }
    }

    fn binding_exists_enclosing(self: &Self, ident: &str, enclosing_scope: Option<usize>) -> bool {
        if let Some(enclosing_scope) = enclosing_scope {
            self.get_maybe_value(ident, enclosing_scope).is_some()
        } else {
            false
        }
    }

    pub fn binding_exists(
        self: &Self,
        ident: &str,
        current_scope: usize,
        enclosing_scope: Option<usize>,
    ) -> bool {
        self.binding_exists_enclosing(ident, enclosing_scope)
            || self.get_maybe_value(ident, current_scope).is_some()
    }

    fn struct_binding_exists_local(self: &Self, ident: &str, current_scope: usize) -> bool {
        self.scopes
            .get(current_scope)
            .unwrap()
            .get_struct_binding(&ident)
            .is_some()
    }

    fn binding_exists_local(self: &Self, ident: &str, current_scope: usize) -> bool {
        self.scopes
            .get(current_scope)
            .unwrap()
            .get_binding(&ident)
            .is_some()
    }
}

fn not_found_in_scope_error(ident: &str) -> RuntimeError {
    RuntimeError::new(&format!(
        "Identifier '{ident}' was not found in this scope."
    ))
}

fn assign_immutable_binding_error(ident: &str) -> RuntimeError {
    RuntimeError::new(&format!("Cannot re-assign to immutable binding `{ident}`."))
}

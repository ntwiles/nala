use std::collections::HashMap;

use crate::{ast::terms::Value, types::type_variant::TypeVariant};

use super::{value_binding::ValueBinding, TypeBinding};

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<usize>,
    bindings: HashMap<String, ValueBinding>,
    type_bindings: HashMap<String, TypeBinding>,
}

impl Scope {
    pub fn new(parent: Option<usize>) -> Scope {
        Scope {
            parent,
            bindings: HashMap::new(),
            type_bindings: HashMap::new(),
        }
    }

    pub fn add_binding(
        self: &mut Self,
        ident: &str,
        value: Value,
        declared_type: Option<TypeVariant>,
        is_mutable: bool,
    ) {
        self.bindings
            .insert(ident.to_owned(), ValueBinding { value, is_mutable });
    }

    pub fn add_type_binding(self: &mut Self, ident: &str, value: TypeBinding) {
        self.type_bindings.insert(ident.to_owned(), value);
    }

    pub fn get_binding(self: &Self, ident: &str) -> Option<ValueBinding> {
        if let Some(binding) = self.bindings.get(ident) {
            Some(binding.clone())
        } else {
            None
        }
    }

    pub fn get_type_binding(self: &Self, ident: &str) -> Option<&TypeBinding> {
        if let Some(binding) = self.type_bindings.get(ident) {
            Some(binding)
        } else {
            None
        }
    }
}

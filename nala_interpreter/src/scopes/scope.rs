use std::collections::HashMap;

use crate::{ast::terms::Value, types::struct_field::StructField};

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<usize>,
    bindings: HashMap<String, (Value, bool)>,
    type_bindings: HashMap<String, Vec<StructField>>,
}

impl Scope {
    pub fn new(parent: Option<usize>) -> Scope {
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

use std::collections::HashMap;

use crate::ast;

pub struct Scope {
    parent: Box<Option<Scope>>,
    bindings: HashMap<String, ast::Term>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Scope {
        Scope {
            parent: Box::new(parent),
            bindings: HashMap::<String, ast::Term>::new(),
        }
    }

    pub fn add_binding(self: &mut Self, ident: String, value: ast::Term) {
        self.bindings.insert(ident, value);
    }

    pub fn get_value(self: &Self, ident: String) -> &ast::Term {
        if let Some(value) = self.bindings.get(&ident) {
            value
        } else if let Some(parent) = &*self.parent {
            parent.get_value(ident)
        } else {
            panic!("The identifier '{}' was not found in scope.", ident)
        }
    }
}

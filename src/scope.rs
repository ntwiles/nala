use std::collections::HashMap;

use crate::ast;

pub struct Scope {
    parent: Box<Option<Scope>>,
    bindings: HashMap<String, Option<ast::Term>>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Scope {
        Scope {
            parent: Box::new(parent),
            bindings: HashMap::<String, Option<ast::Term>>::new(),
        }
    }

    pub fn add_binding(self: &mut Self, ident: String, value: Option<ast::Term>) {
        self.bindings.insert(ident, value);
    }
}

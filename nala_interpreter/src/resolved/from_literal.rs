use crate::{errors::RuntimeError, scopes::Scopes};

pub trait FromLiteral<T> {
    fn from_literal(
        literal: T,
        scopes: &mut Scopes,
        current_scope: usize,
    ) -> Result<Self, RuntimeError>
    where
        Self: Sized;
}

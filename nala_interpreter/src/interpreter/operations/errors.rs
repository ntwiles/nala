use crate::{errors::RuntimeError, types::type_variant::TypeVariant};

pub fn oper_not_implemented_error(oper: &str, the_type: &TypeVariant) -> RuntimeError {
    RuntimeError::new(&format!(
        "Operator `{oper}` is not implemented for type {the_type}.",
    ))
}

pub fn oper_not_implemented_for_error(
    oper: &str,
    left: &TypeVariant,
    right: &TypeVariant,
) -> RuntimeError {
    RuntimeError::new(&format!(
        "Operator `{oper}` is not implemented for types {left} and {right}."
    ))
}

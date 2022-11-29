use crate::types::type_variant::TypeVariant;

pub fn panic_oper_not_impl(oper: &str, the_type: &TypeVariant) -> ! {
    panic!(
        "Operator `{0}` is not implemented for type {1}.",
        oper, the_type,
    )
}

pub fn panic_oper_not_impl_for(oper: &str, left: &TypeVariant, right: &TypeVariant) -> ! {
    panic!(
        "Operator `{0}` is not implemented for types {1} and {2}.",
        oper, left, right
    )
}

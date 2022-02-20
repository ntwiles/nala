use crate::{
    ast::{terms::Value, types::*},
    errors::*,
};

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

pub fn check_operator_implemented_both(
    left: TypeVariant,
    right: TypeVariant,
    operator: String,
    interface: PrimitiveInterface,
) -> Result<Value, NalaRuntimeError> {
    check_operator_implemented(left, operator.clone(), interface.clone())?;
    check_operator_implemented(right, operator, interface)?;

    Ok(Value::Void)
}

pub fn check_operator_implemented(
    _type: TypeVariant,
    operator: String,
    interface: PrimitiveInterface,
) -> Result<Value, NalaRuntimeError> {
    if !_type.implements_interface(interface.clone()) {
        Err(operator_not_implemented_error(_type, operator, interface))
    } else {
        Ok(Value::Void)
    }
}

#[derive(Clone, Debug)]
pub struct OperatorNotImplementedError {
    pub _type: TypeVariant,
    pub operator: String,
    pub interface: PrimitiveInterface,
}

fn operator_not_implemented_error(
    _type: TypeVariant,
    operator: String,
    interface: PrimitiveInterface,
) -> NalaRuntimeError {
    NalaRuntimeError {
        message: format!(
            "Cannot use {0} operator with values of type `{1}`. `{1}` Does not implement interface `{2}`.",
            operator,
            _type,
            interface
        )
    }
}

use crate::{
    ast::{terms::Term, types::*},
    errors::*,
};

macro_rules! panic_oper_not_impl {
    ($oper:expr, $type:expr) => {
        panic!(
            "Operator `{0}` is not implemented for type {1}.",
            $oper, $type,
        )
    };

    ($oper:expr, $left:expr, $right:expr) => {
        panic!(
            "Operator `{0}` is not implemented for types {1} and {2}.",
            $oper, $left, $right
        )
    };
}

pub fn check_operator_implemented_both(
    left: TypeVariant,
    right: TypeVariant,
    operator: String,
    interface: PrimitiveInterface,
) -> Result<Term, NalaRuntimeError> {
    let left = check_operator_implemented(left, operator.clone(), interface.clone());
    let right = check_operator_implemented(right, operator, interface);

    return if let Term::Exception(e) = left.clone() {
        Err(e)
    } else if let Term::Exception(e) = right.clone() {
        Err(e)
    } else {
        Ok(Term::Void)
    };
}

pub fn check_operator_implemented(
    _type: TypeVariant,
    operator: String,
    interface: PrimitiveInterface,
) -> Term {
    if !_type.implements_interface(interface.clone()) {
        operator_not_implemented_error(_type, operator, interface)
    } else {
        Term::Void
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
) -> Term {
    Term::Exception(NalaRuntimeError {
        message: format!(
            "Cannot use {0} operator with values of type `{1}`. `{1}` Does not implement interface `{2}`.",
            operator,
            _type,
            interface
        )
    })
}

pub(crate) use panic_oper_not_impl;

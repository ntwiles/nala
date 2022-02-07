use crate::{
    ast::{terms::Term, types::*},
    errors::*,
};

macro_rules! panic_oper_not_impl {
    ($oper:expr, $type:expr) => {
        panic!(
            "Operator `{0}` is not implemented for type {1}.",
            $oper.to_string(),
            $type.to_string(),
        )
    };

    ($oper:expr, $left:expr, $right:expr) => {
        panic!(
            "Operator `{0}` is not implemented for types {1} and {2}.",
            $oper.to_string(),
            $left.to_string(),
            $right.to_string()
        )
    };
}

pub fn check_operator_implemented_both(
    left: TypeVariant,
    right: TypeVariant,
    operator: String,
    interface: PrimitiveInterface,
) -> Result<Term, Term> {
    let left = check_operator_implemented(left, operator.clone(), interface.clone());
    let right = check_operator_implemented(right, operator, interface);

    return if let Term::Exception(_) = left.clone() {
        Err(left)
    } else if let Term::Exception(_) = right.clone() {
        Err(right)
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
            _type.to_string(),
            interface.to_string()
        )
    })
}

pub(crate) use panic_oper_not_impl;

use crate::{ast::types::*, errors::*};

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

pub fn check_operator_implemented(
    _type: TypeVariant,
    operator: String,
    interface: PrimitiveInterface,
) -> () {
    if !_type.implements_interface(interface.clone()) {
        runtime_error(OperatorNotImplementedError {
            _type,
            operator,
            interface,
        })
    }
}

pub struct OperatorNotImplementedError {
    pub _type: TypeVariant,
    pub operator: String,
    pub interface: PrimitiveInterface,
}

impl NalaRuntimeError for OperatorNotImplementedError {
    fn message(&self) -> String {
        format!(
            "Cannot use {0} operator with values of type `{1}`. `{1}` Does not implement interface `{2}`.",
            self.operator,
            self._type.to_string(),
            self.interface.to_string()
        )
    }
}

pub(crate) use panic_oper_not_impl;

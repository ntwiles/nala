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

pub(crate) use panic_oper_not_impl;

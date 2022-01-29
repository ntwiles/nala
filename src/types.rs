use crate::ast::*;

pub fn get_value_type(value: Term) -> ValueType {
    match value {
        Term::Array(_) => ValueType::Array,
        Term::Bool(_) => ValueType::Bool,
        Term::Break(_) => ValueType::Break,
        Term::Func(_, _) => ValueType::Func,
        Term::Num(_) => ValueType::Num,
        Term::String(_) => ValueType::String,
        Term::Symbol(_) => ValueType::Symbol,
        Term::Void => ValueType::Void,
    }
}

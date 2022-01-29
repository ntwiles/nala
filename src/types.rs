use crate::ast::*;

pub fn get_type_name(value: Term) -> String {
    let type_name = match value {
        Term::Array(_) => "Array",
        Term::Bool(_) => "Bool",
        Term::Break(_) => "<Break>",
        Term::Func(_, _) => "Func",
        Term::Num(_) => "Num",
        Term::String(_) => "String",
        Term::Symbol(_) => "<Symbol>",
        Term::Void => "<Void>",
    };

    String::from(type_name)
}

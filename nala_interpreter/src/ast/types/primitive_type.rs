use std::fmt;

#[derive(Eq, Debug, Clone)]
pub enum PrimitiveType {
    Array,
    Bool,
    Break,
    Func,
    Number,
    String,
    Symbol,
    Void,
    Object,
    Any, // TODO: This is temporary until generics can be supported.
}

impl PrimitiveType {
    pub fn is_assignable_to(&self, param: &PrimitiveType) -> bool {
        // TODO: Is this a good way to compare types?
        // TODO: Get rid of Any when we have a better solution.
        if let PrimitiveType::Any = self {
            true
        } else {
            self.to_string() == param.to_string()
        }
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = match self {
            PrimitiveType::Array => "Array",
            PrimitiveType::Bool => "Bool",
            PrimitiveType::Break => "<Break>",
            PrimitiveType::Func => "Func",
            PrimitiveType::Number => "Number",
            PrimitiveType::Object => "<Object>",
            PrimitiveType::String => "String",
            PrimitiveType::Symbol => "<Symbol>",
            PrimitiveType::Void => "Void",
            PrimitiveType::Any => "Any",
        };

        write!(f, "{}", type_name)
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

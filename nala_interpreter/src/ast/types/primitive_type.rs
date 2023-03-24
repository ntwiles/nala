use std::fmt;

#[derive(Eq, Debug, Clone)]
pub enum PrimitiveType {
    Array,
    Bool,
    Break,
    Func,
    Number,
    String,
    Void,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = match self {
            PrimitiveType::Array => "Array",
            PrimitiveType::Bool => "Bool",
            PrimitiveType::Break => "<Break>",
            PrimitiveType::Func => "Func",
            PrimitiveType::Number => "Number",
            PrimitiveType::String => "String",
            PrimitiveType::Void => "Void",
        };

        write!(f, "{}", type_name)
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

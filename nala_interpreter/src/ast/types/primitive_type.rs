use std::fmt;

/* TODO: Get rid of Any. Right now Any is being used in generics to type an enum
 * variant whose value does not allow inference of the generic type. For example,
 * with the following type:
 *
 * Option<T> {
 *   Some(T),
 *   None,
 * }
 *
 * A value Option::Some(8) is known to be of type Option<Number>, but a value of
 * Option::None cannot be inferred. In the future, we should force a type declaration
 * for such values. Right now though we do not support explicit type declarations when
 * declaring variables, so we type the value as Option<Any>
 *
 * Additionally: Builtin functions like `print()` were created before generics were
 * supported. They're currently type to accept Any but should be changed when support
 * for generic functions is added.
 *
 * Do not use `Any` for any situations beyond those outlined above.
 *
 */
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
    Object, // TODO: Should this exist?
    Any,
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

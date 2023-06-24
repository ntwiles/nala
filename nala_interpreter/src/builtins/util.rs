use crate::resolved::value::{EnumVariantValue, Value};

pub fn build_some(data: Value) -> Value {
    let variant = EnumVariantValue {
        enum_ident: String::from("Option"),
        variant_ident: String::from("Some"),
        data: Some(Box::new(data)),
    };

    Value::Variant(variant)
}

pub fn build_none() -> Value {
    let variant = EnumVariantValue {
        enum_ident: String::from("Option"),
        variant_ident: String::from("None"),
        data: None,
    };

    Value::Variant(variant)
}

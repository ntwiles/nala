use crate::{resolved::value::Value, types::type_variant::TypeVariant};

#[derive(Clone, Debug)]
pub struct ValueBinding {
    pub value: Value,
    pub declared_type: Option<TypeVariant>,
    pub is_mutable: bool,
}

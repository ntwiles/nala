use super::{type_variant::TypeVariant, NalaType};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CompositeType {
    pub outer: NalaType,
    pub inner: Vec<TypeVariant>,
}

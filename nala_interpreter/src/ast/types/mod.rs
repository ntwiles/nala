pub mod nala_type;
pub mod primitive_type;
pub mod type_variant;

use super::types::type_variant::TypeVariant;

#[derive(Debug, Clone)]
pub struct StructField {
    pub ident: String,
    pub field_type: TypeVariant
}
use self::type_literal_variant::TypeLiteralVariant;

pub mod primitive_type;
pub mod type_literal;
pub mod type_literal_variant;

#[derive(Debug, Clone)]
pub struct StructLiteralField {
    pub ident: String,
    pub field_type: TypeLiteralVariant,
}

impl StructLiteralField {
    pub fn new(ident: &str, field_type: TypeLiteralVariant) -> Self {
        Self {
            ident: ident.to_owned(),
            field_type,
        }
    }
}
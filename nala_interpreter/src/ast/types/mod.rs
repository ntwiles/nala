use self::type_literal_variant::TypeLiteralVariant;

pub mod enum_variant;
pub mod primitive_type;
pub mod type_literal;
pub mod type_literal_variant;
pub mod variant_declare;

#[derive(Debug, Clone)]
pub enum StructLiteralFieldValue {
    Nested(Vec<StructLiteralField>),
    Type(TypeLiteralVariant),
}

#[derive(Debug, Clone)]
pub struct StructLiteralField {
    pub ident: String,
    pub value: StructLiteralFieldValue,
}

impl StructLiteralField {
    pub fn new(ident: &str, value: StructLiteralFieldValue) -> Self {
        Self {
            ident: ident.to_owned(),
            value,
        }
    }
}

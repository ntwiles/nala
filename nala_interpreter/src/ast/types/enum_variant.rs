use crate::ast::math::Addend;

#[derive(Debug, Clone)]
pub enum EnumVariantOrAddend {
    EnumVariant(String, String),
    Addend(Addend),
}

use crate::ast::{math::Addend, Expr};

#[derive(Debug, Clone)]
pub enum EnumVariantOrAddend {
    EnumVariant(String, String, Option<Box<Expr>>),
    Addend(Addend),
}

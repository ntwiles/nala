use crate::ast::{math::Addition, Expr};

#[derive(Debug, Clone)]
pub enum EnumVariantOrAddition {
    EnumVariant(String, String, Option<Box<Expr>>),
    Addition(Addition),
}

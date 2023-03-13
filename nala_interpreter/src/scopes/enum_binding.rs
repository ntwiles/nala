use crate::ast::types::variant_declare::VariantDeclare;

#[derive(Clone, Debug)]
pub struct EnumBinding {
    pub variants: Vec<VariantDeclare>,
    pub closure_scope: usize,
    pub generic_ident: Option<String>,
}

impl EnumBinding {
    pub fn get_generic_ident(&self) -> Option<String> {
        self.generic_ident.clone()
    }
}

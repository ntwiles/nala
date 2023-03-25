use crate::types::struct_field::StructField;

#[derive(Clone, Debug)]
pub struct StructBinding {
    pub fields: Vec<StructField>,
    pub closure_scope: usize,
    pub generic_ident: Option<String>,
}

impl StructBinding {
    pub fn get_generic_ident(&self) -> Option<String> {
        self.generic_ident.clone()
    }
}

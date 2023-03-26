use crate::resolved::enum_variants::EnumVariant;

#[derive(Clone, Debug)]
pub struct EnumBinding {
    pub variants: Vec<EnumVariant>,
}

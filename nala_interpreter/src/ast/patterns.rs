use crate::resolved::value::Value;

#[derive(Clone, Debug)]
pub enum Pattern {
    Variant(String, String, Option<String>),
}

impl Pattern {
    pub fn matches(&self, value: &Value) -> Option<Vec<(String, Value)>> {
        match self {
            Pattern::Variant(enum_ident, variant_ident, new_ident) => {
                if let Value::Variant(variant) = value {
                    if variant_ident == &variant.variant_ident && enum_ident == &variant.enum_ident
                    {
                        if let Some(new_ident) = new_ident {
                            return Some(vec![(
                                new_ident.to_owned(),
                                *variant.data.clone().unwrap(),
                            )]);
                        } else {
                            return Some(Vec::new());
                        }
                    }
                }

                None
            }
        }
    }
}

use crate::ast::types::{PrimitiveInterface::*, *};

pub fn get_interfaces_for_primitive_type(primitive: PrimitiveType) -> Vec<PrimitiveInterface> {
    match primitive {
        PrimitiveType::Array => vec![IPrint],
        PrimitiveType::Bool => vec![IPrint],
        PrimitiveType::Func => vec![IPrint],
        PrimitiveType::Kind => vec![IPrint],
        PrimitiveType::Number => vec![
            IAdd, ICompare, IDivide, IEqual, IMultiply, IPrint, ISubtract,
        ],
        PrimitiveType::String => vec![IAdd, IEqual, IPrint],
        _ => vec![],
    }
}

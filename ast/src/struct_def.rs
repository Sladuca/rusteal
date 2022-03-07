use std::collections::HashMap;

use crate::typing::TypePrimitive;

#[derive(Default)]
pub struct StructDef {
    pub fields: HashMap<String, TypePrimitive>,
}

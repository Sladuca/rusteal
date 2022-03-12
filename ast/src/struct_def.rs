use std::collections::HashMap;
use std::convert::{AsRef, AsMut};

use crate::typing::TypePrimitive;

pub type FieldMap = HashMap<String, TypePrimitive>;

#[derive(Default)]
pub struct StructDef {
    pub fields: FieldMap,
}

impl AsRef<FieldMap> for StructDef {
    fn as_ref(&self) -> &FieldMap {
        &self.fields
    }
}

impl AsMut<FieldMap> for StructDef {
    fn as_mut(&mut self) -> &mut FieldMap {
        &mut self.fields
    }
}

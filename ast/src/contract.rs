use crate::{program::Program, struct_def::StructDef};

pub struct Contract {
    pub schema_global: StructDef,
    pub schema_local: StructDef,
    pub txn_approval: Program,
    pub txn_clear: Program,
}

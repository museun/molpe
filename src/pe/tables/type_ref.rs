use crate::pe::TableKind;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TypeRefTable {
    pub resolution_scope: u16,
    pub type_name: u16,
    pub type_namespace: u16,
}

impl TypeRefTable {
    pub fn resolution_scope_table_and_entry(self) -> (usize, usize) {
        let tag = self.resolution_scope & 0b0000_0000_0000_0011; // resolution scope
        let table = match tag {
            0 => TableKind::Module.into(),
            1 => TableKind::ModuleRef.into(),
            2 => TableKind::AssemblyRef.into(),
            3 => TableKind::TypeRef.into(),
            _ => unreachable!(),
        };
        let entry = self.resolution_scope as usize >> 2;
        (table, entry)
    }
}

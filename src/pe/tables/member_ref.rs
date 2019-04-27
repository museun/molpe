use crate::pe::TableKind;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct MemberRefTable {
    pub class: u16,
    pub name: u16,
    pub signature: u16,
}

impl MemberRefTable {
    pub fn class_table_and_entry(self) -> (usize, usize) {
        let tag = self.class & 0b0000_0000_0000_0111; // member ref parent
        let table = match tag {
            0 => TableKind::TypeDef.into(),
            1 => TableKind::TypeRef.into(),
            2 => TableKind::ModuleRef.into(),
            3 => TableKind::MethodDef.into(),
            4 => TableKind::TypeSpec.into(),
            _ => unreachable!(),
        };
        let entry = self.class as usize >> 3;
        (table, entry)
    }
}

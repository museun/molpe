#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct TypeDefTable {
    pub flag: u32,
    pub type_name: u16,
    pub type_namespace: u16,
    pub extends: u16,
    pub field_list: u16,
    pub module_list: u16,
}

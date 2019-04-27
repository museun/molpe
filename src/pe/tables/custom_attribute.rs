#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct CustomAttributeTable {
    pub parent: u16,
    pub type_: u16,
    pub value: u16,
}

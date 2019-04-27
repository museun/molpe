#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct MemberDefTable {
    pub rva: u32,
    pub impl_flags: u16,
    pub flags: u16,
    pub name: u16,
    pub signature: u16,
    pub param_list: u16,
}

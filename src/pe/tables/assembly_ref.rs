#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct AssemblyRefTable {
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,
    pub flags: u32,
    pub public_key_or_token: u16,
    pub name: u16,
    pub culture: u16,
    pub hash_value: u16,
}

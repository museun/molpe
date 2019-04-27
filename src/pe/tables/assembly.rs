#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct AssemblyTable {
    pub hash_alg_id: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,
    pub flags: u32,
    pub public_key: u16,
    pub name: u16,
    pub culture: u16,
}

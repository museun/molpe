#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct ModuleTable {
    pub generation: u16,
    pub name: u16,
    pub mvid: u16,
    pub env_id: u16,
    pub env_base_id: u16,
}

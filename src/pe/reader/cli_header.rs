use std::io::{Seek, Read};
use crate::pe::*;

impl<R: Read + Seek> ReadPe<R> for CliHeader {
    fn read(rd: &mut Reader<R>) -> Result<Self> {
        let cb = rd.read_u32()?;
        let major_runtime_version = rd.read_u16()?;
        let minor_runtime_version = rd.read_u16()?;
        let metadata_rva = rd.read_u32()?;
        let metadata_size = rd.read_u32()?;
        let flags = rd.read_u32()?;
        let entry_point_token = rd.read_u32()?;
        let resources_rva = rd.read_u32()?;
        let resources_size = rd.read_u32()?;
        let strong_name_sig_rva = rd.read_u32()?;
        let strong_name_sig_version = rd.read_u32()?;
        let code_manager_table = rd.read_u64()?;
        cmp!(code_manager_table == 0); // reserved
        let vtable_fixups_virtual_address = rd.read_u32()?;
        let vtable_fixup_size = rd.read_u16()?;
        let vtable_fixups_type = rd.read_u16()?;
        // export address tables jumps (u64) == 0
        // managed native header (u64) == 0
        Ok(Self {
            cb,
            major_runtime_version,
            minor_runtime_version,
            metadata_rva,
            metadata_size,
            flags,
            entry_point_token,
            resources_rva,
            resources_size,
            strong_name_sig_rva,
            strong_name_sig_version,
            vtable_fixups_virtual_address,
            vtable_fixup_size,
            vtable_fixups_type,
        })
    }
}

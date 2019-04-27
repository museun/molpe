use std::io::{Seek, Read};
use crate::pe::*;

impl<R: Read + Seek> ReadPe<R> for OptionalHeader {
    fn read(rd: &mut Reader<R>) -> Result<Self> {
        cmp!(rd.read_u16()? == 0x10b); // magic

        let _major = rd.read_u8()?;
        cmp!(rd.read_u8()? == 0); //minor

        let code_size = rd.read_u32()?;
        let initialized_data_size = rd.read_u32()?;
        let uninitialized_data_size = rd.read_u32()?;
        let entry_point_rva = rd.read_u32()?;
        let base_of_code = rd.read_u32()?;
        let base_of_data = rd.read_u32()?;
        let image_base = rd.read_u32()?;
        let section_alignment = rd.read_u32()?;

        cmp!(rd.read_u32()? == 0x200); // file_alignment

        let os_major = rd.read_u16()?;
        cmp!(os_major == 5 || os_major == 4);

        cmp!(rd.read_u16()? == 0); // os_minor
        cmp!(rd.read_u16()? == 0); //user_major
        cmp!(rd.read_u16()? == 0); // user_minor

        let subsystem_major = rd.read_u16()?;
        cmp!(subsystem_major == 5 || subsystem_major == 4);
        cmp!(rd.read_u16()? == 0); // subsystem_minor
        cmp!(rd.read_u32()? == 0); // reserved

        let image_size = rd.read_u32()?;
        let header_size = rd.read_u32()?;

        cmp!(rd.read_u32()? == 0); // checksum

        let sub_system = rd.read_u16()?;
        let dll_flags = rd.read_u16()?;
        let stack_reserve_size = rd.read_u32()?;
        let stack_commit_size = rd.read_u32()?;
        let heap_reserve_size = rd.read_u32()?;
        let heap_commit_size = rd.read_u32()?;
        let loader_flags = rd.read_u32()?;
        let number_of_data_dirs = rd.read_u32()?;

        cmp!(rd.read_u64()? == 0); // export_table

        let import_table_rva = rd.read_u32()?;
        let import_table_size = rd.read_u32()?;

        let _resource_table = rd.read_u64()?;
        // cmp!(resource_table == 0); // TODO // TODO even more

        cmp!(rd.read_u64()? == 0); // exception_table
        cmp!(rd.read_u64()? == 0); // certification_table

        let base_reloc_table_rva = rd.read_u32()?;
        let base_reloc_table_size = rd.read_u32()?;

        cmp!(rd.read_u64()? == 0); // is_debug
        cmp!(rd.read_u64()? == 0); // copyright
        cmp!(rd.read_u64()? == 0); // global_pointer
        cmp!(rd.read_u64()? == 0); // tls_table
        cmp!(rd.read_u64()? == 0); // load_config_table
        cmp!(rd.read_u64()? == 0); // bound_import

        let iat_rva = rd.read_u32()?;
        let iat_size = rd.read_u32()?;

        cmp!(rd.read_u64()? == 0); // delay_import_descriptor

        let cli_header_rva = rd.read_u32()?;
        let cli_header_size = rd.read_u32()?;

        cmp!(rd.read_u64()? == 0); // reserved

        Ok(Self {
            code_size,
            initialized_data_size,
            uninitialized_data_size,
            entry_point_rva,
            base_of_code,
            base_of_data,
            image_base,
            section_alignment,
            image_size,
            header_size,
            sub_system,
            dll_flags,
            stack_reserve_size,
            stack_commit_size,
            heap_reserve_size,
            heap_commit_size,
            loader_flags,
            number_of_data_dirs,
            import_table_rva,
            import_table_size,
            base_reloc_table_rva,
            base_reloc_table_size,
            iat_rva,
            iat_size,
            cli_header_rva,
            cli_header_size,
        })
    }
}

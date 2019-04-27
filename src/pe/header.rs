#[derive(Debug, Copy, Clone)]
pub struct MsDosHeader {}

/// 11.25.2.2
#[derive(Debug, Copy, Clone)]
pub struct FileHeader {
    pub number_of_sections: u16,
    pub timestamp: u32,
    pub optional_header_size: u16,
    pub characteristics: u16,
}

/// 11.25.2.3.1
#[derive(Debug, Clone)]
pub struct OptionalHeader {
    pub code_size: u32,
    pub initialized_data_size: u32,
    pub uninitialized_data_size: u32,
    pub entry_point_rva: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub image_size: u32,
    pub header_size: u32,
    pub sub_system: u16,
    pub dll_flags: u16,
    pub stack_reserve_size: u32,
    pub stack_commit_size: u32,
    pub heap_reserve_size: u32,
    pub heap_commit_size: u32,
    pub loader_flags: u32,
    pub number_of_data_dirs: u32,
    pub import_table_rva: u32,
    pub import_table_size: u32,
    pub base_reloc_table_rva: u32,
    pub base_reloc_table_size: u32,
    pub iat_rva: u32,
    pub iat_size: u32,
    pub cli_header_rva: u32,
    pub cli_header_size: u32,
}

/// 11.25.3
#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: String, // SmallString
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocs: u32,
    pub pointer_to_line_numbers: u32,
    pub number_of_relocs: u16,
    pub number_of_line_numbers: u16,
    pub characteristics: u32,
}

/// 11.25.3.3
#[derive(Debug, Clone)]
pub struct CliHeader {
    pub cb: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub metadata_rva: u32,
    pub metadata_size: u32,
    pub flags: u32,
    pub entry_point_token: u32,
    pub resources_rva: u32,
    pub resources_size: u32,
    pub strong_name_sig_rva: u32,
    pub strong_name_sig_version: u32,
    pub vtable_fixups_virtual_address: u32,
    pub vtable_fixup_size: u16,
    pub vtable_fixups_type: u16,
}

#[derive(Debug, Clone)]
pub struct MetaDataHeader {
    pub version: String,
    pub streams: u16,
}

#[derive(Debug, Clone)]
pub struct StreamHeader {
    pub offset: u32,
    pub size: u32,
    pub name: String,
}

#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum PeCharacteristics {
    IMAGE_FILE_RELOCS_STRIPPED = 0x0001,
    IMAGE_FILE_EXE_IMAGE = 0x0002,
    IMAGE_FILE_32BIT_MACHINE = 0x0100,
    IMAGE_FILE_DLL = 0x2000,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SectionCharacteristics {
    IMAGE_SCN_CNT_CODE = 0x0000_0020,
    IMAGE_SCN_CNT_INITIALIZED_DATA = 0x0000_0040,
    IMAGE_SCN_CNT_UNINITIALIZED_DATA = 0x0000_0080,
    IMAGE_SCN_MEM_EXECUTE = 0x2000_0000,
    IMAGE_SCN_MEM_READ = 0x4000_0000,
    IMAGE_SCN_MEM_WRITE = 0x8000_0000,
}

#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SubSystem {
    IMAGE_SUBSYSTEM_WINDOWS_CLI = 0x03,
    IMAGE_SUBSYSTEM_WINDOWS_GUI = 0x02,
}

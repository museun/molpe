use std::io::{Seek, Read};
use crate::pe::*;

impl<R: Read + Seek> ReadPe<R> for FileHeader {
    fn read(rd: &mut Reader<R>) -> Result<Self> {
        const PE_MAGIC: [u8; 4] = [b'P', b'E', 0, 0];
        let mut signature = [0u8; 4];
        rd.read_exact(&mut signature)?;
        cmp!(signature == PE_MAGIC);

        cmp!(rd.read_u16()? == 0x14C); // machine

        let number_of_sections = rd.read_u16()?;
        let timestamp = rd.read_u32()?;

        cmp!(rd.read_u32()? == 0); // symbol_table
        cmp!(rd.read_u32()? == 0); // number_of_symbols

        let optional_header_size = rd.read_u16()?;
        let characteristics = rd.read_u16()?;

        Ok(Self {
            number_of_sections,
            timestamp,
            optional_header_size,
            characteristics,
        })
    }
}

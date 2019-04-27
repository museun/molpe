use std::io::{Seek, Read};
use crate::pe::*;

impl<R: Read + Seek> ReadPe<R> for SectionHeader {
    fn read(rd: &mut Reader<R>) -> Result<Self> {
        Ok(Self {
            name: rd.read_string(&mut [0u8; 8])?,
            virtual_size: rd.read_u32()?,
            virtual_address: rd.read_u32()?,
            size_of_raw_data: rd.read_u32()?,
            pointer_to_raw_data: rd.read_u32()?,
            pointer_to_relocs: rd.read_u32()?,
            pointer_to_line_numbers: rd.read_u32()?,
            number_of_relocs: rd.read_u16()?,
            number_of_line_numbers: rd.read_u16()?,
            characteristics: rd.read_u32()?,
        })
    }
}

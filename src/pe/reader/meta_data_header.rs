use std::io::{Seek, Read};
use crate::pe::*;

impl<R: Read + Seek> ReadPe<R> for MetaDataHeader {
    fn read(rd: &mut Reader<R>) -> Result<Self> {
        cmp!(rd.read_u32()? == 0x424A_5342); // signature
        cmp!(rd.read_u16()? == 1); // major
        cmp!(rd.read_u16()? == 1); // minor
        cmp!(rd.read_u32()? == 0); // reserved

        let mut version = vec![0u8; rd.read_u32()? as usize];
        let version = rd.read_string(&mut version)?;

        cmp!(rd.read_u16()? == 0); // reserved

        let streams = rd.read_u16()?;
        Ok(Self { version, streams })
    }
}

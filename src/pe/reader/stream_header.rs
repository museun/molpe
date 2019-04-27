use std::io::{Seek, Read};
use crate::pe::*;

impl<R: Read + Seek> ReadPe<R> for StreamHeader {
    fn read(rd: &mut Reader<R>) -> Result<Self> {
        let offset = rd.read_u32()?;
        let size = rd.read_u32()?;

        let mut name = String::new();
        let mut count = 1;
        loop {
            let c = rd.read_u8()?;
            if c == 0 {
                while count % 4 != 0 {
                    rd.read_u8()?;
                    count += 1;
                }
                break;
            }
            name.push(c as char);
            count += 1;
        }
        Ok(Self { offset, size, name })
    }
}

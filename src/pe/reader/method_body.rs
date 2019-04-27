use std::io::{Seek, Read};
use crate::pe::*;
use crate::exec::Decoder;

impl<R: Read + Seek> ReadPe<R> for MethodBody {
    fn read(rd: &mut Reader<R>) -> Result<Self> {
        use std::convert::TryInto;

        let head = rd.read_u8()?;
        let ty: MethodHeaderType = head.try_into().expect("valid method header type"); // TODO handle this
        match ty {
            MethodHeaderType::TinyFormat { bytes } => {
                let mut body = vec![0u8; bytes];
                rd.read_exact(&mut body)?;
                let body = Decoder::new(&body).collect();
                Ok(Self { ty, body })
            }
            MethodHeaderType::FatFormat => unimplemented!("TODO implement the fat format"),
        }
    }
}

pub struct Decoder<'a> {
    iter: std::slice::Iter<'a, u8>,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { iter: data.iter() }
    }
}

impl<'a> Iterator for Decoder<'a> {
    type Item = Instruction;
    fn next(&mut self) -> Option<Self::Item> {
        let inst = match self.iter.next()? {
            // ldstr
            0x72 => {
                let token = self.read_u32()?;
                assert!(token & 0xFF00_0000 == 0x7000_0000);
                let us_offset = token & 0x00FF_FFFF;
                Instruction::Ldstr { us_offset }
            }
            // call
            0x28 => {
                let token = self.read_u32()?;
                let table = token as usize >> (32 - 8);
                let entry = token as usize & 0x00FF_FFFF;
                Instruction::Call { table, entry }
            }
            // ret
            0x2a => Instruction::Ret,
            x => {
                log::trace!("unknown inst: {:#X}", x);
                Instruction::Unknown(*x)
            }
        };

        Some(inst)
    }
}

impl<'a> Decoder<'a> {
    fn read_u32(&mut self) -> Option<u32> {
        let w = u32::from(*self.iter.next()?);
        let x = u32::from(*self.iter.next()?);
        let y = u32::from(*self.iter.next()?);
        let z = u32::from(*self.iter.next()?);
        Some((z << 24) + (y << 16) + (x << 8) + w)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Ldstr { us_offset: u32 },
    Call { table: usize, entry: usize },
    Ret,

    // Temporary
    Unknown(u8),
}

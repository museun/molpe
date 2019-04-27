use std::rc::Rc;
use std::cell::RefCell;

use crate::exec::Instruction;

pub type MethodBodyRef = Rc<RefCell<MethodBody>>;

#[derive(Debug, Clone, PartialEq)]
pub struct MethodBody {
    pub ty: MethodHeaderType,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MethodHeaderType {
    TinyFormat { bytes: usize },
    FatFormat,
}

impl std::convert::TryFrom<u8> for MethodHeaderType {
    type Error = ();
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        const TINY_FORMAT: u8 = 0x02;
        const FAT_FORMAT: u8 = 0x03;

        match val & 0b0000_0011 {
            TINY_FORMAT => Ok(MethodHeaderType::TinyFormat {
                bytes: val as usize >> 2,
            }),
            FAT_FORMAT => Ok(MethodHeaderType::FatFormat),
            _ => Err(()),
        }
    }
}

use std::io::{Seek, SeekFrom, BufReader, Read};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::*;

#[macro_use]
macro_rules! cmp {
    ($expr:expr) => {{
        if !$expr {
            if cfg!(test) {
                panic!("bad read");
            } else {
                return Err(Error::InvalidData);
            }
        }
    }};
}

pub(crate) trait ReadPe<R: Read + Seek>
where
    Self: Sized,
{
    fn read(rd: &mut Reader<R>) -> Result<Self>;
}

mod method_body;
mod ms_dos_header;
mod file_header;
mod optional_header;
mod section_header;
mod cli_header;
mod meta_data_header;
mod stream_header;
mod meta_data_stream;
mod meta_data_streams;

pub use self::{
    method_body::*,
    ms_dos_header::*,
    file_header::*,
    optional_header::*,
    section_header::*,
    cli_header::*,
    meta_data_header::*,
    stream_header::*,
    meta_data_stream::*,
    meta_data_streams::*,
};

pub struct Reader<R> {
    reader: BufReader<R>,
}

impl<R: Read + Seek> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    pub fn read_entry_method(&mut self, image: &mut Image) -> Result<MethodBodyRef> {
        let text_section = image
            .cli_info
            .sections
            .iter()
            .find(|section| section.name == ".text")
            .expect("text section");

        let kind = image.cli_info.cli_header.entry_point_token as usize >> (32 - 8);
        let row = image.cli_info.cli_header.entry_point_token as usize & 0x00FF_FFFF;
        log::debug!("entrytoken: kind: {:02X}, row: {:02X}", kind, row);

        let method_or_file = &image.metadata.metadata_stream.tables[kind][row - 1];
        let method = match method_or_file {
            Table::MethodDef(t) => t,
            _ => unimplemented!("TODO handle file"),
        };
        log::debug!("entry point: {:#?}", method);

        let start =
            u64::from(method.rva - text_section.virtual_address + text_section.pointer_to_raw_data);
        log::debug!("start: 0x{:X}", start);
        self.seek(SeekFrom::Start(start))?;

        let method = MethodBody::read(self)?;
        log::debug!("method: {:#?}", method);

        let method_ref = Rc::new(RefCell::new(method));
        image.method_cache.insert(row, Rc::clone(&method_ref));

        Ok(method_ref)
    }

    pub fn create_image(&mut self) -> Result<Image> {
        MsDosHeader::read(self)?;

        let pe = FileHeader::read(self)?;
        let mut headers = vec![];
        for i in 0..pe.optional_header_size / 0xE0 {
            let header = OptionalHeader::read(self)?;
            log::trace!("{}: {:#?}", i, header);
            headers.push(header);
        }

        let mut sections = vec![];
        for i in 0..pe.number_of_sections {
            let section = SectionHeader::read(self)?;
            log::trace!("{}: {:#?}", i, section);
            sections.push(section)
        }

        let text_section = sections
            .iter()
            .find(|k| k.name == ".text")
            .ok_or_else(|| panic!("TODO missing text section"))?;
        let cli_start = u64::from(text_section.pointer_to_raw_data + 8);
        log::info!(".text starts at {:#X}", cli_start);

        self.reader
            .seek(SeekFrom::Start(cli_start))
            .map_err(Error::Io)?;

        let cli_header = CliHeader::read(self)?;
        log::trace!("{:#?}", cli_header);

        let metadata_start = cli_header.metadata_rva - text_section.virtual_address
            + text_section.pointer_to_raw_data;
        log::info!("metadata starts at: {:#X}", metadata_start);

        self.reader
            .seek(SeekFrom::Start(u64::from(metadata_start)))
            .map_err(Error::Io)?;

        let metadata_header = MetaDataHeader::read(self)?;
        log::trace!("{:#?}", metadata_header);

        let mut stream_headers = vec![];
        for _ in 0..metadata_header.streams {
            stream_headers.push(StreamHeader::read(self)?);
        }
        log::trace!("{:#?}", stream_headers);

        let metadata_streams = MetaDataStreams::parse(
            self, //
            u64::from(metadata_start),
            &stream_headers,
        )?;
        log::trace!("{:#?}", metadata_streams);

        Ok(Image {
            cli_info: CliInfo {
                cli_header,
                sections,
            },
            metadata: metadata_streams,
            method_cache: HashMap::new(),
        })
    }
}

impl<R: Read + Seek> Reader<R> {
    pub(crate) fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.reader.seek(pos).map_err(Error::Io)
    }
}

impl<R: Read> Reader<R> {
    pub(crate) fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.reader.read_exact(buf).map_err(Error::Io)
    }

    pub(crate) fn read_string(&mut self, buf: &mut [u8]) -> Result<String> {
        self.read_exact(buf).map(|_| {
            buf.iter()
                .take_while(|&&b| b != 0)
                .map(|&b| b as char)
                .collect()
        })
    }

    pub(crate) fn read_blob_length(&mut self) -> Result<(u32, u32)> {
        let head = u32::from(self.read_u8()?);
        if head & 0b1000_0000 == 0 {
            Ok((head & 0b0111_1111, 1))
        } else if head & 0b1000_0000 > 0 {
            let head = ((head & 0b0111_1111) << 8) + u32::from(self.read_u8()?);
            Ok((head, 2))
        } else if head & 0b1100_0000 > 0 {
            let head = ((head & 0b0011_1111) << 24)
                + (u32::from(self.read_u8()?) << 16)
                + (u32::from(self.read_u8()?) << 8)
                + u32::from(self.read_u8()?);
            Ok((head, 4))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub(crate) fn read_struct<T>(&mut self) -> Result<T> {
        let size = std::mem::size_of::<T>();
        unsafe {
            let mut out = std::mem::zeroed();
            self.read_exact(std::slice::from_raw_parts_mut(
                &mut out as *mut _ as *mut u8,
                size,
            ))?;
            Ok(out)
        }
    }

    pub(crate) fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        let res = (u64::from(buf[7]) << 56)
            + (u64::from(buf[6]) << 48)
            + (u64::from(buf[5]) << 40)
            + (u64::from(buf[4]) << 32)
            + (u64::from(buf[3]) << 24)
            + (u64::from(buf[2]) << 16)
            + (u64::from(buf[1]) << 8)
            + (u64::from(buf[0]));
        Ok(res)
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        let res = (u32::from(buf[3]) << 24)
            + (u32::from(buf[2]) << 16)
            + (u32::from(buf[1]) << 8)
            + u32::from(buf[0]);
        Ok(res)
    }

    pub(crate) fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        let res = (u16::from(buf[1]) << 8) + u16::from(buf[0]);
        Ok(res)
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

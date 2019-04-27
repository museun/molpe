use std::io::{Seek, SeekFrom, Read};
use std::collections::HashMap;
use crate::pe::*;

impl MetaDataStreams {
    pub fn parse<R: Read + Seek>(
        rd: &mut Reader<R>,
        offset: u64,
        headers: &[StreamHeader],
    ) -> Result<Self> {
        let mut metadata_stream = None;
        let mut strings = None;
        let mut user_strings = None;
        let mut blob = None;
        let mut guid = None;

        for stream in headers {
            rd.seek(SeekFrom::Start(offset + u64::from(stream.offset)))?;

            match stream.name.as_str() {
                "#~" => {
                    metadata_stream = Some(MetaDataStream::read(rd)?);
                    log::trace!("#~: {:#?}", metadata_stream)
                }
                "#Strings" => {
                    let mut map = HashMap::new();
                    let mut buf = vec![];
                    let mut pos = 0;
                    for i in 0..stream.size {
                        let ch = rd.read_u8()?;
                        if ch == 0 {
                            map.insert(
                                pos,
                                std::str::from_utf8(&buf).expect("valid utf-8").to_string(),
                            );
                            buf.clear();
                            pos = i + 1;
                        } else {
                            buf.push(ch);
                        }
                    }
                    strings = Some(map)
                }
                "#US" => {
                    let mut map = HashMap::new();
                    let mut buf = vec![];
                    let mut count = 0;
                    while count < stream.size {
                        let (len, offset) = rd.read_blob_length()?;
                        for _ in 0..len / 2 {
                            buf.push(rd.read_u16()?);
                        }
                        map.insert(count as u32, buf.clone());
                        buf.clear();
                        count += len + offset as u32
                    }
                    user_strings = Some(map)
                }
                "#Blob" => {
                    let mut map = HashMap::new();
                    let mut buf = vec![];
                    let mut count = 0;
                    while count < stream.size {
                        let (len, offset) = rd.read_blob_length()?;
                        for _ in 0..len {
                            buf.push(rd.read_u8()?);
                        }
                        map.insert(count as u32, buf.clone());
                        buf.clear();
                        count += offset + len as u32
                    }
                    blob = Some(map)
                }
                "#GUID" => {
                    let a = rd.read_u32()?;
                    let b = rd.read_u16()?;
                    let c = rd.read_u16()?;
                    let mut d = [0u8; 8];
                    rd.read_exact(&mut d)?;
                    guid = Some(format!(
                        "{:08X}-{:04X}-{:04X}{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                        a, b, c, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
                    ))
                }
                _ => unreachable!(),
            }
        }

        Ok(Self {
            metadata_stream: metadata_stream.unwrap(),
            strings: strings.unwrap(),
            user_strings: user_strings.unwrap(),
            blob: blob.unwrap(),
            guid: guid.unwrap(),
        })
    }
}

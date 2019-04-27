use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
/// #~ 11.24.2.6
pub struct MetaDataStream {
    pub major_version: u8,
    pub minor_version: u8,
    pub heap_sizes: u8,
    pub valid: u64,
    pub sorted: u64,
    pub rows: Vec<u32>,
    pub tables: Vec<Vec<Table>>,
}

#[derive(Debug, Clone)]
pub struct MetaDataStreams {
    pub metadata_stream: MetaDataStream,
    pub strings: HashMap<u32, String>,
    pub user_strings: HashMap<u32, Vec<u16>>,
    pub blob: HashMap<u32, Vec<u8>>,
    pub guid: String,
}

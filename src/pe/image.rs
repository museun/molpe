use std::collections::HashMap;

use super::*;

#[derive(Clone)]
pub struct Image {
    pub cli_info: CliInfo,
    pub metadata: MetaDataStreams,
    pub method_cache: HashMap<usize, MethodBodyRef>,
}

impl Image {
    pub fn get_string<I>(&self, index: I) -> &str
    where
        I: Into<u32>,
    {
        &self.metadata.strings[&index.into()].as_str()
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("cli_info", &self.cli_info)
            .field("metadata", &self.metadata)
            .field("method_cache", &self.method_cache)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct CliInfo {
    pub cli_header: CliHeader,
    pub sections: Vec<SectionHeader>,
}

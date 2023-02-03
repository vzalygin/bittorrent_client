use super::metadata::TorrentMetadata;
use crate::{
    io::{
        serialization::{BencodeDictBuilder, SerializeTo},
        consts::*,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Torrent {
    pub data: TorrentMetadata,
    pub hash: [u8; 20],
}

impl SerializeTo<Vec<u8>> for Torrent {
    fn serialize(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(DATA, self.data.clone())
            .required(HASH, self.hash.to_vec())
            .fin()
    }
}
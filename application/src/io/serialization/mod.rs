pub mod error;
pub mod serialize;

mod consts;
mod deserialize;
mod node;
mod parsing;

#[cfg(test)]
mod tests;

use crate::common_types::{files::TorrentFile, data::Torrent};
use error::ParsingError;
use parsing::parse_node;
use sha1::{Sha1, Digest};
use self::{node::Node, consts::INFO};

use super::repo::TorrentRepo;

pub fn make_torrent_from_bytes(bytes: &[u8]) -> Result<Torrent, ParsingError> {
    let node = parse_node(bytes);

    if let Ok((_, node)) = node {
        Ok(Torrent {
            hash: get_info_hash(&node)?,
            data: TorrentFile::try_from(node)?,
        })
    } else {
        Err(ParsingError::InvalidFormat)
    }
}

fn get_info_hash(node: &Node) -> Result<[u8; 20], ParsingError> {
    if let Node::Dict(dict, w) = node {
        if let Some(v) = dict.get(INFO) {
            if let Node::Dict(_, raw) = node {
                let mut hasher = Sha1::new();
                hasher.update(raw);
                Ok(hasher.finalize().into())
            } else {
                Err(ParsingError::InvalidFormat)
            }
        } else {
            Err(ParsingError::InvalidFormat)
        }
    } else {
        Err(ParsingError::InvalidFormat)
    }
}

pub fn deserialize_torrent_repo(bytes: &[u8]) -> Result<TorrentRepo, ParsingError> {
    let node = parse_node(bytes);

    if let Ok((_, node)) = node {
        TorrentRepo::try_from(node)
    } else {
        Err(ParsingError::InvalidFormat)
    }
}

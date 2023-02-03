pub mod error;

mod deserialize;
mod parsing;
mod primitives;
mod util;

#[cfg(test)]
mod tests;

pub use parsing::parse_node;
pub use util::{Node, optional, required};

use sha1::{Digest, Sha1};

use super::{consts::INFO, repo::TorrentRepo};
use crate::common_types::{data::Torrent, metadata::TorrentMetadata};
use error::ParsingError;

pub fn make_torrent_from_bytes(bytes: &[u8]) -> Result<Torrent, ParsingError> {
    let node = parse_node(bytes);

    if let Ok((_, node)) = node {
        Ok(Torrent {
            hash: get_info_hash(&node)?,
            data: TorrentMetadata::try_from(node)?,
        })
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

fn get_info_hash(node: &Node) -> Result<[u8; 20], ParsingError> {
    if let Node::Dict(torrent, _) = node {
        if let Some(info) = torrent.get(INFO) {
            if let Node::Dict(_, raw) = info {
                let mut hasher = Sha1::new();
                hasher.update(raw);
                return Ok(hasher.finalize().into());
            }
        }
    }
    return Err(ParsingError::InvalidFormat);
}
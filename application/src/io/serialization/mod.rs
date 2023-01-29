pub mod error;

mod consts;
mod deserialize;
mod node;
mod parsing;
mod serialize;

#[cfg(test)]
mod tests;

use crate::common_types::files::TorrentFile;
use error::ParsingError;
use parsing::parse_node;

use super::repo::TorrentRepo;

impl TryFrom<&[u8]> for TorrentFile {
    type Error = ParsingError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let node = parse_node(value);

        if let Ok((_, node)) = node {
            node.try_into()
        } else {
            Err(ParsingError::InvalidFormat)
        }
    }
}

impl TryFrom<&[u8]> for TorrentRepo {
    type Error = ParsingError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let node = parse_node(value);

        if let Ok((_, node)) = node {
            node.try_into()
        } else {
            Err(ParsingError::InvalidFormat)
        }
    }
}

pub mod error;

mod node;
mod parsing;

#[cfg(test)]
mod tests;

use crate::common_types::files::TorrentFile;
use error::ParsingError;
use parsing::parse_node;

use super::repo::TorrentRepo;

impl TryInto<TorrentFile> for &[u8] {
    type Error = ParsingError;

    fn try_into(self) -> Result<TorrentFile, Self::Error> {
        let node = parse_node(self);

        if let Ok((_, node)) = node {
            node.try_into()
        } else {
            Err(ParsingError::InvalidFormat)
        }
    }
}

impl TryInto<TorrentRepo> for &[u8] {
    type Error = ParsingError;

    fn try_into(self) -> Result<TorrentRepo, Self::Error> {
        let node = parse_node(self);

        if let Ok((_, node)) = node {
            node.try_into()
        } else {
            Err(ParsingError::InvalidFormat)
        }
    }
}

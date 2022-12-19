pub mod error;

mod parsing;
mod node;

#[cfg(test)]
mod tests;

use crate::common_types::files::Torrent;
use error::ParsingError;
use parsing::{parse_node};

impl TryInto<Torrent> for &[u8] {
    type Error = ParsingError;

    fn try_into(self) -> Result<Torrent, Self::Error> {
        let node = parse_node(self);

        if let Ok((_, node)) = node {
            node.try_into()
        } else {
            Err(ParsingError::InvalidFormat)
        }
    }
}
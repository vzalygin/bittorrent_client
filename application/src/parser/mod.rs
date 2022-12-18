pub mod error;

mod parsing;
mod node;

#[cfg(test)]
mod tests;

use crate::common_types::files::Torrent;
use error::ParsingError;
use parsing::{parse_node};

pub fn parse_from_bytes<'a>(bytes: &'a [u8]) -> Result<Torrent, ParsingError> {
    let node = parse_node(bytes);

    if let Ok((_, node)) = node {
        node.try_into()
    } else {
        Err(ParsingError::InvalidFormat)
    }
}
mod parsing;

use crate::common_types::{
    files::Torrent,
    errors::ParsingError,
};

use self::parsing::parse_node;

pub fn parse_from_bytes(bytes: &[u8]) -> Result<Torrent, ParsingError> {
    let node = parse_node(bytes);

    if let Ok((_, node)) = node {
        unimplemented!()
    } else {
        Err(ParsingError::InvalidFormat)
    }
}
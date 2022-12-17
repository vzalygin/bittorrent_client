mod parsing;

use crate::common_types::{
    files::Torrent,
    errors::ParsingError,
};

pub fn parse_from_bytes(bytes: &[u8]) -> Result<Torrent, ParsingError> {
    unimplemented!()
}
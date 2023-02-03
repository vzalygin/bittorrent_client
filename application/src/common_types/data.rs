use crate::{
    common_types::metadata::TorrentMetadata,
    io::{
        consts::*,
        deserialization::{required, Node, ParsingError},
        serialization::{BencodeDictBuilder, SerializeTo},
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

impl<'a> TryFrom<Node<'a>> for Torrent {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            let hash: Vec<u8> = required(HASH, &dict)?; // Копилятор без подсказки не смог в двойной вывод типов.

            Ok(Torrent {
                data: required(DATA, &dict)?,
                hash: hash.try_into().unwrap(),
            })
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

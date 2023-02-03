/// Модуль с имплементациями для структурных типов.
use std::{collections::HashMap, fmt::Debug};

use crate::{
    common_types::{
        data::Torrent,
        metadata::{
            FileMetadata, FilesMetadata, Info, MultipleFileMode, SingleFileMode, TorrentMetadata,
        },
    },
    io::repo::{Id, TorrentRepo, WithId},
};

use super::{
    super::consts::{
        ANNOUNCE, ANNOUNCE_LIST, COMMENT, CREATED_BY, CREATION_DATE, DATA, ENCODING, FILES, HASH,
        HTTPSEEDS, ID, INFO, LENGTH, MD5SUM, NAME, PATH, PIECES, PIECE_LENGTH, PRIVATE, TORRENTS,
        VALUE,
    },
    util::Node,
};

use super::error::ParsingError;

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

impl<'a, T> TryFrom<Node<'a>> for WithId<T>
where
    T: TryFrom<Node<'a>, Error = ParsingError> + Clone + PartialEq + Debug,
{
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            let value: T = {
                // По неясным причинам борроу чекер не вывозит проверку без инлайна
                let key: &[u8] = VALUE;
                let dict = &dict;
                if let Some(node) = dict.get(key) {
                    node.clone().try_into()
                } else {
                    Err(ParsingError::MissingField(
                        String::from_utf8(key.to_vec()).unwrap(),
                    ))
                }
            }?;
            let id: Id = required(ID, &dict)?;

            Ok(WithId { value, id })
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for Id {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::String(s) = value {
            if s.len() == 16 {
                let s = s[0..16].to_vec();

                Ok(Id::from_bytes(s.try_into().unwrap()))
            } else {
                Err(ParsingError::InvalidFormat)
            }
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for TorrentRepo {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            Ok(TorrentRepo {
                torrents: required(TORRENTS, &dict)?,
            })
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

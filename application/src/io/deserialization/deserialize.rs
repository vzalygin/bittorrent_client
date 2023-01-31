/// Модуль с имплементациями для структурных типов.
use std::{collections::HashMap, fmt::Debug};

use crate::{
    common_types::{
        data::Torrent,
        files::{File, Files, Info, MultipleFileMode, SingleFileMode, TorrentFile},
    },
    io::repo::{Id, TorrentRepo, WithId},
};

use super::{
    super::consts::{
        ANNOUNCE, ANNOUNCE_LIST, COMMENT, CREATED_BY, CREATION_DATE, DATA, ENCODING, FILES, HASH,
        HTTPSEEDS, ID, INFO, LENGTH, MD5SUM, NAME, PATH, PIECES, PIECE_LENGTH, PRIVATE, TORRENTS,
        VALUE,
    },
    parsing::Node,
};

use super::error::ParsingError;

impl<'a> TryFrom<Node<'a>> for File {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            Ok(File {
                path: required(PATH, &dict)?,
                length: required(LENGTH, &dict)?,
                md5sum: optional(MD5SUM, &dict)?,
            })
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for Info {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            let files = {
                let single = dict.contains_key(b"length" as &[u8]);
                let multi = dict.contains_key(b"files" as &[u8]);

                if single && !multi {
                    Files::Single(SingleFileMode {
                        name: required(NAME, &dict)?,
                        length: required(LENGTH, &dict)?,
                        md5sum: optional(MD5SUM, &dict)?,
                    })
                } else if !single && multi {
                    Files::Multiple(MultipleFileMode {
                        base_name: required(NAME, &dict)?,
                        files: required(FILES, &dict)?,
                    })
                } else {
                    return Err(ParsingError::InvalidFormat);
                }
            };

            Ok(Info {
                piece_length: required(PIECE_LENGTH, &dict)?,
                pieces: required(PIECES, &dict)?,
                private: optional(PRIVATE, &dict)?,
                files,
            })
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for TorrentFile {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            Ok(TorrentFile {
                info: required(INFO, &dict)?,
                announce: required(ANNOUNCE, &dict)?,
                encoding: optional(ENCODING, &dict)?,
                httpseeds: optional(HTTPSEEDS, &dict)?,
                announce_list: optional(ANNOUNCE_LIST, &dict)?,
                creation_date: optional(CREATION_DATE, &dict)?,
                comment: optional(COMMENT, &dict)?,
                created_by: optional(CREATED_BY, &dict)?,
            })
        } else {
            Err(ParsingError::TypeMismatch)
        }
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

fn required<'a, T>(key: &[u8], dict: &'a HashMap<&[u8], Node<'a>>) -> Result<T, ParsingError>
where
    T: TryFrom<Node<'a>, Error = ParsingError>,
{
    if let Some(node) = dict.get(key) {
        node.clone().try_into()
    } else {
        Err(ParsingError::MissingField(
            String::from_utf8(key.to_vec()).unwrap(),
        ))
    }
}

fn optional<'a, T>(
    key: &[u8],
    dict: &'a HashMap<&[u8], Node<'a>>,
) -> Result<Option<T>, ParsingError>
where
    T: TryFrom<Node<'a>, Error = ParsingError>,
{
    if let Some(node) = dict.get(key) {
        Ok(Some(node.clone().try_into()?))
    } else {
        Ok(None)
    }
}

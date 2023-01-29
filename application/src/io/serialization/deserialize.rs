use std::{collections::HashMap, vec};

use sha1::{Digest, Sha1};

use crate::{
    common_types::{
        data::Torrent,
        files::{File, Files, Info, MultipleFileMode, SingleFileMode, TorrentFile},
    },
    io::repo::{Id, TorrentRepo, WithId},
};

use super::{
    consts::{
        ANNOUNCE, ANNOUNCE_LIST, COMMENT, CREATED_BY, CREATION_DATE, DATA, ENCODING, FILES,
        HTTPSEEDS, ID, INFO, LENGTH, MD5SUM, NAME, PATH, PIECES, PIECE_LENGTH, PRIVATE, TORRENTS,
        VALUE,
    },
    node::Node,
};

use super::error::ParsingError;

impl<'a> TryFrom<Node<'a>> for u64 {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::UnsignedNum(i) = value {
            Ok(i as u64)
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for Vec<u8> {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::String(s) = value {
            Ok(s.to_vec())
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for String {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::String(s) = value {
            let s = String::from_utf8(s.to_vec());
            if let Ok(s) = s {
                Ok(s)
            } else {
                Err(ParsingError::InvalidFormat)
            }
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a, T> TryFrom<Node<'a>> for Vec<T>
where
    T: TryFrom<Node<'a>, Error = ParsingError>,
{
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::List(list) = value {
            let mut new_list: Vec<T> = vec![];
            for node in list {
                new_list.push(node.try_into()?);
            }
            Ok(new_list)
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

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
        if let Node::Dict(dict, raw) = value {
            let mut hasher = Sha1::new();
            hasher.update(raw);

            let files = {
                let single =
                    dict.contains_key(NAME as &[u8]) && dict.contains_key(b"length" as &[u8]);
                let multi =
                    dict.contains_key(NAME as &[u8]) && dict.contains_key(b"files" as &[u8]);

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
                hash: hasher.finalize().into(),
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
            Ok(Torrent {
                data: required(DATA, &dict)?,
            })
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a, T> TryFrom<Node<'a>> for WithId<T>
where
    T: TryFrom<Node<'a>, Error = ParsingError>,
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
            Ok(TorrentRepo::from(required(TORRENTS, &dict)?))
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

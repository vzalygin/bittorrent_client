use std::{collections::HashMap, vec};

use sha1::{Digest, Sha1};

use crate::common_types::files::{
    File, Files, Info, MultipleFileMode, SingleFileMode, TorrentFile,
};

use super::error::ParsingError;

/// Структура, которая размечает байты, передаваемые на парсинг.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
    Integer(i64),
    String(&'a [u8]),
    List(Vec<Node<'a>>),
    Dict(HashMap<&'a [u8], Node<'a>>, &'a [u8]), // Также храним кусок, в котором этот словарь размещён, чтобы взять хеш от инфо-словарика
}

impl<'a> TryFrom<Node<'a>> for i64 {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Integer(i) = value {
            Ok(i)
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for u64 {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Integer(i) = value {
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
                path: required(b"path", &dict)?,
                length: required(b"length", &dict)?,
                md5sum: optional(b"", &dict)?,
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
                    dict.contains_key(b"name" as &[u8]) && dict.contains_key(b"length" as &[u8]);
                let multi =
                    dict.contains_key(b"name" as &[u8]) && dict.contains_key(b"files" as &[u8]);

                if single && !multi {
                    Files::Single(SingleFileMode {
                        name: required(b"name", &dict)?,
                        length: required(b"length", &dict)?,
                        md5sum: optional(b"md5sum", &dict)?,
                    })
                } else if !single && multi {
                    Files::Multiple(MultipleFileMode {
                        base_name: required(b"name", &dict)?,
                        files: required(b"files", &dict)?,
                    })
                } else {
                    return Err(ParsingError::InvalidFormat);
                }
            };

            Ok(Info {
                piece_length: required(b"piece length", &dict)?,
                pieces: required(b"pieces", &dict)?,
                private: optional(b"private", &dict)?,
                files,
                hash: hasher.finalize().into(),
            })
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryInto<TorrentFile> for Node<'a> {
    type Error = ParsingError;

    fn try_into(self) -> Result<TorrentFile, Self::Error> {
        if let Node::Dict(dict, _) = self {
            Ok(TorrentFile {
                info: required(b"info", &dict)?,
                announce: required(b"announce", &dict)?,
                encoding: optional(b"encoding", &dict)?,
                httpseeds: optional(b"httpseeds", &dict)?,
                announce_list: optional(b"announce-list", &dict)?,
                creation_date: optional(b"creation date", &dict)?,
                comment: optional(b"comment", &dict)?,
                created_by: optional(b"created by", &dict)?,
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

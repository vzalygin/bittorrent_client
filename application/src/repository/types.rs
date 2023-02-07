use sha1::{Digest, Sha1};

use crate::io::{
    consts::*,
    deserialization::{parse_node, DataProvider, Node, ParsingError, TryDeserialize},
    serialization::{BencodeDictBuilder, Serialize},
};

#[derive(Debug, Clone, PartialEq)]
pub struct FileMetadata {
    pub path: Vec<String>,
    pub length: u64,
    pub md5sum: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SingleFileMode {
    pub name: String,
    pub length: u64,
    pub md5sum: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultipleFileMode {
    pub base_name: String,
    pub files: Vec<FileMetadata>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilesMetadata {
    Single(SingleFileMode),
    Multiple(MultipleFileMode),
}

impl Serialize for FileMetadata {
    fn serialize(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(PATH, self.path.clone())
            .required(LENGTH, self.length)
            .optional(MD5SUM, self.md5sum.clone())
            .fin()
    }
}

impl<'a> TryDeserialize<'a> for FileMetadata {
    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError> {
        let dp = DataProvider::try_from(node)?;
        Ok(FileMetadata {
            path: dp.required(PATH)?,
            length: dp.required(LENGTH)?,
            md5sum: dp.optional(MD5SUM)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    pub piece_length: u64,
    pub pieces: Vec<u8>,
    pub private: Option<u64>,
    pub files: FilesMetadata,
}

impl Serialize for Info {
    fn serialize(&self) -> Vec<u8> {
        match &self.files {
            FilesMetadata::Single(file) => BencodeDictBuilder::new()
                .required(NAME, file.name.clone())
                .required(LENGTH, file.length)
                .optional(MD5SUM, file.md5sum.clone()),
            FilesMetadata::Multiple(files) => BencodeDictBuilder::new()
                .required(NAME, files.base_name.clone())
                .required(FILES, files.files.clone()),
        }
        .required(PIECE_LENGTH, self.piece_length)
        .required(PIECES, self.pieces.clone())
        .optional(PRIVATE, self.private)
        .fin()
    }
}

impl<'a> TryDeserialize<'a> for Info {
    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError> {
        let dp = DataProvider::try_from(node)?;

        let files = {
            let single = dp.optional::<u64>(LENGTH)?.is_some();
            let multi = dp.optional::<Vec<FileMetadata>>(FILES)?.is_some();

            if single && !multi {
                FilesMetadata::Single(SingleFileMode {
                    name: dp.required(NAME)?,
                    length: dp.required(LENGTH)?,
                    md5sum: dp.optional(MD5SUM)?,
                })
            } else if !single && multi {
                FilesMetadata::Multiple(MultipleFileMode {
                    base_name: dp.required(NAME)?,
                    files: dp.required(FILES)?,
                })
            } else {
                return Err(ParsingError::InvalidFormat);
            }
        };

        Ok(Info {
            piece_length: dp.required(PIECE_LENGTH)?,
            pieces: dp.required(PIECES)?,
            private: dp.optional(PRIVATE)?,
            files,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TorrentMetadata {
    pub info: Info,
    pub announce: String,
    pub encoding: Option<String>,
    pub httpseeds: Option<Vec<String>>,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<u64>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
}

impl Serialize for TorrentMetadata {
    fn serialize(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(INFO, self.info.clone())
            .required(ANNOUNCE, self.announce.clone())
            .optional(ENCODING, self.encoding.clone())
            .optional(HTTPSEEDS, self.httpseeds.clone())
            .optional(ANNOUNCE_LIST, self.announce_list.clone())
            .optional(CREATION_DATE, self.creation_date.clone())
            .optional(COMMENT, self.comment.clone())
            .optional(CREATED_BY, self.created_by.clone())
            .fin()
    }
}

impl<'a> TryDeserialize<'a> for TorrentMetadata {
    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError> {
        let dp = DataProvider::try_from(node)?;
        Ok(TorrentMetadata {
            info: dp.required(INFO)?,
            announce: dp.required(ANNOUNCE)?,
            encoding: dp.optional(ENCODING)?,
            httpseeds: dp.optional(HTTPSEEDS)?,
            announce_list: dp.optional(ANNOUNCE_LIST)?,
            creation_date: dp.optional(CREATION_DATE)?,
            comment: dp.optional(COMMENT)?,
            created_by: dp.optional(CREATED_BY)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Torrent {
    pub data: TorrentMetadata,
    pub hash: [u8; 20],
}

impl Serialize for Torrent {
    fn serialize(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(DATA, self.data.clone())
            .required(HASH, self.hash.to_vec())
            .fin()
    }
}

impl<'a> TryDeserialize<'a> for Torrent {
    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError> {
        let dp = DataProvider::try_from(node)?;

        let hash: Vec<u8> = dp.required(HASH)?; // Копилятор без подсказки не смог в двойной вывод типов.
        Ok(Torrent {
            data: dp.required(DATA)?,
            hash: hash.try_into().unwrap(),
        })
    }

    fn try_deserialize(bytes: &'a [u8]) -> Result<Self, ParsingError> {
        let node = parse_node(bytes);

        if let Ok((_, node)) = node {
            Ok(Torrent {
                hash: get_info_hash(&node)?,
                data: TorrentMetadata::try_deserialize_from_node(node)?,
            })
        } else {
            Err(ParsingError::InvalidFormat)
        }
    }
}

fn get_info_hash(node: &Node) -> Result<[u8; 20], ParsingError> {
    if let Node::Dict(torrent, _) = node {
        if let Some(info) = torrent.get(INFO) {
            if let Node::Dict(_, raw) = info {
                let mut hasher = Sha1::new();
                hasher.update(raw);
                return Ok(hasher.finalize().into());
            }
        }
    }
    return Err(ParsingError::InvalidFormat);
}

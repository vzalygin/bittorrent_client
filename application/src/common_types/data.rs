use crate::io::{
    consts::*,
    deserialization::{optional, required, Node, ParsingError},
    serialization::{BencodeDictBuilder, SerializeTo},
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

impl SerializeTo<Vec<u8>> for FileMetadata {
    fn serialize(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(PATH, self.path.clone())
            .required(LENGTH, self.length)
            .optional(MD5SUM, self.md5sum.clone())
            .fin()
    }
}

impl<'a> TryFrom<Node<'a>> for FileMetadata {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            Ok(FileMetadata {
                path: required(PATH, &dict)?,
                length: required(LENGTH, &dict)?,
                md5sum: optional(MD5SUM, &dict)?,
            })
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    pub piece_length: u64,
    pub pieces: Vec<u8>,
    pub private: Option<u64>,
    pub files: FilesMetadata,
}

impl SerializeTo<Vec<u8>> for Info {
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

impl<'a> TryFrom<Node<'a>> for Info {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            let files = {
                let single = dict.contains_key(b"length" as &[u8]);
                let multi = dict.contains_key(b"files" as &[u8]);

                if single && !multi {
                    FilesMetadata::Single(SingleFileMode {
                        name: required(NAME, &dict)?,
                        length: required(LENGTH, &dict)?,
                        md5sum: optional(MD5SUM, &dict)?,
                    })
                } else if !single && multi {
                    FilesMetadata::Multiple(MultipleFileMode {
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

impl SerializeTo<Vec<u8>> for TorrentMetadata {
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

impl<'a> TryFrom<Node<'a>> for TorrentMetadata {
    type Error = ParsingError;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        if let Node::Dict(dict, _) = value {
            Ok(TorrentMetadata {
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

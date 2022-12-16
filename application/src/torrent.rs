mod raw_data;

use raw_data::{File, RawInfo, RawTorrent, Node};
use serde_bencode::Error;

#[derive(Debug)]
pub struct SingleFileMode {
    pub name: String,
    pub length: u64,
    pub md5sum: Option<String>,
}

#[derive(Debug)]
pub struct MultiplyFileMode {
    pub base_name: String,
    pub files: Vec<File>,
}

#[derive(Debug)]
pub enum Files {
    Single(SingleFileMode),
    Multiply(MultiplyFileMode),
}

#[derive(Debug)]
pub struct Info {
    pub piece_length: u64,
    pub pieces: Vec<u8>,
    pub private: Option<u8>,
    pub files: Files,
}

#[derive(Debug)]
pub struct Torrent {
    pub info: Info,
    pub announce: Option<String>,
    pub nodes: Option<Vec<Node>>,
    pub encoding: Option<String>,
    pub httpseeds: Option<Vec<String>>,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<u64>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
}

impl TryInto<Torrent> for RawTorrent {
    type Error = Error;

    fn try_into(self) -> Result<Torrent, Self::Error> {
        Ok(Torrent {
            info: self.info.try_into()?,
            announce: self.announce,
            encoding: self.encoding,
            httpseeds: self.httpseeds,
            announce_list: self.announce_list,
            creation_date: self.creation_date,
            comment: self.comment,
            created_by: self.created_by,
            nodes: self.nodes,
        })
    }
}

impl TryInto<Info> for RawInfo {
    type Error = Error;

    fn try_into(self) -> Result<Info, Self::Error> {
        let is_single = self.length.is_some() && self.length.is_some();
        let is_multy = self.files.is_some();

        if is_single == is_multy {
            Err(Error::UnknownVariant(
                "Parsing error: info have signs of multy and sigle file both".to_string(),
            ))
        } else {
            let files = if is_single {
                Files::Single(SingleFileMode {
                    name: self.name,
                    length: self.length.unwrap(),
                    md5sum: self.md5sum,
                })
            } else {
                Files::Multiply(MultiplyFileMode {
                    base_name: self.name,
                    files: self.files.unwrap(),
                })
            };

            Ok(Info {
                pieces: self.pieces,
                piece_length: self.piece_length,
                private: self.private,
                files,
            })
        }
    }
}

pub fn parse_torrent_from_bytes(input: &[u8]) -> Result<Torrent, Error> {
    serde_bencode::de::from_bytes::<RawTorrent>(input)?.try_into()
}

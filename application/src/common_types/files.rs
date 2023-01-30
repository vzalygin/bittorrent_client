#[derive(Debug, Clone, PartialEq)]
pub struct File {
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
    pub files: Vec<File>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Files {
    Single(SingleFileMode),
    Multiple(MultipleFileMode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    pub piece_length: u64,
    pub pieces: Vec<u8>,
    pub private: Option<u64>,
    pub files: Files,
    pub hash: [u8; 20],
}

#[derive(Debug, Clone, PartialEq)]
pub struct TorrentFile {
    pub info: Info,
    pub announce: String,
    pub encoding: Option<String>,
    pub httpseeds: Option<Vec<String>>,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<u64>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
}

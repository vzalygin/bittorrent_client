#[derive(Debug)]
pub struct Node(String, u64);

pub struct File {
    pub path: Vec<String>,
    pub length: u64,
    pub md5sum: Option<String>,
}

pub struct SingleFileMode {
    pub name: String,
    pub length: u64,
    pub md5sum: Option<String>,
}

pub struct MultiplyFileMode {
    pub base_name: String,
    pub files: Vec<File>,
}

pub enum Files {
    Single(SingleFileMode),
    Multiply(MultiplyFileMode),
}

pub struct Info {
    pub piece_length: u64,
    pub pieces: Vec<u8>,
    pub private: Option<u8>,
    pub files: Files,
    pub hash: [u8; 20],
}

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

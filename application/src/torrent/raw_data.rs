use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Node(String, u64);

#[derive(Debug, Deserialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: u64,
    #[serde(default)]
    pub md5sum: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawInfo {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<u64>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawTorrent {
    pub info: RawInfo,
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(default)]
    pub nodes: Option<Vec<Node>>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<u64>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}
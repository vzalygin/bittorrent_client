use serde_derive::Deserialize;

// Не нашёл по спецификации информацию об этом
// #[derive(Debug, Deserialize)]
// pub struct Node(String, u64);

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
    pub pieces: String,
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
    // Не нашёл по спецификации, что это такое
    // #[serde(default)]
    // pub nodes: Option<Vec<Node>>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}
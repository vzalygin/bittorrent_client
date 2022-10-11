use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
struct Torrent {
    info: RowInfo,
    announce: String,
    announce_list: Option<Vec<Vec<String>>>,
    creation_date: Option<u64>,
    comment: Option<String>,
    created_by: Option<String>,
    encoding: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RowInfo {
    piece_length: u64,
    pieces: String,
    private: Option<u8>, // must be 0 or 1 or None
    name: String,
    length: Option<u64>,
    md5sum: Option<String>,
    files: Option<Vec<File>>,
}

#[derive(Debug, Deserialize)]
struct File {
    length: u64,
    md5sum: Option<String>,
    path: Vec<String>,
}
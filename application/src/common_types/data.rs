use super::files::TorrentFile;

#[derive(Debug, Clone, PartialEq)]
pub struct Torrent {
    pub data: TorrentFile,
    pub hash: [u8; 20],
}

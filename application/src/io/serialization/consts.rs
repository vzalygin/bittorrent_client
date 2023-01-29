/// The constats for parsing. Don't change to maintain backward compability.

// Spec constants
pub const PATH: &[u8; 4] = b"path";
pub const LENGTH: &[u8; 6] = b"length";
pub const MD5SUM: &[u8; 6] = b"md5sum";
pub const NAME: &[u8; 4] = b"name";
pub const FILES: &[u8; 5] = b"files";
pub const PIECE_LENGTH: &[u8; 12] = b"piece length";
pub const PIECES: &[u8; 6] = b"pieces";
pub const PRIVATE: &[u8; 7] = b"private";
pub const INFO: &[u8; 4] = b"info";
pub const ANNOUNCE: &[u8; 8] = b"announce";
pub const ENCODING: &[u8; 8] = b"encoding";
pub const HTTPSEEDS: &[u8; 9] = b"httpseeds";
pub const ANNOUNCE_LIST: &[u8; 13] = b"announce-list";
pub const CREATION_DATE: &[u8; 13] = b"creation date";
pub const COMMENT: &[u8; 7] = b"comment";
pub const CREATED_BY: &[u8; 10] = b"created by";

// Private constants
pub const DATA: &[u8; 4] = b"data";
pub const VALUE: &[u8; 5] = b"value";
pub const ID: &[u8; 2] = b"id";
pub const TORRENTS: &[u8; 8] = b"torrents";

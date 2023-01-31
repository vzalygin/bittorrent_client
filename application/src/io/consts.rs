/// The constats for parsing. Don't change to maintain backward compability.

// Specification constants
pub const PATH: &[u8] = b"path";
pub const LENGTH: &[u8] = b"length";
pub const MD5SUM: &[u8] = b"md5sum";
pub const NAME: &[u8] = b"name";
pub const FILES: &[u8] = b"files";
pub const PIECE_LENGTH: &[u8] = b"piece length";
pub const PIECES: &[u8] = b"pieces";
pub const PRIVATE: &[u8] = b"private";
pub const INFO: &[u8] = b"info";
pub const ANNOUNCE: &[u8] = b"announce";
pub const ENCODING: &[u8] = b"encoding";
pub const HTTPSEEDS: &[u8] = b"httpseeds";
pub const ANNOUNCE_LIST: &[u8] = b"announce-list";
pub const CREATION_DATE: &[u8] = b"creation date";
pub const COMMENT: &[u8] = b"comment";
pub const CREATED_BY: &[u8] = b"created by";

// Repo constants
pub const DATA: &[u8] = b"data";
pub const VALUE: &[u8] = b"value";
pub const ID: &[u8] = b"id";
pub const TORRENTS: &[u8] = b"torrents";
pub const HASH: &[u8] = b"hash";

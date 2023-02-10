use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::{Client, Response};

use crate::{repository::types::Torrent, client::{PEER_ID, PORT, COMPACT}};

pub struct TorrentState {
    pub torrent: Torrent,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
}

pub async fn get_start(client: Client, state: TorrentState) -> Response{
    let hash = STANDARD.encode(&state.torrent.hash[..]);
    let url = state.torrent.metadata.announce;
    client
        .get(url)
        .header("peer_id", &PEER_ID[..])
        .header("port", PORT)
        .header("compact", COMPACT as u16) 
        .header("info_hash", hash) // тут проблема
        .header("uploaded", state.uploaded)
        .header("downloaded", state.downloaded)
        .header("left", state.left)
        .header("event", "started")
        .send().await.unwrap()
}

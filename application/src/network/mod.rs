use reqwest::{Client, Response};

use crate::{repository::types::Torrent, client::{PEER_ID, PORT, COMPACT}};

pub struct TorrentState {
    pub torrent: Torrent,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
}

pub async fn get_start(client: Client, state: TorrentState) -> Response{
    let url = state.torrent.metadata.announce;
    let r = client
        .get(url)
        .header("peer_id", &PEER_ID[..])
        .header("port", PORT)
        .header("compact", COMPACT as u16) 
        .header("info_hash", &state.torrent.hash[..]) // тут проблема
        .header("uploaded", state.uploaded)
        .header("downloaded", state.downloaded)
        .header("left", state.left)
        .header("event", "started")
        .send().await;
    
    match r {
        Ok(r) => r,
        Err(e) => {
            panic!("jopa")
        }, 
    }
}

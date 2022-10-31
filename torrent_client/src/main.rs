use std::fs;
use torrent_lib::torrent::{parse_torrent_from_bytes, Torrent};

fn render_torrent(torrent: &Torrent) {
    println!("p:\t\t{}", torrent.info.piece_length);
}

fn main() {
    let r = fs::read("D:\\repos\\torrent_client\\doom-eternal.torrent");

    match r {
        Ok(bytes) => {
            let torrent = parse_torrent_from_bytes(&bytes).unwrap();
            render_torrent(&torrent);
        }
        Err(e) => println!("{}", e),
    }
}


use std::fs;
use torrent_lib::torrent::{parse_torrent_from_bytes, Torrent, Files};

fn render_torrent(torrent: &Torrent) {
    println!("announce:\t{:?}", torrent.announce);
    println!("nodes:\t\t{:?}", torrent.nodes);
    if let Some(al) = &torrent.announce_list {
        for a in al {
            println!("announce list:\t{}", a[0]);
        }
    }
    println!("httpseeds:\t{:?}", torrent.httpseeds);
    println!("creation date:\t{:?}", torrent.creation_date);
    println!("comment:\t{:?}", torrent.comment);
    println!("created by:\t{:?}", torrent.created_by);
    println!("encoding:\t{:?}", torrent.encoding);
    println!("piece length:\t{:?}", torrent.info.piece_length);
    println!("private:\t{:?}", torrent.info.private);
    if let Files::Multiply(e) = &torrent.info.files {
        println!("file base:\t{:?}", e.base_name);
        for f in &e.files {
            println!("file path:\t{:?}", f.path);
            println!("file length:\t{}", f.length);
            println!("file md5sum:\t{:?}", f.md5sum);
        }
    } else if let Files::Single(e) = &torrent.info.files {
        println!("file path:\t{:?}", e.name);
        println!("file length:\t{}", e.length);
        println!("file md5sum:\t{:?}", e.md5sum);
    }
}

fn main() {
    let r = fs::read("D:\\repos\\torrent_client\\doom-eternal.torrent");

    match r {
        Ok(bytes) => {
            match parse_torrent_from_bytes(&bytes) {
                Ok(t) => render_torrent(&t),
                Err(e) => println!("Parsing err: {}", e.to_string())
            }
        }
        Err(e) => println!("Read err: {}", e),
    }
}
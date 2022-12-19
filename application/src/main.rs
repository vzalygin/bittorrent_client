mod common_types;
mod parser;

use common_types::{
    error::AsyncErr,
    files::{Files, Torrent},
};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), AsyncErr> {
    let path = "./1.torrent";

    let mut f = File::open(path).await?;
    let mut buf: Vec<u8> = vec![];

    f.read_to_end(&mut buf).await?;

    let torrent: Torrent = buf[..].try_into()?;
    render_torrent(&torrent);

    Ok(())
}

fn render_torrent(torrent: &Torrent) {
    println!("announce:\t{:?}", torrent.announce);
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
    println!("hash:\t{:?}", torrent.info.hash);
    if let Files::Multiple(e) = &torrent.info.files {
        println!("file base:\t{:?}", e.base_name);
        // for f in &e.files {
        //     println!("file path:\t{:?}", f.path);
        //     println!("file length:\t{}", f.length);
        //     println!("file md5sum:\t{:?}", f.md5sum);
        // }
    } else if let Files::Single(e) = &torrent.info.files {
        println!("file path:\t{:?}", e.name);
        println!("file length:\t{}", e.length);
        println!("file md5sum:\t{:?}", e.md5sum);
    }
}

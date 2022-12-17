mod common_types;
mod parser;

use parser::parse_from_bytes;
use common_types::files::{Torrent, Files};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

type AsyncErr = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), AsyncErr> {
    let path = "./1.torrent";

    let mut f = File::open(path).await?;
    let mut buf: Vec<u8> = vec![];

    f.read_to_end(&mut buf).await?;
    
    let torrent = parse_from_bytes(&buf[..]);
    if let Ok(torrent) = torrent {
        render_torrent(&torrent);
    } else {
        println!("something went wrong");
    }
    

    Ok(())
}

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

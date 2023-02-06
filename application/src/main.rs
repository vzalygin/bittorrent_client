mod error;
mod io;
mod repository;

use tokio::io::AsyncReadExt;
use tokio::{fs::File, io::AsyncWriteExt};

use error::AsyncErr;
use io::{deserialization::TryDeserialize, serialization::SerializeTo};
use repository::data::{FilesMetadata, Torrent};

#[tokio::main]
async fn main() -> Result<(), AsyncErr> {
    let path = "./1.torrent";

    let mut f = File::open(path).await?;
    let mut buf: Vec<u8> = vec![];

    f.read_to_end(&mut buf).await?;

    let torrent = Torrent::try_deserialize(&buf[..])?;
    render_torrent(&torrent);

    let e = torrent.serialize();
    let mut f1 = File::create("./temp.torrent").await?;
    f1.write(&e[..]).await?;

    Ok(())
}

fn render_torrent(torrent: &Torrent) {
    let torrent = &torrent.data;
    if let FilesMetadata::Multiple(e) = &torrent.info.files {
        println!("file base:\t{:?}", e.base_name);
        for f in &e.files {
            println!("file path:\t{:?}", f.path);
            println!("file length:\t{}", f.length);
            println!("file md5sum:\t{:?}", f.md5sum);
        }
    } else if let FilesMetadata::Single(e) = &torrent.info.files {
        println!("file path:\t{:?}", e.name);
        println!("file length:\t{}", e.length);
        println!("file md5sum:\t{:?}", e.md5sum);
    }
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
}

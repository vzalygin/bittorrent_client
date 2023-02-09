use uuid::Uuid;

use crate::{
    io::{deserialization::TryDeserialize, serialization::Serialize},
    repository::{
        types::{FileMetadata, FilesMetadata, Info, SingleFileMode, Torrent, TorrentMetadata},
        TorrentRepo, WithId,
    },
};

#[test]
fn serialize_str() {
    let data: &[u8] = b"4:spam";

    let str = String::try_deserialize(data).unwrap();
    let new = &str.serialize()[..];

    assert_eq!(data, new);
}

#[test]
fn serialize_unsigned() {
    let data: &[u8] = b"i42e";

    let num = u64::try_deserialize(data).unwrap();
    let new = &num.serialize();

    assert_eq!(data, new);
}

#[test]
fn serialize_bytes() {
    let data: &[u8] = b"4:abab";

    let bytes = Vec::<u8>::try_deserialize(data).unwrap();
    let new = &bytes.serialize();

    assert_eq!(data, new);
}

#[test]
fn serialize_list() {
    let data: &[u8] = b"li24ei42ee";

    let list = Vec::<u64>::try_deserialize(data).unwrap();
    let new = &list.serialize();

    assert_eq!(data, new);
}

#[test]
fn serialize_file() {
    let data: &[u8] = b"d4:pathl5:abobae6:lengthi42ee";

    let file = FileMetadata::try_deserialize(data).unwrap();
    let new = &file.serialize();

    assert_eq!(data, new);
}

// #[test]
// fn serialize_info() {
//     let data: &[u8] = b"";
// }

fn generate_repo_object() -> TorrentRepo {
    TorrentRepo {
        torrents: vec![WithId {
            id: Uuid::new_v4(),
            value: Torrent {
                metadata: TorrentMetadata {
                    info: Info {
                        piece_length: 256,
                        pieces: "QWERTYILKNAWKJN".to_string().as_bytes().to_vec(),
                        private: Some(1),
                        files: FilesMetadata::Single(SingleFileMode {
                            name: "1".to_string(),
                            length: 16,
                            md5sum: None,
                        }),
                    },
                    announce: "TEST".to_string(),
                    encoding: None,
                    httpseeds: None,
                    announce_list: Some(vec![vec!["TEST1".to_string(), "TEST2".to_string()]]),
                    creation_date: Some(123),
                    comment: Some("FOOBAR".to_string()),
                    created_by: Some("Zalygin".to_string()),
                },
                hash: *b"12345678901234567890",
                downloaded_pieces: vec![6u8, 4u8, 5u8],
                downloaded: 0
            },
        }],
    }
}

#[test]
fn serialize_repo() {
    let repo = generate_repo_object();

    let bytes = &repo.serialize()[..];
    let new_repo = TorrentRepo::try_deserialize(bytes);

    if let Err(e) = &new_repo {
        println!("{:?}", e);
    }
    assert!(new_repo.is_ok());
    let new_repo = new_repo.unwrap();
    assert_eq!(repo, new_repo);
}

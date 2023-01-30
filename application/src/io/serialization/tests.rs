use uuid::Uuid;

use crate::{common_types::{data::Torrent, files::{TorrentFile, Info, Files, SingleFileMode}}, io::repo::{TorrentRepo, WithId}};

use super::{node::Node, parsing::parse_node, serialize::SerializeTo, error::ParsingError};

#[test]
fn parse_a_pos_num() {
    let inp = b"i42e";

    let res = parse_node(inp);

    assert!(res.is_ok());
    if let (_, Node::UnsignedNum(num)) = res.unwrap() {
        assert_eq!(42, num);
    } else {
        assert!(false)
    }
}

// #[test]
// fn parse_a_neg_num() {
//     let inp = b"i-42e";

//     let res = parse_node(inp);

//     assert!(res.is_ok());
//     if let (_, Node::Integer(num)) = res.unwrap() {
//         assert_eq!(-42, num);
//     } else {
//         assert!(false)
//     }
// }

#[test]
fn parse_not_a_num() {
    let inp = b"ie";

    let res = parse_node(inp);

    assert!(res.is_err())
}

#[test]
fn parse_str() {
    let inp = b"4:spami3e";

    let res = parse_node(inp);

    assert!(res.is_ok());
    let (next, s) = res.unwrap();

    if let Node::String(s) = s {
        assert_eq!(b"spam", s);
    } else {
        assert!(false)
    }
    assert_eq!(b"i3e", next);
}

#[test]
fn parse_empty_str() {
    let inp = b"0:lol";

    let res = parse_node(inp);

    assert!(res.is_ok());
    let (next, s) = res.unwrap();
    if let Node::String(s) = s {
        assert_eq!(b"", s);
    } else {
        assert!(false)
    }
    assert_eq!(b"lol", next);
}

#[test]
fn parse_list() {
    let inp = b"l4:spami42eelol";

    let res = parse_node(inp);

    assert!(res.is_ok());
    let (next, list) = res.unwrap();
    if let Node::List(list) = list {
        assert_eq!(Node::String(b"spam"), list[0]);
        assert_eq!(Node::UnsignedNum(42), list[1]);
    } else {
        assert!(false)
    }
    assert_eq!(b"lol", next)
}

#[test]
fn parse_empty_list() {
    let inp = b"lelol";

    let res = parse_node(inp);

    assert!(res.is_ok());
    let (next, list) = res.unwrap();
    if let Node::List(list) = list {
        assert_eq!(0, list.len())
    } else {
        assert!(false)
    }
    assert_eq!(b"lol", next);
}

#[test]
fn parse_dict() {
    let inp = b"d4:spami42e5:hello3:lolelol";

    let res = parse_node(inp);

    assert!(res.is_ok());
    let (next, dict) = res.unwrap();
    if let Node::Dict(dict, raw) = dict {
        assert_eq!(b"d4:spami42e5:hello3:lole", raw);
        assert_eq!(2, dict.len());
        assert_eq!(Node::UnsignedNum(42), dict[b"spam" as &[u8]]);
        assert_eq!(Node::String(b"lol"), dict[b"hello" as &[u8]]);
    } else {
        assert!(false)
    }
    assert_eq!(b"lol", next);
}

#[test]
fn parse_empty_dict() {
    let inp = b"delol";

    let res = parse_node(inp);

    assert!(res.is_ok());
    let (next, dict) = res.unwrap();
    if let Node::Dict(dict, raw) = dict {
        assert_eq!(b"de", raw);
        assert_eq!(0, dict.len());
    } else {
        assert!(false)
    }
    assert_eq!(b"lol", next);
}

fn generate_repo_object() -> TorrentRepo{
    TorrentRepo::from(vec![WithId { 
            id: Uuid::new_v4(), 
            value: Torrent {
                data: TorrentFile {
                    info: Info {
                        piece_length: 256,
                        pieces: "QWERTYILKNAWKJN".to_string().as_bytes().to_vec(),
                        private: Some(1),
                        files: Files::Single(SingleFileMode { 
                            name: "1".to_string(), 
                            length: 16, 
                            md5sum: None, 
                        }),
                        hash: *b"12345678901234567890",
                    },
                    announce: "TEST".to_string(),
                    encoding: None,
                    httpseeds: None,
                    announce_list: Some(vec![vec!["TEST1".to_string(), "TEST2".to_string()]]),
                    creation_date: Some(123),
                    comment: Some("FOOBAR".to_string()),
                    created_by: Some("Zalygin".to_string()),
                },
            }, 
        }]
    )
}

#[test]
fn serialize_single_file_torrent() {
    let repo = generate_repo_object();

    let bytes = &repo.serialize_to()[..];
    let new_repo = TorrentRepo::try_from(bytes);

    if let Err(e) = &new_repo {
        println!("{:?}", e);
    }
    assert!(new_repo.is_ok());
    let new_repo = new_repo.unwrap();
    assert_eq!(repo, new_repo);
}
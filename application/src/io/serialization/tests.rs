use super::{node::Node, parsing::parse_node};

#[test]
fn parse_a_pos_num() {
    let inp = b"i42e";

    let res = parse_node(inp);

    assert!(res.is_ok());
    if let (_, Node::Integer(num)) = res.unwrap() {
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
        assert_eq!(Node::Integer(42), list[1]);
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
        assert_eq!(Node::Integer(42), dict[b"spam" as &[u8]]);
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

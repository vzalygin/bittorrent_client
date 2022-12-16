use std::collections::HashMap;

use nom::{
    bytes::complete::{take, take_while},
    character::{complete::char, is_digit},
    combinator::map_res,
    error::{Error as Err, ErrorKind},
    sequence::{preceded, tuple, delimited},
    Err::Error,
    IResult, branch::alt, multi::many0,
};

enum Node {
    Integer(i64),
    String(String), 
    List(Vec<Node>),
    Dict(HashMap<String, Node>, Vec<u8>) // Нужно ссылаться на кусок байтов, чтобы потом можно было взять от них хеш
}

fn parse_digits(inp: &[u8]) -> IResult<&[u8], u32> {
    let (inp, r) = take_while(is_digit)(inp)?;

    if r.len() != 0 {
        let mut digits = 0;
        for b in r {
            digits *= 10;
            digits += (b - b'0') as u32;
        }

        Ok((inp, digits))
    } else {
        Err(Error(Err {
            input: inp,
            code: ErrorKind::TakeWhile1,
        }))
    }
}

fn parse_minus(inp: &[u8]) -> IResult<&[u8], bool> {
    // Достаточно тупо, но работает...
    let (inp, r) = take_while(|c| c == b'-')(inp)?;
    Ok((inp, r.len() == 1))
}

fn parse_number(inp: &[u8]) -> IResult<&[u8], Node> {
    let (inp, (_, minus, r, _)) = tuple((char('i'), parse_minus, parse_digits, char('e')))(inp)?;
    let number = if minus { -(r as i64) } else { r as i64 };
    Ok((inp, Node::Integer(number)))
}

fn parse_string(inp: &[u8]) -> IResult<&[u8], Node> {
    let (inp, length) = parse_digits(inp)?;

    let (inp, s) = map_res(
        preceded(char(':'), take(length)), 
        |s: &[u8]| String::from_utf8(s.to_vec())
    )(inp)?;

    Ok((inp, Node::String(s)))
}

fn parse_list(inp: &[u8]) -> IResult<&[u8], Node> {
    let parse_nodes = alt((parse_string, parse_number, parse_list)); 

    map_res(
        delimited(char('l'), many0(parse_nodes), char('e')),
        |list| { Result::<Node, ()>::Ok(Node::List(list)) }
    )(inp) 
}

fn parse_dict(inp: &[u8]) -> IResult<&[u8], Node> {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_pos_num() {
        let inp = b"i42e";

        let res = parse_number(inp);

        assert!(res.is_ok());
        if let (_, Node::Integer(num)) = res.unwrap() {
            assert_eq!(42, num);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_a_neg_num() {
        let inp = b"i-42e";

        let res = parse_number(inp);

        assert!(res.is_ok());
        if let (_, Node::Integer(num)) = res.unwrap() {
            assert_eq!(-42, num);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_not_a_num() {
        let inp = b"ie";

        let res = parse_number(inp);

        assert!(res.is_err())
    }

    #[test]
    fn parse_str() {
        let inp = b"4:spami3e";

        let res = parse_string(inp);

        assert!(res.is_ok());
        let (next, s) = res.unwrap();
        
        if let Node::String(s) = s {
            assert_eq!("spam", s);
        }
        assert_eq!(b"i3e", next);
    }

    #[test]
    fn parse_empty_str() {
        let inp = b"0:lol";

        let res = parse_string(inp);

        assert!(res.is_ok());
        let (next, s) = res.unwrap();
        if let Node::String(s) = s {
            assert_eq!("", s);
        }
        assert_eq!(b"lol", next);
    }
}

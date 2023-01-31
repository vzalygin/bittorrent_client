/// Модуль с парсерами.
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{take, take_while},
    character::{complete::char, is_digit},
    combinator::map_res,
    error::{Error as Err, ErrorKind},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    Err::Error,
    IResult,
};

/// Структура, которая размечает байты, передаваемые на парсинг.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
    UnsignedNum(u64),
    String(&'a [u8]),
    List(Vec<Node<'a>>),
    Dict(HashMap<&'a [u8], Node<'a>>, &'a [u8]), // Также храним кусок, в котором этот словарь размещён, чтобы взять хеш от инфо-словарика
}

#[inline(always)]
pub fn parse_node(inp: &[u8]) -> IResult<&[u8], Node> {
    alt((parse_string, parse_number, parse_list, parse_dict))(inp)
}

fn parse_digits(inp: &[u8]) -> IResult<&[u8], u64> {
    let (inp, r) = take_while(is_digit)(inp)?;

    if r.len() != 0 {
        let mut digits = 0;
        for b in r {
            digits *= 10;
            digits += (b - b'0') as u64;
        }

        Ok((inp, digits))
    } else {
        Err(Error(Err {
            input: inp,
            code: ErrorKind::TakeWhile1,
        }))
    }
}

fn parse_number(inp: &[u8]) -> IResult<&[u8], Node> {
    let (inp, (_, number, _)) = tuple((char('i'), parse_digits, char('e')))(inp)?;

    Ok((inp, Node::UnsignedNum(number)))
}

fn parse_string(inp: &[u8]) -> IResult<&[u8], Node> {
    let (inp, length) = parse_digits(inp)?;

    let (inp, s) = preceded(char(':'), take(length))(inp)?;

    Ok((inp, Node::String(s)))
}

fn parse_list(inp: &[u8]) -> IResult<&[u8], Node> {
    map_res(delimited(char('l'), many0(parse_node), char('e')), |list| {
        Result::<Node, ()>::Ok(Node::List(list))
    })(inp)
}

fn parse_dict<'a>(inp: &'a [u8]) -> IResult<&[u8], Node> {
    let pairs_to_dict =
        |pairs: Vec<(Node<'a>, Node<'a>)>| -> Result<HashMap<&'a [u8], Node<'a>>, ()> {
            let mut dict = HashMap::new();
            for (key, value) in pairs {
                if let Node::String(s) = key {
                    dict.insert(s, value);
                } else {
                    return Result::Err(());
                }
            }
            Result::Ok(dict)
        };

    let parse_pair = pair(parse_string, parse_node);

    let (new_inp, dict) = map_res(
        delimited(char('d'), many0(parse_pair), char('e')),
        pairs_to_dict,
    )(inp)?;

    Ok((
        new_inp,
        Node::Dict(dict, &inp[0..(inp.len() - new_inp.len())]),
    ))
}

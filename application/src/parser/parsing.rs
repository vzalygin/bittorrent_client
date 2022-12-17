use nom::{
    Err::Error,
    error::{Error as Err, ErrorKind},
    bytes::complete::take_while,
    character::{complete::char, is_digit},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Node {
    Integer(i32),
}

fn parse_digits(inp: &[u8]) -> IResult<&[u8], i32> {
    let (inp, r) = take_while(is_digit)(inp)?;

    if r.len() != 0 {
        let mut digits = 0;
        for b in r {
            digits *= 10;
            digits += (b - b'0') as i32;
        }

        Ok((inp, digits))
    } else {
        Err(Error(Err { input: inp, code: ErrorKind::TakeWhile1 }))
    }
}

fn parse_minus(inp: &[u8]) -> IResult<&[u8], bool> {
    let (inp, r) = take_while(|c| c == b'-')(inp)?;
    Ok((inp, r.len() == 1))
}

fn parse_number(inp: &[u8]) -> IResult<&[u8], i32> {
    let (inp, (_, minus, r, _)) = tuple((char('i'), parse_minus, parse_digits, char('e')))(inp)?;
    let number = if minus { -r } else { r };
    Ok((inp, number))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_a_pos_num() {
        let inp = b"i42e";

        let res = super::parse_number(inp);

        assert!(res.is_ok());
        assert_eq!(42, res.unwrap().1);
    }

    #[test]
    fn parse_a_neg_num() {
        let inp = b"i-42e";

        let res = super::parse_number(inp);

        assert!(res.is_ok());
        assert_eq!(-42, res.unwrap().1);
    }

    #[test]
    fn parse_not_a_num() {
        let inp = b"ie";

        let res = super::parse_number(inp);

        assert!(res.is_err())
    }
}

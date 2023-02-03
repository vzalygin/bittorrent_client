use std::collections::HashMap;

use super::error::ParsingError;

/// Структура, которая размечает байты, передаваемые на парсинг.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
    UnsignedNum(u64),
    String(&'a [u8]),
    List(Vec<Node<'a>>),
    Dict(HashMap<&'a [u8], Node<'a>>, &'a [u8]), // Также храним кусок, в котором этот словарь размещён, чтобы взять хеш от инфо-словарика
}

pub fn required<'a, T>(key: &[u8], dict: &'a HashMap<&[u8], Node<'a>>) -> Result<T, ParsingError>
where
    T: TryFrom<Node<'a>, Error = ParsingError>,
{
    if let Some(node) = dict.get(key) {
        node.clone().try_into()
    } else {
        Err(ParsingError::MissingField(
            String::from_utf8(key.to_vec()).unwrap(),
        ))
    }
}

pub fn optional<'a, T>(
    key: &[u8],
    dict: &'a HashMap<&[u8], Node<'a>>,
) -> Result<Option<T>, ParsingError>
where
    T: TryFrom<Node<'a>, Error = ParsingError>,
{
    if let Some(node) = dict.get(key) {
        Ok(Some(node.clone().try_into()?))
    } else {
        Ok(None)
    }
}

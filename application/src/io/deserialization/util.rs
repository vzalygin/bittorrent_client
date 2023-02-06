use std::collections::HashMap;

use super::{error::ParsingError, parse_node};

/// Структура, которая размечает байты, передаваемые на парсинг.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
    UnsignedNum(u64),
    String(&'a [u8]),
    List(Vec<Node<'a>>),
    Dict(HashMap<&'a [u8], Node<'a>>, &'a [u8]), // Также храним кусок, в котором этот словарь размещён, чтобы взять хеш от инфо-словарика
}

pub trait TryDeserialize<'a>
where
    Self: Sized,
{
    fn try_deserialize(bytes: &'a [u8]) -> Result<Self, ParsingError> {
        match parse_node(bytes) {
            Ok((_, node)) => self::TryDeserialize::try_deserialize_from_node(node),
            Err(_) => Err(ParsingError::TypeMismatch),
        }
    }

    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError>;
}

#[inline]
pub fn required<'a, T>(key: &[u8], dict: &'a HashMap<&[u8], Node<'a>>) -> Result<T, ParsingError>
where
    T: TryDeserialize<'a>,
{
    if let Some(node) = dict.get(key) {
        T::try_deserialize_from_node(node.clone())
    } else {
        Err(ParsingError::MissingField(
            String::from_utf8(key.to_vec()).unwrap(),
        ))
    }
}

#[inline]
pub fn optional<'a, T>(
    key: &[u8],
    dict: &'a HashMap<&[u8], Node<'a>>,
) -> Result<Option<T>, ParsingError>
where
    T: TryDeserialize<'a>,
{
    if let Some(node) = dict.get(key) {
        Ok(Some(T::try_deserialize_from_node(node.clone())?))
    } else {
        Ok(None)
    }
}

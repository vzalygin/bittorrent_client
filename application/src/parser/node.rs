use std::{collections::HashMap, vec};

use crate::common_types::files::Torrent;

use super::error::ParsingError;

/// Структура, которая размечает байты, передаваемые на парсинг.
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Integer(i64),
    String(&'a [u8]),
    List(Vec<Node<'a>>),
    Dict(HashMap<&'a [u8], Node<'a>>, &'a [u8]), // Также храним кусок, в котором этот словарь размещён, чтобы взять хеш от инфо-словарика
}

impl<'a> TryInto<i64> for Node<'a> {
    type Error = ParsingError;

    fn try_into(self) -> Result<i64, Self::Error> {
        if let Node::Integer(i) = self {
            Ok(i)
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryInto<String> for Node<'a> {
    type Error = ParsingError;

    fn try_into(self) -> Result<String, Self::Error> {
        if let Node::String(s) = self {
            let s = String::from_utf8(s.to_vec());
            if let Ok(s) = s {
                Ok(s)
            } else {
                Err(ParsingError::InvalidFormat)
            }
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a, T> TryInto<Vec<T>> for Node<'a>
where
    T: TryFrom<Node<'a>, Error = ParsingError>,
{
    type Error = ParsingError;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        if let Node::List(list) = self {
            let mut new_list: Vec<T> = vec![];
            for node in list {
                new_list.push(node.try_into()?);
            }
            Ok(new_list)
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryInto<Torrent> for Node<'a> {
    type Error = ParsingError;

    fn try_into(self) -> Result<Torrent, Self::Error> {
        if let Node::Dict(dict, _) = self {
            // Ok(Torrent {
            //     info: (),
            //     announce: (),
            //     nodes: (),
            //     encoding: (),
            //     httpseeds: (),
            //     announce_list: (),
            //     creation_date: (),
            //     comment: (),
            //     created_by: (),
            // })
            unimplemented!()
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

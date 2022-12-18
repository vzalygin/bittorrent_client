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

impl<'a, Element: TryFrom<Node<'a>>> TryInto<Vec<Element>> for Node<'a> {
    type Error = ParsingError;

    fn try_into(self) -> Result<Vec<Element>, Self::Error> {
        if let Node::List(list) = self {
            let new_list: Vec<Element> = vec![];
            for node in list {
                let r = Element::try_from(node);
            }
            Ok(new_list)
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

// impl<'a> TryFrom<Node<'a>> for i64 {
//     type Error = ParsingError;

//     fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
//         if let Node::Integer(i) = value {
//             Ok(i)
//         } else {
//             Err(ParsingError::TypeMismatch)
//         }
//     }
// }

// impl<'a> TryInto<Torrent> for Node<'a> {
//     type Error = ParsingError;

//     fn try_into(self) -> Result<Torrent, Self::Error> {
//         if let Node::Dict(dict, _) = self {
//             Ok(Torrent {
//                 announce: get(dict, b"announce")?.try_into(),
//                 info: todo!(),
//                 nodes: todo!(),
//                 encoding: todo!(),
//                 httpseeds: todo!(),
//                 announce_list: todo!(),
//                 creation_date: todo!(),
//                 comment: todo!(),
//                 created_by: todo!(),
//             })
//         } else {
//             Err(ParsingError::TypeMismatch)
//         }
//     }
// }

// /// Возвращает
// fn get<'a>(dict: HashMap<&'a [u8], Node<'a>>, key: &'a [u8]) -> Result<Node<'a>, ParsingError> {
//     if let Some()
// }

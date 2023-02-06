/// Модуль с имплементациями для примитивных типов.
use super::{
    error::ParsingError,
    util::{Node, TryDeserialize},
};

impl<'a> TryDeserialize<'a> for u64 {
    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError> {
        if let Node::UnsignedNum(num) = node {
            Ok(num)
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryDeserialize<'a> for Vec<u8> {
    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError> {
        if let Node::String(s) = node {
            Ok(s.to_vec())
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

impl<'a> TryDeserialize<'a> for String {
    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError> {
        if let Node::String(s) = node {
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

impl<'a, T> TryDeserialize<'a> for Vec<T>
where
    T: TryDeserialize<'a>,
{
    fn try_deserialize_from_node(node: Node<'a>) -> Result<Self, ParsingError> {
        if let Node::List(list) = node {
            let mut new_list: Vec<T> = vec![];
            for node in list {
                new_list.push(T::try_deserialize_from_node(node.clone())?);
            }
            Ok(new_list)
        } else {
            Err(ParsingError::TypeMismatch)
        }
    }
}

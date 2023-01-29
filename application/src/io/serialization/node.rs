use std::collections::HashMap;

/// Структура, которая размечает байты, передаваемые на парсинг.
#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
    Integer(i64),
    String(&'a [u8]),
    List(Vec<Node<'a>>),
    Dict(HashMap<&'a [u8], Node<'a>>, &'a [u8]), // Также храним кусок, в котором этот словарь размещён, чтобы взять хеш от инфо-словарика
}

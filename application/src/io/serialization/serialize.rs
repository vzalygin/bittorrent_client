/// Писалось втупую, поэтому вероятны лишние аллокации.
use std::{any::Any, collections::HashMap};

use crate::{
    common_types::{
        data::Torrent,
        files::{File, Info, TorrentFile},
    },
    io::repo::{Id, TorrentRepo, WithId},
};

use super::consts::{LENGTH, MD5SUM, PATH};

pub trait SerializeTo<T> {
    fn serialize_to(&self) -> T;
}

impl SerializeTo<Vec<u8>> for u64 {
    fn serialize_to(&self) -> Vec<u8> {
        let value = self.to_string();

        let mut res = vec![b'i'];
        res.extend_from_slice(value.as_bytes());
        res.push(b'e');
        res
    }
}

impl SerializeTo<Vec<u8>> for Vec<u8> {
    fn serialize_to(&self) -> Vec<u8> {
        let mut res = vec![];
        let len = self.len().to_string();

        res.extend_from_slice(len.as_bytes());
        res.push(b':');
        res.extend(self);

        res
    }
}

impl SerializeTo<Vec<u8>> for String {
    fn serialize_to(&self) -> Vec<u8> {
        self.as_bytes().to_vec().serialize_to()
    }
}

impl<T> SerializeTo<Vec<u8>> for Vec<T>
where
    T: SerializeTo<Vec<u8>>,
{
    fn serialize_to(&self) -> Vec<u8> {
        let mut res = vec![];

        for t in self.into_iter() {
            res.extend(t.serialize_to());
        }

        res
    }
}

// impl SerializeTo<Vec<u8>> for HashMap<&[u8], Box<dyn SerializeTo<Vec<u8>>>> {
//     fn serialize_to(&self) -> Vec<u8> {
//         let mut res = vec![];
//         res.push(b'd');
//         for (k, v) in self {
//             res.extend(k.iter());
//             res.extend(v.serialize_to());
//         }
//         res.push(b'e');
//         res
//     }
// }

// fn add_required_field<'a: 'b, 'b, T>(
//     // Можно удобно выводить длинные типы
//     dict: &'b mut HashMap<&'a [u8], Box<dyn SerializeTo<Vec<u8>>>>,
//     k: &'a [u8],
//     v: &'a T,
// ) where T: SerializeTo<Vec<u8>> {
//     let v = (*v);
//     let v: Box<dyn SerializeTo<Vec<u8>>> = Box::new(v);
//     dict.insert(k, v);
// }

struct BencodeDictBuilder {
    data: Vec<u8>,
}

impl BencodeDictBuilder {
    fn new() -> BencodeDictBuilder {
        BencodeDictBuilder { data: vec![b'd'] }
    }

    fn required<T>(self, k: &[u8], v: T) -> BencodeDictBuilder
    where T: SerializeTo<Vec<u8>> {
        let mut data = self.data;
        data.extend_from_slice(k);
        data.extend(v.serialize_to().into_iter());
        BencodeDictBuilder { data }
    }

    fn optional<T>(self, k: &[u8], v: Option<T>) -> BencodeDictBuilder
    where T: SerializeTo<Vec<u8>> {
        if let Some(v) = v {
            self.required(k, v)
        } else {
            self
        }
    }

    fn fin(self) -> Vec<u8> {
        let mut data = self.data;
        data.push(b'e');
        data
    }
}

impl SerializeTo<Vec<u8>> for File {
    fn serialize_to(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(PATH, self.path.clone())
            .required(LENGTH, self.length)
            .optional(MD5SUM, self.md5sum.clone())
            .fin()
    }
}

impl SerializeTo<Vec<u8>> for Info {
    fn serialize_to(&self) -> Vec<u8> {
        todo!()
    }
}

impl SerializeTo<Vec<u8>> for TorrentFile {
    fn serialize_to(&self) -> Vec<u8> {
        todo!()
    }
}

impl SerializeTo<Vec<u8>> for Torrent {
    fn serialize_to(&self) -> Vec<u8> {
        todo!()
    }
}

impl<T> SerializeTo<Vec<u8>> for WithId<T>
where
    T: SerializeTo<Vec<u8>>,
{
    fn serialize_to(&self) -> Vec<u8> {
        todo!()
    }
}

impl SerializeTo<Vec<u8>> for Id {
    fn serialize_to(&self) -> Vec<u8> {
        todo!()
    }
}

impl SerializeTo<Vec<u8>> for TorrentRepo {
    fn serialize_to(&self) -> Vec<u8> {
        todo!()
    }
}

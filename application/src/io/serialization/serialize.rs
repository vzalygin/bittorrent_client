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

impl SerializeTo<Vec<u8>> for HashMap<&[u8], Box<dyn SerializeTo<Vec<u8>>>> {
    fn serialize_to(&self) -> Vec<u8> {
        let mut res = vec![];
        res.push(b'd');
        for (k, v) in self {
            res.extend(k.iter());
            res.extend(v.serialize_to());
        }
        res.push(b'e');
        res
    }
}

fn add_required_field<'a: 'b, 'b, T>(
    // Можно удобно выводить длинные типы
    dict: &'b mut HashMap<&'a [u8], Box<dyn SerializeTo<Vec<u8>>>>,
    k: &'a [u8],
    v: &'a T,
) where T: SerializeTo<Vec<u8>> {
    let v = (*v);
    let v: Box<dyn SerializeTo<Vec<u8>>> = Box::new(v);
    dict.insert(k, v);
}

impl SerializeTo<Vec<u8>> for File {
    fn serialize_to(&self) -> Vec<u8> {
        let mut d = HashMap::new();

        add_required_field(&mut d, PATH, &self.path);
        add_required_field(&mut d, LENGTH, &self.length);
        // add_optional_field(&mut d, MD5SUM, Box::new(self.md5sum));

        d.serialize_to()
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

use std::fmt::Debug;

use crate::{
    common_types::{
        data::Torrent,
        files::{File, Files, Info, TorrentFile},
    },
    io::repo::{Id, TorrentRepo, WithId},
};

use super::consts::{
    ANNOUNCE, ANNOUNCE_LIST, COMMENT, CREATED_BY, CREATION_DATE, DATA, ENCODING, FILES, HTTPSEEDS,
    ID, INFO, LENGTH, MD5SUM, NAME, PATH, PIECES, PIECE_LENGTH, PRIVATE, VALUE, TORRENTS,
};

pub trait SerializeTo<T> {
    fn serialize_to(&self) -> T;
}

struct BencodeDictBuilder {
    data: Vec<u8>,
}

impl BencodeDictBuilder {
    fn new() -> BencodeDictBuilder {
        BencodeDictBuilder { data: vec![b'd'] }
    }

    fn required<T>(self, k: &[u8], v: T) -> BencodeDictBuilder
    where
        T: SerializeTo<Vec<u8>>,
    {
        let mut data = self.data;
        data.extend_from_slice(k);
        data.extend(v.serialize_to().into_iter());
        BencodeDictBuilder { data }
    }

    fn optional<T>(self, k: &[u8], v: Option<T>) -> BencodeDictBuilder
    where
        T: SerializeTo<Vec<u8>>,
    {
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
        match &self.files {
            Files::Single(file) => BencodeDictBuilder::new()
                .required(NAME, file.name.clone())
                .required(LENGTH, file.length)
                .optional(MD5SUM, file.md5sum.clone()),
            Files::Multiple(files) => BencodeDictBuilder::new()
                .required(NAME, files.base_name.clone())
                .required(FILES, files.files.clone()),
        }
        .required(PIECE_LENGTH, self.piece_length)
        .required(PIECES, self.pieces.clone())
        .optional(PRIVATE, self.private)
        .fin()
    }
}

impl SerializeTo<Vec<u8>> for TorrentFile {
    fn serialize_to(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(INFO, self.info.clone())
            .required(ANNOUNCE, self.announce.clone())
            .optional(ENCODING, self.encoding.clone())
            .optional(HTTPSEEDS, self.httpseeds.clone())
            .optional(ANNOUNCE_LIST, self.announce_list.clone())
            .optional(CREATION_DATE, self.creation_date.clone())
            .optional(COMMENT, self.comment.clone())
            .optional(CREATED_BY, self.created_by.clone())
            .fin()
    }
}

impl SerializeTo<Vec<u8>> for Torrent {
    fn serialize_to(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(DATA, self.data.clone())
            .fin()
    }
}

impl SerializeTo<Vec<u8>> for Id {
    fn serialize_to(&self) -> Vec<u8> {
        self.as_bytes().to_vec().serialize_to()
    }
}

impl<T> SerializeTo<Vec<u8>> for WithId<T>
where
    T: SerializeTo<Vec<u8>> + Clone + PartialEq + Debug,
{
    fn serialize_to(&self) -> Vec<u8> {
        let e = self.clone();
        BencodeDictBuilder::new()
            .required(VALUE, e.value)
            .required(ID, e.id)
            .fin()
    }
}

impl SerializeTo<Vec<u8>> for TorrentRepo {
    fn serialize_to(&self) -> Vec<u8> {
        BencodeDictBuilder::new()
            .required(TORRENTS, self.get_torrent_list().clone())
            .fin()
    }
}

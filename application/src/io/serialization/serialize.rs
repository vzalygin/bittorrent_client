use nom::AsBytes;

pub trait SerializeTo<T> {
    fn serialize_to(self) -> T;
}

impl SerializeTo<Vec<u8>> for u64 {
    fn serialize_to(self) -> Vec<u8> {
        let value = self.to_string();

        let mut res = vec![b'i'];
        res.extend_from_slice(value.as_bytes());
        res.push(b'e');
        res
    }
}

impl SerializeTo<Vec<u8>> for Vec<u8> {
    fn serialize_to(self) -> Vec<u8> {
        let mut res = vec![];
        let len = self.len().to_string();

        res.extend_from_slice(len.as_bytes());
        res.push(b':');
        res.extend(self);

        res
    }
}

impl SerializeTo<Vec<u8>> for String {
    fn serialize_to(self) -> Vec<u8> {
        self.as_bytes().to_vec().serialize_to()
    }
}

impl<T> SerializeTo<Vec<u8>> for Vec<T>
where
    T: SerializeTo<Vec<u8>>,
{
    fn serialize_to(self) -> Vec<u8> {
        let mut res = vec![];

        for t in self.into_iter() {
            res.extend(t.serialize_to());
        }

        res
    }
}

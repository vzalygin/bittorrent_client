use super::Serialize;

impl Serialize for u64 {
    fn serialize(&self) -> Vec<u8> {
        let value = self.to_string();

        let mut res = vec![b'i'];
        res.extend_from_slice(value.as_bytes());
        res.push(b'e');
        res
    }
}

impl Serialize for Vec<u8> {
    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![];
        let len = self.len().to_string();

        res.extend_from_slice(len.as_bytes());
        res.push(b':');
        res.extend(self);

        res
    }
}

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec().serialize()
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![];

        res.push(b'l');
        for t in self.into_iter() {
            res.extend(t.serialize());
        }
        res.push(b'e');

        res
    }
}

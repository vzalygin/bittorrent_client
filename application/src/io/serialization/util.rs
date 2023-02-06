pub trait SerializeTo<T> {
    fn serialize(&self) -> T;
}

pub struct BencodeDictBuilder {
    data: Vec<u8>,
}

impl BencodeDictBuilder {
    pub fn new() -> BencodeDictBuilder {
        BencodeDictBuilder { data: vec![b'd'] }
    }

    pub fn required<T>(self, k: &[u8], v: T) -> BencodeDictBuilder
    where
        T: SerializeTo<Vec<u8>>,
    {
        let mut data = self.data;
        data.extend(k.to_vec().serialize());
        data.extend(v.serialize().into_iter());
        BencodeDictBuilder { data }
    }

    pub fn optional<T>(self, k: &[u8], v: Option<T>) -> BencodeDictBuilder
    where
        T: SerializeTo<Vec<u8>>,
    {
        if let Some(v) = v {
            self.required(k, v)
        } else {
            self
        }
    }

    pub fn fin(self) -> Vec<u8> {
        let mut data = self.data;
        data.push(b'e');
        data
    }
}

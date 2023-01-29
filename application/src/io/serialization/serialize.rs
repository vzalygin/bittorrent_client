pub fn serialize(value: u64) -> Vec<u8> {
    let value = value.to_string();

    let mut res = vec![b'i'];
    res.extend_from_slice(value.as_bytes());
    res.push(b'e');
    res
}

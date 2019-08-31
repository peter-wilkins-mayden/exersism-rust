#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter()
        .flat_map(|&n| {
            if n >= 128 {
                let first: u8 = ((n & 0x7f) | 0x80) as u8;
                let second = (n >> 7) as u8;
                vec![first, second]
            } else {
                vec![n as u8]
            }
        }).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!()
}

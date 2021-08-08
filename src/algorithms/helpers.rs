/// Pads the given last chunk according to the MD spec.
/// Returns a Vec of chunks (can contain 1 or 2 chunks)
///
/// Bits "10000..." are appended until message length is 56 mod 64.
/// Then message length is appended as a little-endian u64.
pub fn md_pad_last(last_chunk: &Vec<u8>, message_length: u64) -> Vec<Vec<u8>> {
    let mut appended = last_chunk.to_owned();
    let len_bytes = message_length.to_le_bytes();

    appended.push(128);
    while appended.len() % 64 != 56 { appended.push(0); }
    for len_byte in len_bytes { appended.push(len_byte) }

    appended.chunks(64).map(|chunk| { chunk.to_owned() }).collect()
}

/// Returns all 32-bit little-endian words from a byte vector.
///
/// Trailing bytes that are insufficient to make up a u32 are ignored.
pub fn exact_32_bit_words(bytes: &Vec<u8>) -> Vec<u32> {
    let mut words = vec![];
    let limit = (bytes.len() - bytes.len() % 4) / 4;

    for i in 0..limit {
        words.push(u32::from_le_bytes([
            bytes[i*4],
            bytes[i*4 + 1],
            bytes[i*4 + 2],
            bytes[i*4 + 3],
        ]));
    }

    words
}

/// Bitwise rotates a u32 value to the left.
/// Bits that get pushed outside the u32 range would end up
/// on the right side.
pub fn leftrotate(n: u32, amount: u8) -> u32 { 
    (n << amount) | (n >> (32 - amount)) 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn md_padding_works() {
        let chunk = "abc".as_bytes().to_owned();
        assert_eq!(md_pad_last(&chunk, 3), vec![
            vec![97, 98, 99, 128, 0, 0, 0, 0, 0, 0, 
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 3, 0, 0, 0,
                  0,  0,  0, 0]
        ]);
    }

    #[test]
    fn exact_32_bit_words_works() {
        assert_eq!(exact_32_bit_words(&vec![0, 0, 0, 5]), vec![5u32.to_be()]);
        assert_eq!(exact_32_bit_words(&vec![0, 0, 0, 5, 25, 3, 192, 40, 3]), 
            vec![5u32.to_be(), 0x1903c028u32.to_be()]);
        assert_eq!(exact_32_bit_words(&vec![0, 0, 0, 5, 25, 3, 192, 40, 3, 0, 0]), 
            vec![5u32.to_be(), 0x1903c028u32.to_be()]);
    }

    #[test]
    fn leftrotate_works() {
        assert_eq!(leftrotate(5, 2), 20);
        assert_eq!(leftrotate(3489705808, 4), 718093);
    }
}
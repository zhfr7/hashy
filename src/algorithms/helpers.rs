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
pub enum Endianness {
    Little,
    Big
}

/// Pads the given last chunk according to the MD spec.
/// Returns a Vec of chunks (can contain 1 or 2 chunks)
///
/// Bits "10000..." are appended until message length is 56 mod 64.
/// Then message length is appended as a u64 with the chosen endianness.
pub fn md_length_padding(last_chunk: &Vec<u8>, message_length: u64,
    endianness: Endianness) -> Vec<Vec<u8>> {
    let mut appended = last_chunk.to_owned();
    let len_bytes = match endianness {
        Endianness::Little  => message_length.to_le_bytes(),
        Endianness::Big     => message_length.to_be_bytes()
    };

    appended.push(128);
    while appended.len() % 64 != 56 { appended.push(0); }
    for len_byte in len_bytes { appended.push(len_byte) }

    appended.chunks(64).map(|chunk| { chunk.to_owned() }).collect()
}

/// Returns all 32-bit words in the given endianness from a byte vector.
///
/// Trailing bytes that are insufficient to make up a u32 are ignored.
pub fn exact_32_bit_words(bytes: &Vec<u8>, endianness: Endianness) -> Vec<u32> {
    let mut words = vec![];
    let limit = (bytes.len() - bytes.len() % 4) / 4;

    for i in 0..limit {
        let u32_bytes = [
            bytes[i*4],
            bytes[i*4 + 1],
            bytes[i*4 + 2],
            bytes[i*4 + 3],
        ];
        words.push(match endianness {
            Endianness::Little  => u32::from_le_bytes(u32_bytes),
            Endianness::Big     => u32::from_be_bytes(u32_bytes)
        });
    }

    words
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn md_padding_works() {
        let chunk = "abc".as_bytes().to_owned();
        assert_eq!(md_length_padding(&chunk, 3, Endianness::Little), vec![
            vec![97, 98, 99, 128, 0, 0, 0, 0, 0, 0, 
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 3, 0, 0, 0,
                  0,  0,  0, 0]
        ]);

        assert_eq!(md_length_padding(&chunk, 3, Endianness::Big), vec![
            vec![97, 98, 99, 128, 0, 0, 0, 0, 0, 0, 
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 0, 0, 0, 0, 0, 0, 0,
                  0,  0,  0, 3]
        ]);
    }

    #[test]
    fn exact_32_bit_words_works() {
        assert_eq!(exact_32_bit_words(&vec![0, 0, 0, 5], Endianness::Big), vec![5u32]);
        assert_eq!(exact_32_bit_words(&vec![0, 0, 0, 5, 25, 3, 192, 40, 3], Endianness::Little), 
            vec![5u32.to_be(), 0x1903c028u32.to_be()]);
        assert_eq!(exact_32_bit_words(&vec![0, 0, 0, 5, 25, 3, 192, 40, 3, 0, 0], Endianness::Little), 
            vec![5u32.to_be(), 0x1903c028u32.to_be()]);
    }
}
/// Use this to define the output type of a digest function
pub type DigestResult = Result<Vec<u8>, anyhow::Error>;

pub enum Endianness {
    Little,
    Big
}

/// Pads the given last chunk according to the MD spec.
/// Returns a Vec of chunks (can contain 1 or 2 chunks)
///
/// Bits "10000..." are appended until message length is 56 mod 64 bytes.
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

/// Pads the given last chunk according to the MD spec.
/// Returns a Vec of 64-byte chunks (can contain 1 or 2 chunks)
///
/// Bits "10000..." are appended until message length is 112 mod 128 bytes.
/// Then message length is appended as a u128 with the chosen endianness.
pub fn md_length_padding_64(last_chunk: &Vec<u8>, message_length: u128,
    endianness: Endianness) -> Vec<Vec<u8>> {
    let mut appended = last_chunk.to_owned();
    let len_bytes = match endianness {
        Endianness::Little  => message_length.to_le_bytes(),
        Endianness::Big     => message_length.to_be_bytes()
    };

    appended.push(128);
    while appended.len() % 128 != 112 { appended.push(0); }
    for len_byte in len_bytes { appended.push(len_byte) }

    appended.chunks(128).map(|chunk| { chunk.to_owned() }).collect()
}

/// Returns all 32-bit words in the given endianness from a byte vector.
///
/// Trailing bytes that are insufficient to make up a u32 are ignored.
pub fn exact_32_bit_words(bytes: &Vec<u8>, endianness: Endianness) -> Vec<u32> {
    let mut words = vec![];
    let limit = (bytes.len() - bytes.len() % 4) / 4;
    let is_le = matches!(endianness, Endianness::Little);

    for i in 0..limit {
        let j = i*4;
        let u32_bytes = [
            bytes[j], bytes[j+1], bytes[j+2], bytes[j+3],
        ];
        words.push( 
            if is_le { u32::from_le_bytes(u32_bytes) }
            else { u32::from_be_bytes(u32_bytes) }
        );
    }

    words
}

/// Returns all 64-bit words in the given endianness from a byte vector.
///
/// Trailing bytes that are insufficient to make up a u64 are ignored.
pub fn exact_64_bit_words(bytes: &Vec<u8>, endianness: Endianness) -> Vec<u64> {
    let mut words = vec![];
    let limit = (bytes.len() - bytes.len() % 8) / 8;
    let is_le = matches!(endianness, Endianness::Little);

    for i in 0..limit {
        let j = i*8;
        let u64_bytes = [
            bytes[j], bytes[j + 1], bytes[j + 2], bytes[j + 3],
            bytes[j + 4], bytes[j + 5], bytes[j + 6], bytes[j + 7]
        ];
        words.push( 
            if is_le { u64::from_le_bytes(u64_bytes) }
            else { u64::from_be_bytes(u64_bytes) }
        );
    }

    words
}

/// Tests the digest function, in which it compares
/// the digested input with the intended output.
/// Expects a (&str, &str) tuple for the io_pair (input, output).
#[cfg(test)]
#[macro_export]
macro_rules! test_digest {
    ($digest_fun:expr, $( $io_pair:expr ), *) => {
        $(
            let (input, expected) = $io_pair;
            let data = crate::DataType::Bytes(input.as_bytes().to_vec());
            let digest_bytes = $digest_fun(data).unwrap();

            assert_eq!(*expected, crate::post_process::encode(digest_bytes, 
                crate::post_process::Encoding::Hex(false)));
        )*
    };
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

    #[test]
    fn exact_64_bit_words_works() {
        assert_eq!(exact_64_bit_words(&vec![0, 0, 0, 0, 0, 0, 0, 5], Endianness::Big), vec![5u64]);
        assert_eq!(exact_64_bit_words(&vec![0, 0, 0, 0, 0, 0, 0, 5, 23], Endianness::Big), vec![5u64]);
        assert_eq!(exact_64_bit_words(
            &vec![0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 6], Endianness::Big), vec![5u64, 6u64]);
    }
}
// Reference: https://datatracker.ietf.org/doc/html/rfc1320

use crate::data_container::DataType;
use crate::algorithms::helpers::*;

type MdBuffer = (u32, u32, u32, u32);

const CHUNK_SIZE: usize = 64;
const INIT_MD_BUFFER: MdBuffer = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);
const S_TABLE_REDUCED: [u8; 12] = [
    3, 7, 11, 19,
    3, 5, 9, 13,
    3, 9, 11, 15
];

pub fn digest(data: DataType) -> std::io::Result<Vec<u8>> {
    let mut md_buf = INIT_MD_BUFFER;

    // Process each chunk via last_chunk
    let mut last_chunk = None;
    let mut len: u64 = 0;
    for chunk in data.into_iter(CHUNK_SIZE) {
        process_chunk(last_chunk, &mut md_buf);

        let chunk_bytes = chunk?;
        len = len.wrapping_add((chunk_bytes.len() * 8) as u64);
        last_chunk = Some(chunk_bytes);
    }

    // Default last chunk to an empty Vec
    let last_chunk = last_chunk.unwrap_or_default();

    // Process last padded chunk(s)
    for chunk in md_length_padding(&last_chunk, len, Endianness::Little) {
        process_chunk(Some(chunk), &mut md_buf);
    }

    let (a, b, c, d) = md_buf;
    Ok([a.to_le_bytes(), 
        b.to_le_bytes(), 
        c.to_le_bytes(), 
        d.to_le_bytes()].concat())
}

fn process_chunk(chunk: Option<Vec<u8>>, (a0, b0, c0, d0): &mut MdBuffer) {
    if chunk.is_none() { return }

    let chunk = chunk.unwrap();
    let words = exact_32_bit_words(&chunk, Endianness::Little);

    // Main loop of MD4
    let (a_n, b_n, c_n, d_n) = 
        (0..48).fold((*a0, *b0, *c0, *d0), 
        |(a, b, c, d), i| {
            let aux_plus_const = match i {
                0..=15  => b&c | !b&d,
                16..=31 => (b&c | b&d | c&d).wrapping_add(0x5a827999),
                _       => (b ^ c ^ d)      .wrapping_add(0x6ed9eba1)
            };

            let a = (
                a.wrapping_add(aux_plus_const)
                .wrapping_add(words[k(i)])).rotate_left(s(i));

            (d, a, b, c)
        });

    *a0 = a0.wrapping_add(a_n);
    *b0 = b0.wrapping_add(b_n);
    *c0 = c0.wrapping_add(c_n);
    *d0 = d0.wrapping_add(d_n);
}

fn k(i: usize) -> usize {
    let i_norm = i % 16;
    match i {
        0..=15  => i,
        16..=31 => (4*i_norm + i_norm / 4) % 16,
        _       => {
            let seq = [0, 2, 1, 3];
            seq[i_norm/4] + seq[i_norm%4]*4
        },
    }
}

fn s(i: usize) -> u32 { S_TABLE_REDUCED[ i/16 * 4 + i%4 ] as u32 }

#[cfg(test)]
mod test {
    use super::*;
    use crate::post_process::*;

    #[test]
    fn correct_digests() {
        let input_expected_pairs = [
            ("", 
                "31d6cfe0d16ae931b73c59d7e0c089c0"),
            ("a", 
                "bde52cb31de33e46245e05fbdbd6fb24"),
            ("message digest",
                "d9130a8164549fe818874806e1c7014b"),
            ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                "043f8582f241db351ce627e153e7f0e4")
        ];

        for (input, expected) in input_expected_pairs {
            let data = DataType::Bytes(input.as_bytes().to_vec());
            let digest_bytes = digest(data).unwrap();

            assert_eq!(encode(digest_bytes, Encoding::Hex(false)), expected);
        }
    }

    #[test]
    fn correct_k_values() {
        let left: Vec<usize> = (0..48).map(|i| k(i)).collect();
        let right = vec![
            0, 1, 2, 3,  4, 5, 6, 7,   8, 9, 10, 11, 12, 13, 14, 15,
            0, 4, 8, 12, 1, 5, 9, 13,  2, 6, 10, 14, 3,  7,  11, 15,
            0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5,  13, 3, 11, 7, 15
        ];
        assert_eq!(left, right);
    }

    #[test]
    fn correct_s_values() {
        let left: Vec<u32> = [0, 5, 10, 15, 16, 21, 26, 31, 32, 37, 42, 47]
                    .iter()
                    .map(|&i| s(i)).collect();
        let right = vec![
            3, 7, 11, 19,
            3, 5, 9, 13,
            3, 9, 11, 15
        ];
        assert_eq!(left, right);
    }
}
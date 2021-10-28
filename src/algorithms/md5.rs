// Reference: https://en.wikipedia.org/wiki/MD5

use crate::data_container::DataType;
use super::helpers::*;

type MdBuffer = (u32, u32, u32, u32);

const CHUNK_SIZE: usize = 64;
const INIT_MD_BUFFER: MdBuffer = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);
const S_TABLE_REDUCED: [u8; 16] = [7, 12, 17, 22, 5, 9, 14, 20, 4, 11, 16, 23, 6, 10, 15, 21];
const K_TABLE: [u32; CHUNK_SIZE] = [
0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
];

/// Generates an MD5 digest from the given DataType object.
///
/// Returns an io::Result containing the digest as bytes 
/// in the form of a Vec\<u8>.
pub fn digest(data: DataType) -> DigestResult {
    // Initial MD buffer
    let mut md_buf = INIT_MD_BUFFER;

    // Process each chunk via last_chunk and increments length
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

    // Process remaining chunk(s) after padding the last chunk
    for chunk in md_length_padding(&last_chunk, len, Endianness::Little) {
        process_chunk(Some(chunk), &mut md_buf);
    }

    let (a, b, c, d) = md_buf;
    Ok([a.to_le_bytes(), 
        b.to_le_bytes(), 
        c.to_le_bytes(), 
        d.to_le_bytes()].concat())
}

/// Processes a chunk and mutates the MD buffer accordingly.
fn process_chunk(chunk: Option<Vec<u8>>, (a0, b0, c0, d0): &mut MdBuffer) {
    if chunk.is_none() { return }

    let chunk = chunk.unwrap();
    let words = exact_32_bit_words(&chunk, Endianness::Little);

    // Main loop of MD5
    let (a_n, b_n, c_n, d_n) = 
        (0..64).fold((*a0, *b0, *c0, *d0),
        |(a, b, c, d), i: usize| {
            let (f, g) = 
            match i {
                0..=15  => ((b & c) | (!b & d)  , i             ),
                16..=31 => ((d & b) | (!d & c)  , (5*i + 1) % 16),
                32..=47 => (b ^ c ^ d           , (3*i + 5) % 16),
                _       => (c ^ (b | !d)        , (7*i) % 16    )
            };

            let f = f.wrapping_add(a).wrapping_add(k(i)).wrapping_add(words[g]);

            (d, b.wrapping_add(f.rotate_left(s(i))), b, c)
        });

    *a0 = a0.wrapping_add(a_n);
    *b0 = b0.wrapping_add(b_n);
    *c0 = c0.wrapping_add(c_n);
    *d0 = d0.wrapping_add(d_n);
}

/// Returns the value in the s-table at index i
fn s(i: usize) -> u32 { S_TABLE_REDUCED[4*(i/16) + i%4] as u32 }

/// Returns the value in the k-table at index i
fn k(i: usize) -> u32 { K_TABLE[i] }

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_digest;

    #[test]
    fn correct_digests() {
        test_digest!(digest,
            ("", 
                "d41d8cd98f00b204e9800998ecf8427e"),
            ("The quick brown fox jumps over the lazy dog", 
                "9e107d9d372bb6826bd81d3542a419d6"),
            ("This is a very long string with the purpose of exceeding the chunk length of 64 bytes",
                "ba70257a277a031df015d5741af768f3")
        );
    }

    #[test]
    fn correct_s_values() {
        assert_eq!((s(0), s(1), s(2), s(3)), (7, 12, 17, 22));
        assert_eq!((s(24), s(25), s(26), s(27)), (5, 9, 14, 20));
        assert_eq!((s(40), s(41), s(42), s(43)), (4, 11, 16, 23));
        assert_eq!((s(60), s(61), s(62), s(63)), (6, 10, 15, 21));
    }
}
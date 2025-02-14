use super::{
    helpers::{exact_32_bit_words, md_length_padding, Endianness},
    Algorithm,
};
use crate::chunked_stream::ChunkedStream;

type Buffer = (u32, u32, u32, u32, u32);

const CHUNK_SIZE: usize = 64;
const INIT_BUFFER: Buffer = (0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0);

/// SHA-1 (Secure Hash Algorithm 1)
///
/// Reference: https://en.wikipedia.org/wiki/SHA-1
pub struct Sha1;

impl Algorithm for Sha1 {
    fn digest(&self, data: ChunkedStream) -> super::DigestResult {
        let mut buf = INIT_BUFFER;

        // Process each chunk via last_chunk
        let mut last_chunk: Option<Vec<u8>> = None;
        let mut len: u64 = 0;
        for chunk in data.into_iter(CHUNK_SIZE) {
            if let Some(last_chunk) = last_chunk {
                process_chunk(&last_chunk, &mut buf);
            }

            let chunk_bytes = chunk?;
            len = len.wrapping_add((chunk_bytes.len() * 8) as u64);
            last_chunk = Some(chunk_bytes);
        }

        // Process remaining chunks
        let last_chunk = last_chunk.unwrap_or_default();
        for chunk in md_length_padding(&last_chunk, len, Endianness::Big) {
            process_chunk(&chunk, &mut buf);
        }

        let (h0, h1, h2, h3, h4) = buf;
        Ok([
            h0.to_be_bytes(),
            h1.to_be_bytes(),
            h2.to_be_bytes(),
            h3.to_be_bytes(),
            h4.to_be_bytes(),
        ]
        .concat())
    }
}

/// Processes each chunk according to the SHA1 spec.
/// Ignores chunk if it is None.
fn process_chunk(chunk: &[u8], (h0, h1, h2, h3, h4): &mut Buffer) {
    let mut words = exact_32_bit_words(chunk, Endianness::Big);
    for i in 16..80 {
        words.push((words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16]).rotate_left(1));
    }

    let (a_n, b_n, c_n, d_n, e_n) =
        (0..80).fold((*h0, *h1, *h2, *h3, *h4), |(a, b, c, d, e), i| {
            let f_plus_k = match i {
                0..=19 => (b & c | !b & d).wrapping_add(0x5A827999),
                20..=39 => (b ^ c ^ d).wrapping_add(0x6ED9EBA1),
                40..=59 => (b & c | b & d | c & d).wrapping_add(0x8F1BBCDC),
                _ => (b ^ c ^ d).wrapping_add(0xCA62C1D6),
            };

            let temp = a
                .rotate_left(5)
                .wrapping_add(f_plus_k)
                .wrapping_add(e)
                .wrapping_add(words[i]);

            (temp, a, b.rotate_left(30), c, d)
        });

    *h0 = h0.wrapping_add(a_n);
    *h1 = h1.wrapping_add(b_n);
    *h2 = h2.wrapping_add(c_n);
    *h3 = h3.wrapping_add(d_n);
    *h4 = h4.wrapping_add(e_n);
}

#[cfg(test)]
mod test {
    use crate::algorithms::helpers::test::assert_digest;

    use super::Sha1;

    #[test]
    fn sha1_correct() {
        for (input, expected) in [("", 
                "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            ("The quick brown fox jumps over the lazy dog", 
                "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12"),
            ("This is a very long string with the purpose of exceeding the chunk length of 64 bytes",
                "37c8456433925d4771764b4dad3b8b1c76019d1b")] {
            assert_digest(&Sha1, input, expected);
        }
    }
}

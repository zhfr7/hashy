use std::io;

use rayon::prelude::*;

use crate::{
    algorithms::helpers::{exact_64_bit_words, Endianness},
    chunked_stream::ChunkedStream,
};

use super::{Algorithm, DigestResult};

// Represents fractional part of sqrt(6)
const Q: [u64; 15] = [
    0x7311c2812425cfa0,
    0x6432286434aac8e7,
    0xb60450e9ef68b7c1,
    0xe8fb23908d9f06f1,
    0xdd2e76cba691e5bf,
    0x0cd0d63b2c30bc41,
    0x1f8ccf6823058f8a,
    0x54e5ed5b88e3775d,
    0x4ad12aae0a6d6031,
    0x3e7f16bb88222e0d,
    0x8af8671d3fb50c2c,
    0x995ad1178bd25c31,
    0xc878c1dd04c4b633,
    0x3b72066c7a1552ac,
    0x0d6f3522631effcb,
];
const T: [usize; 5] = [17, 18, 21, 31, 67];

const R_SHIFTS: [u8; 16] = [10, 5, 13, 10, 11, 12, 2, 7, 14, 15, 7, 13, 11, 7, 6, 12];
const L_SHIFTS: [u8; 16] = [11, 24, 9, 16, 15, 9, 27, 15, 6, 2, 29, 8, 15, 5, 31, 9];

const N: usize = 89;
const S_PRIME_0: u64 = 0x0123456789abcdef;
const S_STAR: u64 = 0x7311c2812425cfa0;

// Mode parameter
const L: u64 = 64;

struct Key {
    /// Padded key value in words
    value: [u64; 8],
    /// Key length in bytes, before padding. Corresponds to `keysize`
    length: usize,
}

impl From<Vec<u8>> for Key {
    fn from(mut key: Vec<u8>) -> Self {
        let length = key.len();

        if length >= 64 {
            // Truncate keys larger than 64 bytes
            Self {
                value: exact_64_bit_words(&key[..64], Endianness::Big)
                    .try_into()
                    .unwrap(),
                length: 64,
            }
        } else {
            let pad_byte_count = 64 - key.len();

            key.extend(std::iter::repeat(0).take(pad_byte_count));

            Self {
                value: exact_64_bit_words(&key, Endianness::Big)
                    .try_into()
                    .unwrap(),
                length,
            }
        }
    }
}

/// MD6 (Message Digest 6) algorithm
///
/// Mode control is set to maximum parallelism (L = 64), therefore data stream
/// will be entirely consumed before hashing algorithm starts. Memory usage might
/// be higher with large files.
///
/// References:
/// - https://web.archive.org/web/20170812072847/https://groups.csail.mit.edu/cis/md6/submitted-2008-10-27/Supporting_Documentation/md6_report.pdf
/// - https://sourceforge.net/projects/md6sum/
pub struct Md6 {
    output_length: usize,
    rounds: usize,
}

impl Md6 {
    pub fn new(output_length: usize, rounds: Option<usize>) -> Self {
        Self {
            output_length,
            rounds: rounds.unwrap_or(40 + output_length / 4),
        }
    }
}

impl Algorithm for Md6 {
    fn digest(&self, data: ChunkedStream) -> DigestResult {
        self.digest_keyed(data, None)
    }
}

impl Md6 {
    fn digest_keyed(&self, data: ChunkedStream, key: Option<Vec<u8>>) -> DigestResult {
        let message_chunks = data
            .into_iter(65536)
            .collect::<io::Result<Vec<Vec<u8>>>>()?;
        let mut message: Vec<u8> = message_chunks.into_iter().flatten().collect();
        let key: Key = key.map(|k| k.into()).unwrap_or(Key {
            value: [0; 8],
            length: 0,
        });

        let mut l = 0;

        loop {
            l = l + 1;

            message = self.par(&message, &key, l);

            if message.len() == 16 * 8 {
                return Ok(message[(16 * 8 - self.output_length / 8)..].to_vec());
            }
        }
    }

    fn par(&self, message: &[u8], key: &Key, level: usize) -> Vec<u8> {
        const BLOCK_SIZE_BYTES: usize = 64 * 8;

        let len_modulo = message.len() % BLOCK_SIZE_BYTES;
        let padding_byte_count = if message.len() == 0 || len_modulo > 0 {
            BLOCK_SIZE_BYTES - len_modulo
        } else {
            0
        };

        let mut padded_message = message.to_vec();

        padded_message.extend(std::iter::repeat(0).take(padding_byte_count));

        let words = exact_64_bit_words(&padded_message, Endianness::Big);
        let blocks: Vec<&[u64]> = words.chunks(BLOCK_SIZE_BYTES / 8).collect();

        blocks
            .par_iter()
            .enumerate()
            .map(|(i, block)| {
                let p: u64 = if i == blocks.len() - 1 {
                    (padding_byte_count as u64) * 8
                } else {
                    0
                };
                let z: u64 = if blocks.len() == 1 { 1 } else { 0 };

                let v = (0 << 60)
                    | ((self.rounds as u64 & 0xFFF) << 48)
                    | ((L & 0xFF) << 40)
                    | ((z & 0xF) << 36)
                    | ((p & 0xFFFF) << 20)
                    | ((key.length as u64 & 0xFF) << 12)
                    | (self.output_length as u64 & 0xFFF);

                let u = ((level as u64) << 56) + (i as u64);

                let mut compress_input = [0u64; N];

                compress_input[..15].copy_from_slice(&Q);
                compress_input[15..23].copy_from_slice(&key.value);
                compress_input[23] = u;
                compress_input[24] = v;
                compress_input[25..].copy_from_slice(block);

                self.compress(compress_input)
                    .map(|word| word.to_be_bytes())
                    .to_vec()
                    .into_flattened()
            })
            .flatten()
            .collect()
    }

    fn compress(&self, input: [u64; N]) -> [u64; 16] {
        let t = self.rounds * 16;
        let mut a = vec![0; t + N];

        a[..N].copy_from_slice(&input);

        let mut s_k = S_PRIME_0;

        for i in N..(t + N) {
            let k = i - N;

            let mut x = s_k ^ a[k] ^ a[i - T[0]];

            x = x ^ (a[i - T[1]] & a[i - T[2]]) ^ (a[i - T[3]] & a[i - T[4]]);
            x = x ^ (x >> R_SHIFTS[k % 16]);

            a[i] = x ^ (x << L_SHIFTS[k % 16]);

            if k % 16 == 15 {
                s_k = get_next_s(s_k);
            }
        }

        // Slice guaranteed to be of size 16
        a[(t + N - 16)..].try_into().unwrap()
    }
}

fn get_next_s(s: u64) -> u64 {
    s.rotate_left(1) ^ (s & S_STAR)
}

#[cfg(test)]
mod test {
    use crate::chunked_stream::ChunkedStream;

    use super::Md6;

    struct TestCase {
        data: ChunkedStream,
        output_length: usize,
        rounds: Option<usize>,
        key: Option<String>,
        expected: &'static str,
    }

    #[test]
    fn correct_digests() {
        let test_cases = vec![
            TestCase {
                data: ChunkedStream::Bytes(vec![]),
                output_length: 256,
                rounds: None,
                key: None,
                expected: "bca38b24a804aa37d821d31af00f5598230122c5bbfc4c4ad5ed40e4258f04ca",
            },
            TestCase {
                // Taken from the first example from specification
                data: ChunkedStream::from("abc".to_string()),
                output_length: 256,
                rounds: Some(5),
                key: None,
                expected: "8854c14dc284f840ed71ad7ba542855ce189633e48c797a55121a746be48cec8",
            },
            TestCase {
                // Taken from the second example from specification
                data: ChunkedStream::Bytes(
                    [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]
                        .into_iter()
                        .cycle()
                        .take(600)
                        .collect(),
                ),
                output_length: 224,
                rounds: Some(5),
                key: Some("abcde12345".to_string()),
                expected: "894cf0598ad3288ed4bb5ac5df23eba0ac388a11b7ed2e3dd5ec5131",
            },
        ];

        for test_case in test_cases {
            let md6 = Md6::new(test_case.output_length, test_case.rounds);

            let digest = md6
                .digest_keyed(test_case.data, test_case.key.map(|k| k.as_bytes().to_vec()))
                .unwrap();
            let digest_hex = hex::encode(digest);

            assert_eq!(test_case.expected, digest_hex);
        }
    }
}

// Reference: https://en.wikipedia.org/wiki/SHA-2

use crate::DataType;
use super::helpers::*;

const CHUNK_SIZE_256: usize = 64;
const CHUNK_SIZE_512: usize = 128;
const INIT_BUFFER_224: [u32; 8] = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 
    0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4
];
const INIT_BUFFER_256: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
];
const INIT_BUFFER_384: [u64; 8] = [
    0xcbbb9d5dc1059ed8, 0x629a292a367cd507, 0x9159015a3070dd17, 0x152fecd8f70e5939, 
    0x67332667ffc00b31, 0x8eb44a8768581511, 0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4
];
const INIT_BUFFER_512: [u64; 8] = [
    0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 
    0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1, 
    0x510e527fade682d1, 0x9b05688c2b3e6c1f, 
    0x1f83d9abfb41bd6b, 0x5be0cd19137e2179
];
const K_TABLE_256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];
const K_TABLE_512: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc, 0x3956c25bf348b538, 
    0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242, 0x12835b0145706fbe, 
    0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2, 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 
    0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5, 0x983e5152ee66dfab, 
    0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725, 
    0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 
    0x53380d139d95b3df, 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b, 
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218, 
    0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8, 0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 
    0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 
    0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec, 
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b, 0xca273eceea26619c, 
    0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba, 0x0a637dc5a2c898a6, 
    0x113f9804bef90dae, 0x1b710b35131c471b, 0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 
    0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
];

pub fn digest_224(data: DataType) -> std::io::Result<Vec<u8>> {
    let result = digest_32(data, INIT_BUFFER_224)?;
    Ok(result[..28].to_vec())
}

pub fn digest_256(data: DataType) -> std::io::Result<Vec<u8>> {
    digest_32(data, INIT_BUFFER_256)
}

pub fn digest_32(data: DataType, init_buffer: [u32; 8]) -> std::io::Result<Vec<u8>> {
    let mut buf = init_buffer;

    // Process each chunk via last_chunk
    let mut last_chunk = None;
    let mut len: u64 = 0;
    for chunk in data.into_iter(CHUNK_SIZE_256) {
        process_chunk_32(last_chunk, &mut buf);

        let chunk_bytes = chunk?;
        len = len.wrapping_add((chunk_bytes.len() * 8) as u64);
        last_chunk = Some(chunk_bytes);
    }

    // Default last_chunk to empty Vec if None
    let last_chunk = last_chunk.unwrap_or_default();

    // Process remaining padded chunk(s)
    for chunk in md_length_padding(&last_chunk, len, Endianness::Big) {
        process_chunk_32(Some(chunk), &mut buf);
    }

    let out: Vec<[u8; 4]> = buf.iter()
        .map(|word| {word.to_be_bytes()})
        .collect();
    Ok(out.concat())
}

fn process_chunk_32(chunk: Option<Vec<u8>>, buffer: &mut [u32; 8]) {
    if chunk.is_none() { return }

    let chunk = chunk.unwrap();

    // Extend 16 words into 64
    let mut words = exact_32_bit_words(&chunk, Endianness::Big);
    for i in 16..64 {
        let s0 = words[i-15].rotate_right(7) ^ words[i-15].rotate_right(18) ^ (words[i-15] >> 3);
        let s1 = words[i-2].rotate_right(17) ^ words[i-2].rotate_right(19)  ^ (words[i-2] >> 10);
        words.push(words[i-16]
            .wrapping_add(s0)
            .wrapping_add(words[i-7])
            .wrapping_add(s1));
    }

    // Main loop
    let buffer_n = (0..64).fold(*buffer, 
        |h, i| {
            let (a, b, c, d, e, f, g, h) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e&f) ^ (!e&g);
            let temp1 = h.wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K_TABLE_256[i])
                .wrapping_add(words[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a&b) ^ (a&c) ^ (b&c);
            let temp2 = s0.wrapping_add(maj);

            [temp1.wrapping_add(temp2), a, b, c, d.wrapping_add(temp1), e, f, g]
        });

    for i in 0..8 {
        buffer[i] = buffer[i].wrapping_add(buffer_n[i]);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_helper::test_digest;

    #[test]
    fn correct_sha224_digests() {
        test_digest(&digest_224, &[
            ("", 
                "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"),
            ("The quick brown fox jumps over the lazy dog", 
                "730e109bd7a8a32b1cb9d9a09aa2325d2430587ddbc0c38bad911525"),
            ("This is a very long string with the purpose of exceeding the chunk length of 64 bytes",
                "c0ebfc1f8de0114969f0164ba381bc3cce984e225adfa79011392cc9")
        ]);
    }

    #[test]
    fn correct_sha256_digests() {
        test_digest(&digest_256, &[
            ("", 
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("The quick brown fox jumps over the lazy dog", 
                "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"),
            ("This is a very long string with the purpose of exceeding the chunk length of 64 bytes",
                "7ad7a19a23f6f2285256b72b0854d14c80e04fcc2ae1173f1ffeb9df296ee954")
        ]);
    }
}
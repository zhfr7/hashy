use super::{
    helpers::{
        exact_32_bit_words, exact_64_bit_words, md_length_padding, md_length_padding_64, Endianness,
    },
    Algorithm, DigestResult,
};
use crate::chunked_stream::ChunkedStream;

const CHUNK_SIZE_256: usize = 64;
const CHUNK_SIZE_512: usize = 128;
const INIT_BUFFER_224: [u32; 8] = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
];
const INIT_BUFFER_256: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];
const INIT_BUFFER_384: [u64; 8] = [
    0xcbbb9d5dc1059ed8,
    0x629a292a367cd507,
    0x9159015a3070dd17,
    0x152fecd8f70e5939,
    0x67332667ffc00b31,
    0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7,
    0x47b5481dbefa4fa4,
];
const INIT_BUFFER_512: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];
const INIT_BUFFER_512_224: [u64; 8] = [
    0x8C3D37C819544DA2,
    0x73E1996689DCD4D6,
    0x1DFAB7AE32FF9C82,
    0x679DD514582F9FCF,
    0x0F6D2B697BD44DA8,
    0x77E36F7304C48942,
    0x3F9D85A86A1D36C8,
    0x1112E6AD91D692A1,
];
const INIT_BUFFER_512_256: [u64; 8] = [
    0x22312194FC2BF72C,
    0x9F555FA3C84C64C2,
    0x2393B86B6F53B151,
    0x963877195940EABD,
    0x96283EE2A88EFFE3,
    0xBE5E1E2553863992,
    0x2B0199FC2C85B8AA,
    0x0EB72DDC81C52CA2,
];
const K_TABLE_256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
const K_TABLE_512: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

pub enum Sha2Variant {
    _224,
    _256,
    _384,
    _512,
    _512_224,
    _512_256,
}

/// SHA-2 (Secure Hash Algorithm 2), includes all variants:
///
/// - SHA-224
/// - SHA-256
/// - SHA-384
/// - SHA-512/224
/// - SHA-512/256
///
/// References:
/// - https://en.wikipedia.org/wiki/SHA-2,
/// - https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
pub struct Sha2 {
    variant: Sha2Variant,
}

impl Sha2 {
    pub fn new(variant: Sha2Variant) -> Self {
        Self { variant }
    }
}

impl Algorithm for Sha2 {
    fn digest(&self, data: ChunkedStream) -> DigestResult {
        match self.variant {
            Sha2Variant::_224 => Ok(digest_32(data, INIT_BUFFER_224)?[..28].to_vec()),
            Sha2Variant::_256 => digest_32(data, INIT_BUFFER_256),
            Sha2Variant::_384 => Ok(digest_64(data, INIT_BUFFER_384)?[..48].to_vec()),
            Sha2Variant::_512 => digest_64(data, INIT_BUFFER_512),
            Sha2Variant::_512_224 => Ok(digest_64(data, INIT_BUFFER_512_224)?[..28].to_vec()),
            Sha2Variant::_512_256 => Ok(digest_64(data, INIT_BUFFER_512_256)?[..32].to_vec()),
        }
    }
}

/// Generalized SHA2 function for variants which use u32s (state size of 256).
fn digest_32(data: ChunkedStream, init_buffer: [u32; 8]) -> DigestResult {
    let mut buf = init_buffer;

    // Process each chunk via last_chunk
    let mut last_chunk: Option<Vec<u8>> = None;
    let mut len: u64 = 0;
    for chunk in data.into_iter(CHUNK_SIZE_256) {
        if let Some(last_chunk) = last_chunk {
            process_chunk_32(&last_chunk, &mut buf);
        }

        let chunk_bytes = chunk?;
        len = len.wrapping_add((chunk_bytes.len() * 8) as u64);
        last_chunk = Some(chunk_bytes);
    }

    // Default last_chunk to empty Vec if None
    let last_chunk = last_chunk.unwrap_or_default();

    // Process remaining padded chunk(s)
    for chunk in md_length_padding(&last_chunk, len, Endianness::Big) {
        process_chunk_32(&chunk, &mut buf);
    }

    let out: Vec<[u8; 4]> = buf.iter().map(|word| word.to_be_bytes()).collect();
    Ok(out.concat())
}

/// Generalized SHA2 function for variants which use u64s (state size of 512).
fn digest_64(data: ChunkedStream, init_buffer: [u64; 8]) -> DigestResult {
    let mut buf = init_buffer;

    // Process each chunk via last_chunk
    let mut last_chunk: Option<Vec<u8>> = None;
    let mut len: u128 = 0;
    for chunk in data.into_iter(CHUNK_SIZE_512) {
        if let Some(last_chunk) = last_chunk {
            process_chunk_64(&last_chunk, &mut buf);
        }

        let chunk_bytes = chunk?;
        len = len.wrapping_add((chunk_bytes.len() * 8) as u128);
        last_chunk = Some(chunk_bytes);
    }

    // Process remaining padded chunk(s)
    let last_chunk = last_chunk.unwrap_or_default();
    for chunk in md_length_padding_64(&last_chunk, len, Endianness::Big) {
        process_chunk_64(&chunk, &mut buf);
    }

    let out: Vec<[u8; 8]> = buf.iter().map(|word| word.to_be_bytes()).collect();
    Ok(out.concat())
}

fn process_chunk_32(chunk: &[u8], buffer: &mut [u32; 8]) {
    // Extend 16 words into 64
    let mut words = exact_32_bit_words(chunk, Endianness::Big);
    for i in 16..64 {
        let s0 =
            words[i - 15].rotate_right(7) ^ words[i - 15].rotate_right(18) ^ (words[i - 15] >> 3);
        let s1 =
            words[i - 2].rotate_right(17) ^ words[i - 2].rotate_right(19) ^ (words[i - 2] >> 10);
        words.push(
            words[i - 16]
                .wrapping_add(s0)
                .wrapping_add(words[i - 7])
                .wrapping_add(s1),
        );
    }

    // Main loop
    let buffer_n = (0..64).fold(*buffer, |h, i| {
        let (a, b, c, d, e, f, g, h) = (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ (!e & g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K_TABLE_256[i])
            .wrapping_add(words[i]);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(maj);

        [
            temp1.wrapping_add(temp2),
            a,
            b,
            c,
            d.wrapping_add(temp1),
            e,
            f,
            g,
        ]
    });

    for i in 0..8 {
        buffer[i] = buffer[i].wrapping_add(buffer_n[i]);
    }
}

fn process_chunk_64(chunk: &[u8], buffer: &mut [u64; 8]) {
    // Extend 16 words into 80
    let mut words = exact_64_bit_words(chunk, Endianness::Big);
    for i in 16..80 {
        let s0 =
            words[i - 15].rotate_right(1) ^ words[i - 15].rotate_right(8) ^ (words[i - 15] >> 7);
        let s1 =
            words[i - 2].rotate_right(19) ^ words[i - 2].rotate_right(61) ^ (words[i - 2] >> 6);
        words.push(
            words[i - 16]
                .wrapping_add(s0)
                .wrapping_add(words[i - 7])
                .wrapping_add(s1),
        );
    }

    // Main loop
    let buffer_n = (0..80).fold(*buffer, |h, i| {
        let (a, b, c, d, e, f, g, h) = (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

        let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
        let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
        let ch = (e & f) ^ (!e & g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K_TABLE_512[i])
            .wrapping_add(words[i]);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(maj);

        [
            temp1.wrapping_add(temp2),
            a,
            b,
            c,
            d.wrapping_add(temp1),
            e,
            f,
            g,
        ]
    });

    for i in 0..8 {
        buffer[i] = buffer[i].wrapping_add(buffer_n[i]);
    }
}

#[cfg(test)]
mod test {
    use crate::algorithms::helpers::test::assert_digest;

    use super::*;

    #[test]
    fn sha224_correct() {
        let sha_224 = Sha2::new(Sha2Variant::_224);

        for (input,expected) in
        [
            ("", "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"),
            ("The quick brown fox jumps over the lazy dog", "730e109bd7a8a32b1cb9d9a09aa2325d2430587ddbc0c38bad911525"),
            ("This is a very long string with the purpose of exceeding the chunk length of 64 bytes", "c0ebfc1f8de0114969f0164ba381bc3cce984e225adfa79011392cc9")
        ] {
            assert_digest(&sha_224, input, expected);
        }
    }

    #[test]
    fn sha256_correct() {
        let sha_256 = Sha2::new(Sha2Variant::_256);

        for (input, expected) in
        [
            ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("The quick brown fox jumps over the lazy dog", "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"),
            ("This is a very long string with the purpose of exceeding the chunk length of 64 bytes", "7ad7a19a23f6f2285256b72b0854d14c80e04fcc2ae1173f1ffeb9df296ee954")
        ] {
            assert_digest(&sha_256, input, expected);
        }
    }

    #[test]
    fn sha384_correct() {
        let sha_384 = Sha2::new(Sha2Variant::_384);

        for (input, expected) in [("",
                "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"),
            ("The quick brown fox jumps over the lazy dog",
                "ca737f1014a48f4c0b6dd43cb177b0afd9e5169367544c494011e3317dbf9a509cb1e5dc1e85a941bbee3d7f2afbc9b1"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "5fa9a4958d5a80f310adb8b272251086117f409625acdfccd315ba7fd43ce6714c15efaa927ae214af79871b893a2fa9")
        ] {
            assert_digest(&sha_384, input, expected);
        }
    }

    #[test]
    fn sha512_correct() {
        let sha_512 = Sha2::new(Sha2Variant::_512);

        for (input, expected) in [("",
                "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"),
            ("The quick brown fox jumps over the lazy dog",
                "07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "4ad3d21be15ceda6dba084544e36d1849f8d3e6a5965d5d8adf0cac416ecafda15839fa7579bde7017fa7c68aef781a5007b048d0f4272a6a93dc290526d542a")
        ] {
            assert_digest(&sha_512, input, expected);
        }
    }

    #[test]
    fn sha512_224_correct() {
        let sha_512_224 = Sha2::new(Sha2Variant::_512_224);

        for (input, expected) in [
            ("",
                "6ed0dd02806fa89e25de060c19d3ac86cabb87d6a0ddd05c333b84f4"),
            ("The quick brown fox jumps over the lazy dog",
                "944cd2847fb54558d4775db0485a50003111c8e5daa63fe722c6aa37"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "467a1e9707042d5374c16c1dbb4e6f16ba1da198f616381bbf6d7806") ] {
            assert_digest(&sha_512_224, input, expected);
        }
    }

    #[test]
    fn sha512_256_correct() {
        let sha_512_256 = Sha2::new(Sha2Variant::_512_256);

        for (input, expected) in [
            ("",
                "c672b8d1ef56ed28ab87c3622c5114069bdd3ad7b8f9737498d0c01ecef0967a"),
            ("The quick brown fox jumps over the lazy dog",
                "dd9d67b371519c339ed8dbd25af90e976a1eeefd4ad3d889005e532fc5bef04d"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "9995b16812727187c944185c5833759f6c73df58aa46248dc0b7763ab5409d33")] {
            assert_digest(&sha_512_256, input, expected);
        }
    }
}

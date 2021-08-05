const S_TABLE_REDUCED: [u8; 16] = [7, 12, 17, 22, 5, 9, 14, 20, 4, 11, 16, 23, 6, 10, 15, 21];
const K_TABLE: [u32; 64] = [
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

pub fn digest(message: String) {
    let mut bytes: Vec<u8> = message.as_bytes().into();
    let appended_len_bytes = ((bytes.len() * 8) as u64).to_le_bytes();

    println!("{:?}", appended_len_bytes);

    // Initial MD buffer
    let mut md_buf: (u32, u32, u32, u32) = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);

    // Append message according to the MD spec
    // - append bits "1000...000" until bit length is 448 mod 512
    // - then append length as a 64 bit int (little endian)
    bytes.push(128);
    while bytes.len() % 64 != 56 { bytes.push(0); }
    for len_byte in appended_len_bytes { bytes.push(len_byte) }

    // Process each 512-bit chunk
    for chunk_i in (0..bytes.len()).step_by(64) {
        let mut words: Vec<u32> = vec![];
        for i in 0..16 {
            let byte_pos = chunk_i + i*4;
            words.push(u32::from_le_bytes(
                [bytes[byte_pos], 
                bytes[byte_pos+1], 
                bytes[byte_pos+2], 
                bytes[byte_pos+3]]))
        }

        let (a_n, b_n, c_n, d_n) = 
        (0..64).fold(md_buf,
        |(a, b, c, d), i: usize| {
            let (f, g) = match i {
                0..=15  => ((b & c) | (!b & d)  , i             ),
                16..=31 => ((d & b) | (!d & c)  , (5*i + 1) % 16),
                32..=47 => (b ^ c ^ d           , (3*i + 5) % 16),
                _       => (c ^ (b | !d)        , (7*i) % 16    )
            };

            let f = f.wrapping_add(a).wrapping_add(k(i)).wrapping_add(words[g]);

            println!("{} {} {} {}", d, b.wrapping_add(leftrotate(f, s(i))), b, c);

            (d, b.wrapping_add(leftrotate(f, s(i))), b, c)
        });

        let (a, b, c, d) = md_buf;
        md_buf = (
            a.wrapping_add(a_n), 
            b.wrapping_add(b_n), 
            c.wrapping_add(c_n), 
            d.wrapping_add(d_n)
        );
    }

    let (a, b, c, d) = md_buf;
    println!("{:08x} {:08x} {:08x} {:08x}", a.to_be(), b.to_be(), c.to_be(), d.to_be());
}

// Returns value in the s-table at index i
fn s(i: usize) -> u8 {
    S_TABLE_REDUCED[4 * (i / 16) + i % 4]
}

fn k(i: usize) -> u32 { K_TABLE[i] }

fn leftrotate(n: u32, amount: u8) -> u32 { (n << amount) | (n >> (32 - amount)) }

mod test {
    use super::*;

    #[test]
    fn correct_s_values() {
        assert_eq!((s(0), s(1), s(2), s(3)), (7, 12, 17, 22));
        assert_eq!((s(24), s(25), s(26), s(27)), (5, 9, 14, 20));
        assert_eq!((s(40), s(41), s(42), s(43)), (4, 11, 16, 23));
        assert_eq!((s(60), s(61), s(62), s(63)), (6, 10, 15, 21));
    }

    #[test]
    fn leftrotate_works() {
        assert_eq!(leftrotate(5, 2), 20);
        assert_eq!(leftrotate(3489705808, 4), 718093);
    }
}
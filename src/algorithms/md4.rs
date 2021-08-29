use crate::data_container::DataType;

type MdBuffer = (u32, u32, u32, u32);

const CHUNK_SIZE: usize = 16;
const INIT_MD_BUFFER: MdBuffer = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);
const S_TABLE_REDUCED: [u8; 12] = [
    3, 7, 11, 19,
    3, 5, 9, 13,
    3, 9, 11, 15
];

pub fn digest(data: DataType) -> std::io::Result<Vec<u8>> {
    let mut md_buf = INIT_MD_BUFFER;

    fn f(x: u32, y: u32, z: u32) -> u32 { x&y | !x&z }
    fn g(x: u32, y: u32, z: u32) -> u32 { x&y | x&z | y&z }
    fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }

    let mut last_chunk = None;
    let mut len: u64 = 0;
    for chunk in data.into_iter(CHUNK_SIZE) {

        let chunk_bytes = chunk?;
        len = len.wrapping_add(chunk_bytes.len() as u64);
        last_chunk = Some(chunk_bytes);
    }

    unimplemented!()
}

fn process_chunk(chunk: Option<Vec<u8>>, md_buffer: &mut MdBuffer) {

}

fn k(i: usize) -> usize {
    let i_norm = i % 16;
    match i {
        0..=15  => i,
        16..=31 => (4*i_norm + i_norm / 4) % 16,
        _       => {
            let a = [0, 2, 1, 3][i_norm/4];
            let b = [0, 2, 1, 3][i_norm%4];
            a + b*4
        },
    }
}

fn s(i: usize) -> u8 { S_TABLE_REDUCED[ i/16 * 4 + i%4 ] }

#[cfg(test)]
mod test {
    use super::*;

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
        let left: Vec<u8> = [0, 5, 10, 15, 16, 21, 26, 31, 32, 37, 42, 47]
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
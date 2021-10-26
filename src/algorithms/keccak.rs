use crate::{algorithms::helpers::{Endianness, exact_64_bit_words}, data_container::DataType};

// References:
// - https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
// - https://keccak.team/keccak_specs_summary.html

type KState = [u8; 200];
type KLanes = [[u64; 5]; 5];

mod step_mapping_funs {
    use super::KLanes;

    pub fn theta(lanes: &mut KLanes) {
        let c: Vec<u64> = (0..5)
                .map(|x| {
                    lanes[x][0] ^ lanes[x][1] ^ lanes[x][2] ^ lanes[x][3] ^ lanes[x][4]
                }).collect();
        let d: Vec<u64> = (0..5)
            .map(|x| c[(x+4) % 5] ^ c[(x+1) % 5].rotate_left(1) ).collect();
    
        for x in 0..5 {
            for y in 0..5 { lanes[x][y] ^= d[x] }
        }
    }
    
    pub fn rho_and_pi(lanes: &mut KLanes) {
        let (mut x, mut y) = (1, 0);
        let mut current = lanes[x][y];
    
        for t in 0..24 {
            let temp = y;
            y = (2*x + 3*y) % 5;
            x = temp;
    
            let temp = lanes[x][y];
            lanes[x][y] = current.rotate_left(((t+1) * (t+2)) / 2);
            current = temp;
        }
    }
    
    pub fn chi(lanes: &mut KLanes) {
        for y in 0..5 {
            let t: Vec<u64> = (0..5).map(|x| lanes[x][y] ).collect();
    
            for x in 0..5 {
                lanes[x][y] = t[x] ^ (!t[(x+1) % 5] & t[(x+2) % 5]);
            }
        }
    }

    pub fn iota(lanes: &mut KLanes, r: &mut u8) {
        for j in 0..7u8 {
            *r = (*r << 1) ^ ((*r >> 7) * 0x71);
            if *r & 2 == 2 {
                lanes[0][0] ^= 1 << ((1 << j) - 1);
            }
        }
    }
}

fn keccak_f_1600_on_lanes(lanes: &mut KLanes) {
    let mut r: u8 = 1;
    use step_mapping_funs::*;

    for _round in 0..24 {
        theta(lanes);
        rho_and_pi(lanes);
        chi(lanes);
        iota(lanes, &mut r);
    }
}

fn keccak_f_1600(state: KState) -> KState {
    let mut lanes = state_to_lanes(state);
    keccak_f_1600_on_lanes(&mut lanes);

    lanes_to_state(lanes)
}

fn keccak(r: usize, data: DataType, d_suffix: u8, out_byte_len: usize)
    -> std::io::Result<Vec<u8>> {
    let mut state = [0; 200];

    let r_bytes = r / 8;

    // Absorbing phase
    let mut last_block: Option<Vec<u8>> = None;
    for block in data.into_iter(r_bytes) {
        if last_block.is_some() {
            let last_block = last_block.unwrap();
            for i in 0..r_bytes {
                state[i] ^= last_block[i];
            }
        }

        state = keccak_f_1600(state);

        let block_bytes = block?;
        last_block = Some(block_bytes);
    }

    let mut last_bytes = last_block.unwrap_or_default();
    last_bytes.push(d_suffix);
    while last_bytes.len() % r_bytes != r_bytes - 1 {
        last_bytes.push(0);
    }
    last_bytes.push(0x80);

    for block in last_bytes.chunks(r_bytes) {
        for i in 0..r_bytes {
            state[i] ^= block[i];
        }

        state = keccak_f_1600(state);
    }

    // Squeezing phase
    let mut out = vec![];
    let mut bytes_left = out_byte_len;
    while bytes_left > 0 {
        let block_size = if out_byte_len < r_bytes { out_byte_len } else { r_bytes };
        for i in 0..block_size {
            out.push(state[i]);
        }
        bytes_left -= block_size;
        if bytes_left > 0 {
            state = keccak_f_1600(state);
        }
    }

    Ok(out)
}

fn state_to_lanes(state: KState) -> KLanes {
    let u64_words = exact_64_bit_words(&state.to_vec(), Endianness::Little);

    let mut lanes = [[0; 5]; 5];
    for x in 0..5 {
        for y in 0..5 {
            lanes[x][y] = u64_words[x + 5*y];
        }
    }

    lanes
}

fn lanes_to_state(lanes: KLanes) -> KState {
    let mut u64_words = [0; 25];
    for x in 0..5 { 
        for y in 0..5 {
            u64_words[x + 5*y] = lanes[x][y]
        }
    }

    let mut state = [0; 200];
    for i in 0..25 {
        let bytes = u64_words[i].to_le_bytes();
        for j in 0..8 {
            state[8*i + j] = bytes[j];
        }
    }

    state
}

#[cfg(test)]
mod test {
    use super::*;
    use super::keccak;
    use crate::DataType;
    use crate::algorithms::helpers::test_helper::test_digest;

    #[test]
    fn state_to_lanes_conversion() {
        let mut state = [0; 200];
        let zero_zeroth: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        for i in 0..8 {
            state[i] = zero_zeroth[i];
        }

        let two_fourth: [u8; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
        for i in 176..184 {
            state[i] = two_fourth[i-176];
        }

        let lanes = state_to_lanes(state);
        assert_eq!(0x807060504030201, lanes[0][0]);
        assert_eq!(0x102030405060708, lanes[2][4]);
    }

    #[test]
    fn lanes_to_state_conversion() {
        let mut lanes = [[0; 5]; 5];
        lanes[0][0] = 0x807060504030201;
        lanes[2][4] = 0x102030405060708;

        let state = lanes_to_state(lanes);
        println!("{:?}", state);
        assert_eq!(1, state[0]);
        assert_eq!(2, state[1]);
        assert_eq!(3, state[2]);
        assert_eq!(4, state[3]);

        assert_eq!(8, state[176]);
        assert_eq!(7, state[177]);
        assert_eq!(6, state[178]);
        assert_eq!(5, state[179]);
    }

    // Tests for step mapping functions from  https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/SHA3-224_Msg0.pdf
    #[test]
    fn correct_theta() {
        let state = [
            0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0];

        let mut lanes = state_to_lanes(state);
        step_mapping_funs::theta(&mut lanes);

        assert_eq!([
            0x06, 0, 0, 0, 0, 0, 0, 0, 0x07, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0x0C, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            07, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x80, 0x0C, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0x07, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0x0C, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            07, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0, 0, 0, 0, 0, 0, 0, 0x80, 0x0C, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 07, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0x0C, 0, 0, 0, 0, 0, 0, 0 
        ], lanes_to_state(lanes));
    }

    #[test]
    fn correct_rho_and_pi() {
        let state = [
            0x06, 0, 0, 0, 0, 0, 0, 0, 0x07, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0x0C, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            07, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x80, 0x0C, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0x07, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0x0C, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            07, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0, 0, 0, 0, 0, 0, 0, 0x80, 0x0C, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 07, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80,
            0x0C, 0, 0, 0, 0, 0, 0, 0 
        ];

        let mut lanes = state_to_lanes(state);
        step_mapping_funs::rho_and_pi(&mut lanes);

        assert_eq!([
            0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x70, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10, 0, 0, 0, 0, 0,
            0, 0, 0x03, 0, 0, 0, 0, 0, 0, 0, 0, 0x08, 0, 0, 0, 0,
            0, 0, 0xC0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x0E, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0x01, 0, 0, 0, 0, 0, 0x0C, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x60, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1C, 0, 0, 0, 0, 0, 0,
            0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0,
            0, 0, 0, 0, 0, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x1C, 0, 0, 0, 0, 0, 0, 0
        ], lanes_to_state(lanes));
    }

    #[test]
    fn correct_chi() {
        let state = [
            0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x70, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10, 0, 0, 0, 0, 0,
            0, 0, 0x03, 0, 0, 0, 0, 0, 0, 0, 0, 0x08, 0, 0, 0, 0,
            0, 0, 0xC0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x0E, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0x01, 0, 0, 0, 0, 0, 0x0C, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x60, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1C, 0, 0, 0, 0, 0, 0,
            0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0,
            0, 0, 0, 0, 0, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x1C, 0, 0, 0, 0, 0, 0, 0
        ];

        let mut lanes = state_to_lanes(state);
        step_mapping_funs::chi(&mut lanes);

        assert_eq!([
            0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10, 0, 0, 0x70, 0, 0,
            0, 0, 0x03, 0, 0, 0, 0, 0, 0x06, 0, 0x10, 0, 0, 0, 0, 0,
            0, 0, 0x03, 0, 0, 0x70, 0, 0, 0, 0, 0, 0x08, 0, 0, 0, 0,
            0, 0, 0xC0, 0, 0, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0x08, 0, 0xE0, 0, 0, 0, 0, 0xC0, 0, 0, 0, 0, 0,
            0x0E, 0, 0, 0x01, 0, 0, 0, 0, 0, 0x0C, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0x01, 0, 0, 0, 0, 0x0E, 0x0C, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1C, 0, 0x60, 0, 0, 0, 0,
            0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0x1C, 0, 0, 0, 0, 0x80, 0,
            0, 0x40, 0, 0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0,
            0, 0, 0, 0, 0, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0,
            0x1C, 0, 0, 0, 0, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x1C, 0, 0, 0, 0, 0, 0x40, 0,
        ], lanes_to_state(lanes));
    }

    #[test]
    fn correct_iota() {
        let state = [
            0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10, 0, 0, 0x70, 0, 0,
            0, 0, 0x03, 0, 0, 0, 0, 0, 0x06, 0, 0x10, 0, 0, 0, 0, 0,
            0, 0, 0x03, 0, 0, 0x70, 0, 0, 0, 0, 0, 0x08, 0, 0, 0, 0,
            0, 0, 0xC0, 0, 0, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0x08, 0, 0xE0, 0, 0, 0, 0, 0xC0, 0, 0, 0, 0, 0,
            0x0E, 0, 0, 0x01, 0, 0, 0, 0, 0, 0x0C, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0x01, 0, 0, 0, 0, 0x0E, 0x0C, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1C, 0, 0x60, 0, 0, 0, 0,
            0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0x1C, 0, 0, 0, 0, 0x80, 0,
            0, 0x40, 0, 0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0,
            0, 0, 0, 0, 0, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0,
            0x1C, 0, 0, 0, 0, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x1C, 0, 0, 0, 0, 0, 0x40, 0,
        ];

        let mut lanes = state_to_lanes(state);
        let mut r = 1;
        step_mapping_funs::iota(&mut lanes, &mut r);

        assert_eq!([
            0x07, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10, 0, 0, 0x70, 0, 0,
            0, 0, 0x03, 0, 0, 0, 0, 0, 0x06, 0, 0x10, 0, 0, 0, 0, 0,
            0, 0, 0x03, 0, 0, 0x70, 0, 0, 0, 0, 0, 0x08, 0, 0, 0, 0,
            0, 0, 0xC0, 0, 0, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0x08, 0, 0xE0, 0, 0, 0, 0, 0xC0, 0, 0, 0, 0, 0,
            0x0E, 0, 0, 0x01, 0, 0, 0, 0, 0, 0x0C, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0x01, 0, 0, 0, 0, 0x0E, 0x0C, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1C, 0, 0x60, 0, 0, 0, 0,
            0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0x1C, 0, 0, 0, 0, 0x80, 0,
            0, 0x40, 0, 0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0,
            0, 0, 0, 0, 0, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0,
            0x1C, 0, 0, 0, 0, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x1C, 0, 0, 0, 0, 0, 0x40, 0,
        ], lanes_to_state(lanes));
    }

    #[test]
    fn keccak_f_1600_correct() {
        let mut lanes: KLanes = [[0; 5]; 5];
        lanes[0][0] = 0x06;
        lanes[2][3] = 0x8000000000000000;

        keccak_f_1600_on_lanes(&mut lanes);
        assert_eq!(0xb7db673642034e6b, lanes[0][0]);
    }

    fn sha3_224_test(data: DataType) -> std::io::Result<Vec<u8>> {
        keccak(1152, data, 0x06, 224/8)
    }

    fn keccak_test() {
        test_digest(&sha3_224_test, &[
            ("",    "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7"),
            ("abc", "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf")
        ]);
    }
}
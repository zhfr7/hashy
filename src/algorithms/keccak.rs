use crate::{algorithms::helpers::{Endianness, exact_64_bit_words}, data_container::DataType};

// References:
// - https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
// - https://keccak.team/keccak_specs_summary.html

type KState = [u8; 200];
type KLanes = [[u64; 5]; 5];

fn keccak_f_1600_on_lanes(lanes: &mut KLanes) {
    let mut r: u8 = 1;

    for round in 0..24 {
        // phi
        let c: Vec<u64> = (0..5)
            .map(|x| {
                lanes[x][0] ^ lanes[x][1] ^ lanes[x][2] ^ lanes[x][3] ^ lanes[x][4]
            }).collect();
        let d: Vec<u64> = (0..5)
            .map(|x| c[(x+4) % 5] ^ c[(x+1) % 5].rotate_left(1) ).collect();

        for x in 0..5 {
            for y in 0..5 { lanes[x][y] ^= d[x] }
        }
        
        // rho and pi
        let (mut x, mut y) = (1, 0);
        let mut current = lanes[x][y];

        for t in 0..24 {
            let temp = y;
            y = (2*x + 3*y) % 5;
            x = temp;

            lanes[x][y] = current.rotate_left(((t+1) * (t+2)) / 2);
            current = lanes[x][y];
        }

        // chi
        for y in 0..5 {
            let t: Vec<u64> = (0..5).map(|x| lanes[x][y] ).collect();

            for x in 0..5 {
                lanes[x][y] = t[x] ^ (!t[(x+1) % 5] & t[(x+2) % 5]);
            }
        }

        // iota
        for j in 0..7u8 {
            r = (r << 1) ^ ((r >> 7) * 0x71);
            if r & 2 == 2 {
                lanes[0][0] ^= 1 << ((1 << j) - 1);
            }
        }
    }
}

fn state_to_lanes(state: KState) -> KLanes {
    let u64_words = exact_64_bit_words(&state.to_vec(), Endianness::Big);

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
        let bytes = u64_words[i].to_be_bytes();
        for j in 0..8 {
            state[8*i + j] = bytes[j];
        }
    }

    state
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DataType;
    use crate::post_process::*;

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
        assert_eq!(0x102030405060708, lanes[0][0]);
        assert_eq!(0x807060504030201, lanes[2][4]);
    }

    #[test]
    fn lanes_to_state_conversion() {
        let mut lanes = [[0; 5]; 5];
        lanes[0][0] = 0x102030405060708;
        lanes[2][4] = 0x807060504030201;

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
}
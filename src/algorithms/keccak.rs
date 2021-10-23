use crate::{algorithms::helpers::{Endianness, exact_64_bit_words}, data_container::DataType};

// References:
// - https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
// - https://keccak.team/keccak_specs_summary.html

type Lanes = [[u64; 5]; 5];

const ROTATION_OFFSETS: [[u32; 5]; 5] = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14]
];

const ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082,
    0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088,
    0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B,
    0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080,
    0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080,
    0x0000000080000001, 0x8000000080008008,
];

struct KeccakState {
    lanes: Lanes
}

impl KeccakState {
    pub fn new() -> Self {
        KeccakState { 
            lanes: [[0; 5]; 5]
        }
    }
    
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        let mut out = KeccakState::new();
        let mut i = 0;

        for y in 0..5 {
            for x in 0..5 {
                out.lanes[x][y] = u64::from_be_bytes(
                    [bytes[i], bytes[i+1], bytes[i+2], bytes[i+3],
                    bytes[i+4], bytes[i+5], bytes[i+6], bytes[i+7]]
                );

                i += 8;
            }
        }

        out
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![];

        for y in 0..5 {
            for x in 0..5 {
                let mut lane = self.lanes[x][y].to_be_bytes().to_vec();
                out.append(&mut lane);
            }
        }

        out
    }
}

fn keccak(r: usize, message: DataType, d: u8, output_len: usize)
    -> std::io::Result<Vec<u8>> {
    let mut state = KeccakState::new();

    let mut last_block = None;
    let mut len_mod_r = 0;

    // Absorbing phase
    for block in message.into_iter(r/8) {
        keccak_process_block(last_block, r, &mut state);
        
        let block_bytes = block?;
        len_mod_r = (len_mod_r + block_bytes.len()*8) % r;
        last_block = Some(block_bytes);
    }

    let mut remaining = last_block.unwrap_or_default();

    // Pad with d, then pad with 00...0001
    remaining.push(d);
    len_mod_r = (len_mod_r + 8) % r;

    while len_mod_r != 0 {
        remaining.push(0);
        len_mod_r = (len_mod_r + 8) % r;
    }
    *remaining.last_mut().unwrap() = 0x80;

    for rem_block in remaining.chunks_exact(r/8) {
        keccak_process_block(Some(rem_block.to_owned()), r, &mut state);
    }

    // Squeezing phase
    let mut out = vec![];
    while out.len() < output_len / 8 {
        for x in 0..5 {
            for y in 0..5 {
                if x + 5*y >= r/64 { break; }

                let mut bytes = state.lanes[x][y].to_be_bytes().to_vec();
                out.append(&mut bytes);
            }
        }

        keccak_f_1600(&mut state);
    }

    Ok(out)
}

fn keccak_process_block(block: Option<Vec<u8>>, r: usize, state: &mut KeccakState) {
    if block.is_none() { return; }

    let block = block.unwrap();

    let block_lanes = exact_64_bit_words(&block, Endianness::Big);

    for x in 0..5 {
        for y in 0..5 {
            if x + 5*y >= r/64 { break; }

            state.lanes[x][y] ^= block_lanes[x + 5*y];
        }
    }

    keccak_f_1600(state);
}

/// Represents the Keccak-f[1600] permutation function with 24 rounds.
/// Mutates the given state argument.
fn keccak_f_1600(state: &mut KeccakState) {
    for i in 0..24 {
        round(state, ROUND_CONSTANTS[i]);
    }
}

fn round(state: &mut KeccakState, round_constant: u64) {
    // phi
    let c: Vec<u64> = (0..5).map(|x| {
        state.lanes[x][0] ^ 
        state.lanes[x][1] ^ 
        state.lanes[x][2] ^ 
        state.lanes[x][3] ^ 
        state.lanes[x][4]
    }).collect();

    let d: Vec<u64> = (0..5).map(|x| {
        c[(x+4) % 5] ^ c[(x+1) % 5].rotate_right(1)
    }).collect();

    for x in 0..5 {
        for y in 0..5 {
            state.lanes[x][y] ^= d[x];
        }
    }

    // rho and pi
    let mut b = [[0; 5]; 5];
    for x in 0..5 {
        for y in 0..5 {
            b[y][(2*x + 3*y) % 5] = 
                state.lanes[x][y].rotate_right(ROTATION_OFFSETS[x][y]);
        }
    }

    // chi
    for x in 0..5 {
        for y in 0..5 {
            state.lanes[x][y] = b[x][y] ^ (!b[(x+1) % 5][y] & b[(x+2) % 5][y]);
        }
    }

    // iota
    state.lanes[0][0] ^= round_constant;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DataType;
    use crate::post_process::*;

    #[test]
    fn from_bytes_correct() {
        let mut bytes = vec![128, 0, 0, 0, 0, 9, 0, 128];
        for _ in 0..192 { bytes.push(0) }

        let state = KeccakState::from_bytes(&bytes);
        println!("{:x}", state.lanes[0][0]);
        assert_eq!(state.lanes[0][0], 0x8000000000090080);
    }

    #[test]
    fn to_bytes_correct() {
        let mut bytes = vec![128, 0, 0, 0, 0, 9, 0, 128];
        for _ in 0..192 { bytes.push(0) }

        let state = KeccakState::from_bytes(&bytes);
        assert_eq!(state.to_bytes(), bytes);
    }

    #[test]
    fn keccak_returns_correctly() {
        let data = DataType::Bytes("".as_bytes().to_vec());
        let digest_bytes = keccak(1152, data, 0x06, 224).unwrap();
        let digest = encode(digest_bytes, Encoding::Hex(false));

        assert_eq!("6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7", digest);
    }
}
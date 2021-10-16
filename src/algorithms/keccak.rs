// References:
// - https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
// - https://github.com/ctz/keccak/blob/master/keccak.py

struct KeccakState {
    lanes: [[u64; 5]; 5]
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
                    [
                        bytes[i],
                        bytes[i+1],
                        bytes[i+2],
                        bytes[i+3],
                        bytes[i+4],
                        bytes[i+5],
                        bytes[i+6],
                        bytes[i+7],
                    ]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_bytes_correct() {
        let mut bytes = vec![128, 0, 0, 0, 0, 9, 0, 128];
        for _ in 0..192 { bytes.push(0) }

        let keccak = KeccakState::from_bytes(&bytes);
        println!("{:x}", keccak.lanes[0][0]);
        assert_eq!(keccak.lanes[0][0], 0x8000000000090080);
    }

    #[test]
    fn to_bytes_correct() {
        let mut bytes = vec![128, 0, 0, 0, 0, 9, 0, 128];
        for _ in 0..192 { bytes.push(0) }

        let keccak = KeccakState::from_bytes(&bytes);
        assert_eq!(keccak.to_bytes(), bytes);
    }
}
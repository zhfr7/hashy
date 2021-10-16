// References:
// - https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
// - https://github.com/ctz/keccak/blob/master/keccak.py

type Lanes = [[u64; 5]; 5];

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

    // Step mapping functions

    pub fn phi(&mut self) {
        let mut c = vec![];
        let mut d = vec![];

        for x in 0..5 {
            c.push(self.lanes[x].iter().fold(0, 
                |current, lane| current & lane));
        }

        for x in 0..5 {
            d.push(c[(x-1) % 5] ^ c[(x+1) % 5].rotate_right(1));
        }

        for x in 0..5 {
            for y in 0..5 {
                self.lanes[x][y] ^= d[x];
            }
        }
    }

    pub fn rho(&mut self) {
        let (mut x, mut y) = (1, 0);

        for t in 0..24 {
            self.lanes[x][y] = self.lanes[x][y].rotate_right((t+1) * (t+2) / 2);
            let temp = y;
            y = (2*x + 3*y) % 5;
            x = temp;
        }
    }

    pub fn pi(&mut self) {
        for x in 0..5 {
            for y in 0..5 {
                self.lanes[x][y] = self.lanes[(x + 3*y) % 5][x];
            }
        }
    }

    pub fn chi(&mut self) {
        let mut new_lanes = [[0; 5]; 5];

        for x in 0..5 {
            for y in 0..5 {
                new_lanes[x][y] = self.lanes[x][y] ^ 
                    ((self.lanes[(x+1) % 5][y] ^ u64::MAX) & self.lanes[(x+2) % 5][y]);
            }
        }

        self.lanes = new_lanes;
    }
    
    // Round constant function for iota
    pub fn rc(t: usize) {

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
/// Encoding types
#[derive(Debug)]
pub enum Encoding {
    /// Hexadecimal encoding with Hex(is_uppercase)
    Hex(bool),
    /// Base64 encoding using the standard character set with '=' padding
    Base64,
    /// Binary encoding
    Binary
}

impl Encoding {
    pub fn from_str(encoding: &str) -> Result<Encoding, String> {
        let enc_lower = encoding.to_lowercase();
        match enc_lower.as_str() {
            "hex"       => Ok(Encoding::Hex(false)),
            "hex_upper" => Ok(Encoding::Hex(true)),
            "base64"    => Ok(Encoding::Base64),
            "bin"       => Ok(Encoding::Binary),
            _           => Err(format!("Unknown encoding type: {}", enc_lower))
        }
    }

    /// Encodes the given bytes according to the encoding type given
    pub fn encode(&self, bytes: Vec<u8>) -> String {
        match self {
            Encoding::Hex(false)    => hex::encode(bytes),
            Encoding::Hex(true)     => hex::encode_upper(bytes),
            Encoding::Base64        => base64::encode(bytes),
            Encoding::Binary        => {
                bytes.into_iter()
                .map(|byte| {
                    let mut cur_value = byte;
                    let mut cur_bitstring = String::new();

                    for _ in 0..8 {
                        cur_bitstring.insert(0, 
                            if cur_value % 2 == 0 { '0' } else { '1' });

                        cur_value /= 2;
                    }

                    cur_bitstring
                })
                .collect::<Vec<String>>()
                .join("")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_hex() {
        assert_eq!(Encoding::Hex(false).encode(vec![]), 
            "".to_string());
        assert_eq!(Encoding::Hex(false).encode(vec![1, 2, 3, 25, 26, 129]), 
            "010203191a81".to_string());
        assert_eq!(Encoding::Hex(true).encode(vec![1, 2, 3, 25, 26, 129]), 
            "010203191A81".to_string());
    }

    #[test]
    fn encode_base64() {
        assert_eq!(Encoding::Base64.encode(vec![]), 
            "".to_string());
        assert_eq!(Encoding::Base64.encode(vec![1, 2, 3, 25, 26, 129]), 
            "AQIDGRqB".to_string());
        assert_eq!(Encoding::Base64.encode(vec![1, 2, 3, 25, 26, 129, 130]), 
            "AQIDGRqBgg==".to_string());
    }
    
    #[test]
    fn encode_binary() {
        assert_eq!(Encoding::Binary.encode(vec![]),
            "".to_string());
        assert_eq!(Encoding::Binary.encode(vec![197, 209, 3]), 
            "110001011101000100000011".to_string());
    }
}

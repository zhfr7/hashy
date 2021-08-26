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

/// Encodes the given bytes according to the encoding type given
pub fn encode(bytes: Vec<u8>, encoding: Encoding) -> String {
    match encoding {
        Encoding::Hex(false)    => hex::encode(bytes),
        Encoding::Hex(true)     => hex::encode_upper(bytes),
        Encoding::Base64        => base64::encode(bytes),
        Encoding::Binary        => encode_binary(bytes)
    }
}

fn encode_binary(bytes: Vec<u8>) -> String {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_hex() {
        assert_eq!(encode(vec![], Encoding::Hex(false)), 
            "".to_string());
        assert_eq!(encode(vec![1, 2, 3, 25, 26, 129], Encoding::Hex(false)), 
            "010203191a81".to_string());
        assert_eq!(encode(vec![1, 2, 3, 25, 26, 129], Encoding::Hex(true)), 
            "010203191A81".to_string());
    }

    #[test]
    fn encode_base64() {
        assert_eq!(encode(vec![], Encoding::Base64), 
            "".to_string());
        assert_eq!(encode(vec![1, 2, 3, 25, 26, 129], Encoding::Base64), 
            "AQIDGRqB".to_string());
        assert_eq!(encode(vec![1, 2, 3, 25, 26, 129, 130], Encoding::Base64), 
            "AQIDGRqBgg==".to_string());
    }
    
    #[test]
    fn encode_binary() {
        assert_eq!(encode(vec![], Encoding::Binary),
            "".to_string());
        assert_eq!(encode(vec![197, 209, 3], Encoding::Binary), 
            "110001011101000100000011".to_string());
    }
}

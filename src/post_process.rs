pub enum Encoding {
    Hex(bool),
    Base64,
    Binary
}

pub fn encode(bytes: Vec<u8>, encoding: Encoding) -> String {
    String::new()
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

// Reference: https://keccak.team/keccak_specs_summary.html

use crate::data_container::DataType;
use super::helpers::DigestResult;
use super::keccak::keccak;

/// Generates a digest for the SHA3-\[out_len\] algorithm variant,
/// where out_len is the intended output size in bits.
/// 
/// out_len is constrained to 224, 256, 384, 512. Other values
/// would result in an Err(...).
pub fn digest(data: DataType, out_len: usize) -> DigestResult {
    if ![224, 256, 384, 512].contains(&out_len) { 
        return Err(anyhow::anyhow!("Impl error: invalid output size")); }
    
    keccak(1600 - 2*out_len, data, 0x06, out_len)
}

/// Generates a SHAKE-128 digest from the data and the intended
/// output size in bits (out_len).
pub fn digest_shake_128(data: DataType, out_len: usize) -> DigestResult {
    keccak(1344, data, 0x1F, out_len)
}

/// Generates a SHAKE-256 digest from the data and the intended
/// output size in bits (out_len).
pub fn digest_shake_256(data: DataType, out_len: usize) -> DigestResult {
    keccak(1088, data, 0x1F, out_len)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_digest;

    // Helper functions

    fn sha3_224(data: DataType) -> DigestResult { digest(data, 224) }
    fn sha3_256(data: DataType) -> DigestResult { digest(data, 256) }
    fn sha3_384(data: DataType) -> DigestResult { digest(data, 384) }
    fn sha3_512(data: DataType) -> DigestResult { digest(data, 512) }

    #[test]
    fn sha3_224_correct() {
        test_digest!(sha3_224,
            ("",    "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7"),
            ("abc", "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf"),
            ("The quick brown fox jumps over the lazy dog",
                "d15dadceaa4d5d7bb3b48f446421d542e08ad8887305e28d58335795"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "c11e6c2cb2c6f9a50f44d47abaf8bc252ace77956aeb347a3432fb5b")
        );
    }

    #[test]
    fn sha3_256_correct() {
        test_digest!(sha3_256,
            ("",    "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"),
            ("abc", "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"),
            ("The quick brown fox jumps over the lazy dog",
                "69070dda01975c8c120c3aada1b282394e7f032fa9cf32f4cb2259a0897dfc04"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "4099a6836b83a7ec3960e6b881648722b905efc4b988dcf1d7096e60d2ec683a")
        );
    }

    #[test]
    fn sha3_384_correct() {
        test_digest!(sha3_384,
            ("",    
                "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004"),
            ("abc", 
                "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25"),
            ("The quick brown fox jumps over the lazy dog",
                "7063465e08a93bce31cd89d2e3ca8f602498696e253592ed26f07bf7e703cf328581e1471a7ba7ab119b1a9ebdf8be41"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "d5fd54ed091312fb4701d91522ce8e44bb97d20693900280fe21cf9be91c83d4cb8f4b7e159350bfed1434f5824184cc")
        );
    }

    #[test]
    fn sha3_512_correct() {
        test_digest!(sha3_512,
            ("",    
                "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"),
            ("abc", 
                "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"),
            ("The quick brown fox jumps over the lazy dog",
                "01dedd5de4ef14642445ba5f5b97c15e47b9ad931326e4b0727cd94cefc44fff23f07bf543139939b49128caf436dc1bdee54fcb24023a08d9403f9b4bf0d450"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "2eece363538ddf64224699bf4f4757d65e0fd050ce591e15c468f653686cf2d3bab634eb27d6ac1fdd780faefbab598f0fd7f2e97d813bb2789e0421419fbc03")
        );
    }

    fn shake_128_helper_1(data: DataType) -> DigestResult { digest_shake_128(data, 64) }
    fn shake_128_helper_2(data: DataType) -> DigestResult { digest_shake_128(data, 184) }
    
    fn shake_256_helper_1(data: DataType) -> DigestResult { digest_shake_256(data, 72) }
    fn shake_256_helper_2(data: DataType) -> DigestResult { digest_shake_256(data, 240) }

    #[test]
    fn shake_128_correct() {
        test_digest!(shake_128_helper_1, 
            ("", "7f9c2ba4e88f827d"),
            ("abc", "5881092dd818bf5c"),
            ("The quick brown fox jumps over the lazy dog",
                "f4202e3c5852f918"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "2a59b3f15b0843e6")
        );

        test_digest!(shake_128_helper_2, 
            ("", "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc"),
            ("abc", "5881092dd818bf5cf8a3ddb793fbcba74097d5c526a6d3"),
            ("The quick brown fox jumps over the lazy dog",
                "f4202e3c5852f9182a0430fd8144f0a74b95e7417ecae1"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "2a59b3f15b0843e697fccb01c389ba6c1b7d9d41222e6e")
        );
    }

    #[test]
    fn shake_128_invalid_param() {
        let data = DataType::Bytes("".as_bytes().to_vec());
        let result = digest_shake_128(data, 183);
        assert!(result.is_err());
    }

    #[test]
    fn shake_256_correct() {
        test_digest!(shake_256_helper_1, 
            ("",    "46b9dd2b0ba88d1323"),
            ("abc", "483366601360a8771c"),
            ("The quick brown fox jumps over the lazy dog",
                "2f671343d9b2e1604d"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "bb059eb64e47b28acb")
        );

        test_digest!(shake_256_helper_2, 
            ("",    "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5"),
            ("abc", "483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b"),
            ("The quick brown fox jumps over the lazy dog",
                "2f671343d9b2e1604dc9dcf0753e5fe15c7c64a0d283cbbf722d411a0e36"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "bb059eb64e47b28acbbf71617f90139ce7c81cd0ca09d70d23979b1c7e24")
        );
    }

    #[test]
    fn shake_256_invalid_param() {
        let data = DataType::Bytes("".as_bytes().to_vec());
        let result = digest_shake_256(data, 201);
        assert!(result.is_err());
    }
}
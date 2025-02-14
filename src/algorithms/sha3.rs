use super::keccak::keccak;
use super::{Algorithm, DigestResult};
use crate::chunked_stream::ChunkedStream;

pub enum Sha3Variant {
    _224,
    _256,
    _384,
    _512,
}

pub enum ShakeVariant {
    _128,
    _256,
}

/// SHA3 (Secure Hash Algorithm 3), includes all variants:
///
/// - SHA3-224
/// - SHA3-256
/// - SHA3-384
/// - SHA3-512
///
/// References:
/// - https://keccak.team/keccak_specs_summary.html
pub struct Sha3 {
    variant: Sha3Variant,
}

/// SHAKE-n (Secure Hash Algorithm Keccak) with n=128, 256.
/// Variable output length.
///
/// References:
/// - https://keccak.team/keccak_specs_summary.html
/// - https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf
pub struct Shake {
    variant: ShakeVariant,
    output_length: usize,
}

#[derive(Debug)]
pub struct InvalidOutputLength;

impl Sha3 {
    pub fn new(variant: Sha3Variant) -> Self {
        Self { variant }
    }
}

impl Shake {
    pub fn new(variant: ShakeVariant, output_length: usize) -> Result<Self, InvalidOutputLength> {
        if output_length % 8 == 0 {
            Ok(Self {
                variant,
                output_length,
            })
        } else {
            Err(InvalidOutputLength)
        }
    }
}

impl Algorithm for Sha3 {
    fn digest(&self, data: ChunkedStream) -> DigestResult {
        let output_length = match self.variant {
            Sha3Variant::_224 => 224,
            Sha3Variant::_256 => 256,
            Sha3Variant::_384 => 384,
            Sha3Variant::_512 => 512,
        };
        let bitrate = 1600 - 2 * output_length;

        keccak(bitrate, data, 0x06, output_length)
    }
}

impl Algorithm for Shake {
    fn digest(&self, data: ChunkedStream) -> DigestResult {
        let bitrate = match self.variant {
            ShakeVariant::_128 => 1344,
            ShakeVariant::_256 => 1088,
        };

        keccak(bitrate, data, 0x1F, self.output_length)
    }
}

#[cfg(test)]
mod test {
    use crate::algorithms::helpers::test::assert_digest;

    use super::*;

    #[test]
    fn sha3_224_correct() {
        let sha3_224 = Sha3::new(Sha3Variant::_224);

        for (input, expected) in [
            ("",    "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7"),
            ("abc", "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf"),
            ("The quick brown fox jumps over the lazy dog",
                "d15dadceaa4d5d7bb3b48f446421d542e08ad8887305e28d58335795"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "c11e6c2cb2c6f9a50f44d47abaf8bc252ace77956aeb347a3432fb5b")
        ] {
            assert_digest(&sha3_224, input, expected);
        }
    }

    #[test]
    fn sha3_256_correct() {
        let sha3_256 = Sha3::new(Sha3Variant::_256);

        for (input, expected) in [
            ("",    "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"),
            ("abc", "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"),
            ("The quick brown fox jumps over the lazy dog",
                "69070dda01975c8c120c3aada1b282394e7f032fa9cf32f4cb2259a0897dfc04"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "4099a6836b83a7ec3960e6b881648722b905efc4b988dcf1d7096e60d2ec683a")
        ] {
            assert_digest(&sha3_256, input, expected);
        }
    }

    #[test]
    fn sha3_384_correct() {
        let sha3_384 = Sha3::new(Sha3Variant::_384);

        for (input, expected) in [
            ("",
                "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004"),
            ("abc",
                "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25"),
            ("The quick brown fox jumps over the lazy dog",
                "7063465e08a93bce31cd89d2e3ca8f602498696e253592ed26f07bf7e703cf328581e1471a7ba7ab119b1a9ebdf8be41"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "d5fd54ed091312fb4701d91522ce8e44bb97d20693900280fe21cf9be91c83d4cb8f4b7e159350bfed1434f5824184cc")
        ] {
            assert_digest(&sha3_384, input, expected);
        }
    }

    #[test]
    fn sha3_512_correct() {
        let sha3_512 = Sha3::new(Sha3Variant::_512);

        for (input, expected) in [
            ("",
                "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"),
            ("abc",
                "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"),
            ("The quick brown fox jumps over the lazy dog",
                "01dedd5de4ef14642445ba5f5b97c15e47b9ad931326e4b0727cd94cefc44fff23f07bf543139939b49128caf436dc1bdee54fcb24023a08d9403f9b4bf0d450"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "2eece363538ddf64224699bf4f4757d65e0fd050ce591e15c468f653686cf2d3bab634eb27d6ac1fdd780faefbab598f0fd7f2e97d813bb2789e0421419fbc03")
        ] {
            assert_digest(&sha3_512, input, expected);
        }
    }

    #[test]
    fn shake_128_correct() {
        let shake128_64 = Shake::new(ShakeVariant::_128, 64).unwrap();
        let shake128_184 = Shake::new(ShakeVariant::_128, 184).unwrap();

        for (input, expected) in [
            ("", "7f9c2ba4e88f827d"),
            ("abc", "5881092dd818bf5c"),
            ("The quick brown fox jumps over the lazy dog",
                "f4202e3c5852f918"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "2a59b3f15b0843e6")
        ] {
            assert_digest(&shake128_64, input, expected);
        }

        for (input, expected) in [
            ("", "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc"),
            ("abc", "5881092dd818bf5cf8a3ddb793fbcba74097d5c526a6d3"),
            ("The quick brown fox jumps over the lazy dog",
                "f4202e3c5852f9182a0430fd8144f0a74b95e7417ecae1"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "2a59b3f15b0843e697fccb01c389ba6c1b7d9d41222e6e")
        ] {
            assert_digest(&shake128_184, input, expected);
        }
    }

    #[test]
    fn shake_256_correct() {
        let shake256_72 = Shake::new(ShakeVariant::_256, 72).unwrap();
        let shake256_240 = Shake::new(ShakeVariant::_256, 240).unwrap();

        for (input, expected) in [
            ("",    "46b9dd2b0ba88d1323"),
            ("abc", "483366601360a8771c"),
            ("The quick brown fox jumps over the lazy dog",
                "2f671343d9b2e1604d"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "bb059eb64e47b28acb")
        ] {
            assert_digest(&shake256_72, input, expected);
        }

        for (input, expected) in [
            ("",    "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5"),
            ("abc", "483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b"),
            ("The quick brown fox jumps over the lazy dog",
                "2f671343d9b2e1604dc9dcf0753e5fe15c7c64a0d283cbbf722d411a0e36"),
            ("This is a very long string with the purpose of exceeding the chunk length of 128 bytes, which can be a bit of a pain to write but whatever I guess",
                "bb059eb64e47b28acbbf71617f90139ce7c81cd0ca09d70d23979b1c7e24")
        ] {
            assert_digest(&shake256_240, input, expected);
        }
    }

    #[test]
    fn shake_invalid_output_length() {
        let result = Shake::new(ShakeVariant::_128, 65);

        assert!(result.is_err());
    }
}

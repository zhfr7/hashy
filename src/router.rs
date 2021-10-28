use crate::algorithms::*;
use crate::DataType;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Algorithm {
    #[strum(serialize = "md2")]
    MD2,
    #[strum(serialize = "md4")]
    MD4,
    #[strum(serialize = "md5")]
    MD5,
    #[strum(serialize = "sha1")]
    SHA1,
    #[strum(serialize = "sha-224")]
    SHA2_224,
    #[strum(serialize = "sha-256", serialize = "sha2")]
    SHA2_256,
    #[strum(serialize = "sha-384")]
    SHA2_384,
    #[strum(serialize = "sha-512")]
    SHA2_512,
    #[strum(serialize = "sha-512-224")]
    SHA2_512_224,
    #[strum(serialize = "sha-512-256")]
    SHA2_512_256,
    #[strum(serialize = "sha3-256")]
    SHA3_256
}

impl Algorithm {
    pub fn digest(&self, data: DataType) -> Result<Vec<u8>, anyhow::Error> {
        match &self {
            Self::MD2 => md2::digest(data),
            Self::MD4 => md4::digest(data),
            Self::MD5 => md5::digest(data),
            Self::SHA1 => sha1::digest(data),
            Self::SHA2_224 => sha2::digest_224(data),
            Self::SHA2_256 => sha2::digest_256(data),
            Self::SHA2_384 => sha2::digest_384(data),
            Self::SHA2_512 => sha2::digest_512(data),
            Self::SHA2_512_224 => sha2::digest_512_224(data),
            Self::SHA2_512_256 => sha2::digest_512_256(data),
            Self::SHA3_256 => Ok(dummy())
        }
    }
}

// Used for testing unimplemented algorithms
fn dummy() -> Vec<u8> { vec![0, 1, 2, 3] }
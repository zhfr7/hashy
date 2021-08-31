use crate::algorithms::*;
use crate::data_container::DataType;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Algorithm {
    #[strum(serialize = "md2")]
    MD2,
    #[strum(serialize = "md4")]
    MD4,
    #[strum(serialize = "md5")]
    MD5,
    #[strum(serialize = "sha3-256")]
    SHA3_256
}

impl Algorithm {
    pub fn digest(&self, data: DataType) -> std::io::Result<Vec<u8>> {
        match &self {
            Self::MD2 => md2::digest(data),
            Self::MD4 => md4::digest(data),
            Self::MD5 => md5::digest(data),
            Self::SHA3_256 => Ok(dummy())
        }
    }
}

// Used for testing unimplemented algorithms
fn dummy() -> Vec<u8> { vec![0, 1, 2, 3] }
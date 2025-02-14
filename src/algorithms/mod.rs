use std::io;

use crate::chunked_stream::ChunkedStream;

pub mod md2;
pub mod md4;
pub mod md5;
pub mod sha1;
pub mod sha2;
pub mod sha3;

mod helpers;
mod keccak;

type DigestResult = Result<Vec<u8>, io::Error>;

pub trait Algorithm {
    fn digest(&self, data: ChunkedStream) -> DigestResult;
}

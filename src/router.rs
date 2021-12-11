use crate::algorithms::*;
use crate::chunked_stream::ChunkedStream;

pub fn digest_from_algorithm(data: ChunkedStream, algorithm: &String) -> Result<Vec<u8>, anyhow::Error> {
    let algo_lower = algorithm.to_lowercase();
    let algo_specs: Vec<&str> = algo_lower.split('-').collect();

    match algo_specs.as_slice() {
        ["md2"]         => md2::digest(data),
        ["md4"]         => md4::digest(data),
        ["md5"]         => md5::digest(data),
        ["sha1"]        => sha1::digest(data),
        ["sha", "224"]  => sha2::digest(data, 256, 224), 
        ["sha", "256"] | ["sha2"] => sha2::digest(data, 256, 256), 
        ["sha", "384"]  => sha2::digest(data, 512, 384),
        ["sha", "512"]  => sha2::digest(data, 512, 512),
        ["sha", "512", "224"]   => sha2::digest(data, 512, 224),
        ["sha", "512", "256"]   => sha2::digest(data, 512, 256),
        ["sha3", "224"] => sha3::digest(data, 224),
        ["sha3", "256"] => sha3::digest(data, 256),
        ["sha3", "384"] => sha3::digest(data, 384),
        ["sha3", "512"] => sha3::digest(data, 512),
        ["shake128", len] => {
            let out_len: usize = len.parse()?;
            sha3::digest_shake_128(data, out_len)
        },
        ["shake256", len] => {
            let out_len: usize = len.parse()?;
            sha3::digest_shake_256(data, out_len)
        }

        other => Err(anyhow::anyhow!(
            format!("Algorithm {} is currently unimplemented or non-existent", other.join("-"))
        ))
    }
}
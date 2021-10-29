use crate::algorithms::*;
use crate::DataType;

pub fn digest_from_algorithm(data: DataType, algorithm: String) -> Result<Vec<u8>, anyhow::Error> {
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
        other => Err(anyhow::anyhow!(
            format!("algorithm {} cannot be found", other.join("-"))
        ))
    }
}
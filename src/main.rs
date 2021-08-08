mod algorithms;
mod chunked_stream;
mod post_process;

use std::io;
use post_process::*;

fn main() -> io::Result<()> {
    let string = "The quick brown fox jumps over the lazy dog".to_string();
    let data = chunked_stream::DataType::Bytes(string.as_bytes().into());
    let result = algorithms::md5::digest(data)?;

    println!("{}", encode(result, Encoding::Hex(true)));

    Ok(())
}

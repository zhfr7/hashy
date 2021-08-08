mod algorithms;
mod chunked_stream;
mod post_process;

use std::io;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let string = "The quick brown fox jumps over the lazy dog".to_string();
    let data = chunked_stream::DataType::Bytes(string.as_bytes().into());
    let result = algorithms::md5::digest(data)?;

    for byte in result { print!("{} ", byte) }
    println!();

    Ok(())
}

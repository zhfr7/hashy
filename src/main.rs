#[path = "algorithms/md5.rs"]
mod md5;

fn main() {
    println!("Hello, world!");
    md5::digest("The quick brown fox jumps over the lazy dog".to_string());
}

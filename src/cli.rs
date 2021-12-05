use std::fs::File; 
use std::io::BufReader;

use crate::post_process::Encoding;
use crate::chunked_stream::ChunkedStream;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Does the input denote a filepath?
    #[structopt(short, long)]
    pub file: bool,

    /// Input string
    #[structopt()]
    pub input: String,

    /// Chosen algorithm name, must be present
    #[structopt(short, long)]
    pub algorithm: String,

    /// Encoding type for output hash
    #[structopt(short, long, 
        parse(try_from_str = Encoding::from_str), 
        default_value = "hex")]
    pub encoding: Encoding
}

impl Opts {
    /// Processes the Opts struct (consumes it) and prints the result
    /// of the digest to stdout
    pub fn process(self) -> anyhow::Result<()> {
        let data = 
        if self.file {
            let file = File::open(self.input)?;
            ChunkedStream::File(BufReader::new(file))
        }
        else { ChunkedStream::Bytes(self.input.as_bytes().to_owned()) };

        let digest_bytes = crate::router::digest_from_algorithm(data, self.algorithm)?;
        let digest_encoded = crate::post_process::encode(digest_bytes, self.encoding);

        println!("{}", digest_encoded);

        Ok(())
    }
}
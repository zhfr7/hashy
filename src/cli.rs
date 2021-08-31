use super::post_process::Encoding;
use super::router::Algorithm;
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
    pub algorithm: Algorithm,

    /// Encoding type for output hash
    #[structopt(short, long, 
        parse(try_from_str = parse_encoding), 
        default_value = "hex")]
    pub encoding: Encoding
}

fn parse_encoding(encoding: &str) -> Result<Encoding, String> {
    let enc_lower = encoding.to_lowercase();
    match enc_lower.as_str() {
        "hex"       => Ok(Encoding::Hex(false)),
        "hex_upper" => Ok(Encoding::Hex(true)),
        "base64"       => Ok(Encoding::Base64),
        "bin"       => Ok(Encoding::Binary),
        _           => Err(format!("Unknown encoding type: {}", enc_lower))
    }
}
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
        parse(try_from_str = Encoding::from_str), 
        default_value = "hex")]
    pub encoding: Encoding
}
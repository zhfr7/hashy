use structopt::StructOpt;

use super::encoding::Encoding;

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Input denotes a filepath (cannot be a directory)
    #[structopt(short, long)]
    pub file: bool,

    /// Prints the list of all the available hashing algorithms
    #[structopt(short, long)]
    pub list: bool,

    /// Chosen hashing algorithm name
    #[structopt(required_unless = "list")]
    pub algorithm: Option<String>,

    /// Input string
    #[structopt(required_unless = "list")]
    pub input: Option<String>,

    /// Encoding type for output hash
    #[structopt(short, long, default_value = "hex")]
    pub encoding: Encoding,

    /// Show verbose output
    #[structopt(short, long)]
    pub verbose: bool,
}

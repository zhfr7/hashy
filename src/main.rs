mod algorithms;
mod data_container;
mod post_process;
mod cli;

use cli::Opts;
use data_container::DataType;
use std::{fs::File, io::{self, BufReader}};
use structopt::StructOpt;

use algorithms::md5;

fn main() -> io::Result<()> {
    let opts = Opts::from_args();
    println!("{:?}", opts);
    
    let data = if opts.file {
        let file = File::open(opts.input)?;
        DataType::File(BufReader::new(file))
    } else {
        DataType::Bytes(opts.input.as_bytes().to_owned())
    };

    let out_bytes = md5::digest(data)?;
    let out_encoded = post_process::encode(out_bytes, opts.encoding);

    println!("{}", out_encoded);

    Ok(())
}

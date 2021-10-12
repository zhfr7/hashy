mod algorithms;
mod cli;
mod data_container;
mod router;
mod post_process;

use cli::Opts;
use data_container::DataType;
use std::{fs::File, io::BufReader};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    println!("{:?}", opts);
    
    let data = if opts.file {
        let file = File::open(opts.input)?;
        DataType::File(BufReader::new(file))
    } else {
        DataType::Bytes(opts.input.as_bytes().to_owned())
    };

    let out_bytes = opts.algorithm.digest(data)?;
    let out_encoded = post_process::encode(out_bytes, opts.encoding);

    println!("{}", out_encoded);

    Ok(())
}

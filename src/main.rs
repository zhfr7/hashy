mod algorithms;
mod data_container;
mod post_process;
mod cli;

use std::io;
use cli::Opts;
use structopt::StructOpt;

fn main() -> io::Result<()> {
    let opt = Opts::from_args();
    println!("{:?}", opt);

    Ok(())
}

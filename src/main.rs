mod algorithms;
mod cli;
mod chunked_stream;
mod post_process;
mod router;

use cli::Opts;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    println!("{:?}", opts);
    
    opts.process()
}

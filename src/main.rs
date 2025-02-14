use std::process;

use anyhow::anyhow;
use cli::{
    command::Command,
    opts::{CommandParseError, Opts},
};
use structopt::StructOpt;

mod algorithms;
mod chunked_stream;
mod cli;

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    if opts.verbose {
        println!("Input options: {:?}", opts);
    }

    let command: Command = opts.try_into().map_err(|err| match err {
        CommandParseError::FileDoesNotExist => anyhow!("Path does not exist!"),
        CommandParseError::PathIsDirectory => anyhow!("Path cannot be a directory!"),
        CommandParseError::InvalidPath(io_err) => anyhow!("Invalid path! {}", io_err),
        CommandParseError::InvalidAlgorithm(parse_error_message) => {
            anyhow!("Invalid algorithm! {}", parse_error_message)
        }
        CommandParseError::InvalidEnvironment => {
            Opts::clap().print_help().unwrap();
            process::exit(2);
        }
        CommandParseError::NotImplemented => {
            anyhow!("Command unimplemented! Please consult the maintainer of this CLI.")
        }
    })?;

    command.execute()
}

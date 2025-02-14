use std::{
    fs::File,
    io::{self, stdin, IsTerminal},
    path::PathBuf,
};

use structopt::StructOpt;

use crate::chunked_stream::ChunkedStream;

use super::{command::Command, encoding::Encoding, parsers::parse_algorithm};

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Prints the list of all the available hashing algorithms
    #[structopt(short, long)]
    pub list: bool,

    /// Chosen hashing algorithm name
    #[structopt(required_unless = "list")]
    pub algorithm: Option<String>,

    /// Path to file to read from, cannot be a directory.
    /// Defaults to stdin if not present.
    #[structopt()]
    pub file_path: Option<PathBuf>,

    /// Text to generate hash from.
    /// Use this option to pass a text instead of through stdin.
    #[structopt(short, long)]
    pub text: Option<String>,

    /// Encoding type for output hash
    #[structopt(short, long, default_value = "hex")]
    pub encoding: Encoding,

    /// Show verbose output
    #[structopt(short, long)]
    pub verbose: bool,
}

pub enum CommandParseError {
    FileDoesNotExist,
    PathIsDirectory,
    InvalidPath(io::Error),
    InvalidAlgorithm(String),
    InvalidEnvironment,
    NotImplemented,
}

fn get_data(opts: &Opts) -> Result<ChunkedStream, CommandParseError> {
    // Use file_path first
    if let Some(path) = &opts.file_path {
        let file = File::open(&path).map_err(|err| match err.kind() {
            io::ErrorKind::NotFound => CommandParseError::FileDoesNotExist,
            _ => CommandParseError::InvalidPath(err),
        })?;

        if path.is_dir() {
            return Err(CommandParseError::PathIsDirectory);
        }

        return Ok(ChunkedStream::from(file));
    }
    // Otherwise use "text" option
    else if let Some(text) = &opts.text {
        return Ok(ChunkedStream::from(text.clone()));
    }
    // Otherwise use stdin
    else {
        if stdin().is_terminal() {
            return Err(CommandParseError::InvalidEnvironment);
        }

        return Ok(ChunkedStream::from(stdin()));
    }
}

impl TryInto<Command> for Opts {
    type Error = CommandParseError;

    fn try_into(self) -> Result<Command, Self::Error> {
        if self.list {
            return Ok(Command::List);
        }

        if let Some(algorithm) = &self.algorithm {
            let data = get_data(&self)?;
            let (_, algorithm) = parse_algorithm(algorithm)
                .map_err(|err| CommandParseError::InvalidAlgorithm(err.to_string()))?;

            return Ok(Command::Digest {
                algorithm,
                data,
                encoding: self.encoding,
                verbose: self.verbose,
            });
        }

        return Err(CommandParseError::NotImplemented);
    }
}

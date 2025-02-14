use std::{
    fs::File,
    io,
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{
    algorithms::Algorithm,
    chunked_stream::ChunkedStream,
    cli::algorithms::{Specification, ALGORITHMS},
};

use super::{encoding::Encoding, opts::Opts, parsers::parse_algorithm};

pub enum Command {
    List,
    Digest {
        algorithm: Box<dyn Algorithm>,
        data: ChunkedStream,
        encoding: Encoding,
        verbose: bool,
    },
}

pub enum CommandParseError {
    FileDoesNotExist,
    PathIsDirectory,
    InvalidPath(io::Error),
    InvalidAlgorithm(String),
    NotImplemented,
}

fn get_data(opts: &Opts, input: String) -> Result<ChunkedStream, CommandParseError> {
    if opts.file {
        let path = PathBuf::from(input);

        let file = File::open(&path).map_err(|err| match err.kind() {
            io::ErrorKind::NotFound => CommandParseError::FileDoesNotExist,
            _ => CommandParseError::InvalidPath(err),
        })?;

        if path.is_dir() {
            return Err(CommandParseError::PathIsDirectory);
        }

        return Ok(ChunkedStream::from(file));
    } else {
        return Ok(ChunkedStream::from(input));
    }
}

fn get_formatted_time_taken(duration: Duration) -> String {
    let s = duration.as_secs();
    let ms = duration.as_millis();
    let us = duration.as_micros();

    if s > 2000 {
        format!("{}.{}s", s, duration.subsec_millis())
    } else if ms > 2 {
        format!("{}.{}ms", ms, duration.subsec_micros())
    } else {
        format!("{}.{}us", us, duration.subsec_nanos())
    }
}

impl TryFrom<Opts> for Command {
    type Error = CommandParseError;

    fn try_from(opts: Opts) -> Result<Self, Self::Error> {
        if opts.list {
            return Ok(Command::List);
        }

        if let (Some(input), Some(algorithm)) = (&opts.input, &opts.algorithm) {
            let data = get_data(&opts, input.to_string())?;
            let (_, algorithm) = parse_algorithm(algorithm)
                .map_err(|err| CommandParseError::InvalidAlgorithm(err.to_string()))?;

            return Ok(Command::Digest {
                algorithm,
                data,
                encoding: opts.encoding,
                verbose: opts.verbose,
            });
        }

        return Err(CommandParseError::NotImplemented);
    }
}

impl Command {
    pub fn execute(self) -> Result<(), anyhow::Error> {
        match self {
            Self::List => {
                println!("{}", list_algorithms());
                Ok(())
            }
            Self::Digest {
                data,
                algorithm,
                encoding,
                verbose,
            } => {
                let start_time = Instant::now();

                let digest_bytes = algorithm.digest(data)?;
                let digest_encoded = encoding.encode(digest_bytes);

                let end_time = Instant::now();
                let time_taken = end_time - start_time;

                println!("{}", digest_encoded);

                if verbose {
                    println!("Time taken: {}", get_formatted_time_taken(time_taken));
                }

                Ok(())
            }
        }
    }
}

fn list_algorithms() -> String {
    let count = ALGORITHMS.iter().fold(0, |current, spec| match spec {
        Specification::Single(_) => current + 1,
        Specification::Family { name: _, members } => current + members.len(),
    });

    let list = ALGORITHMS
        .iter()
        .map(|spec| match spec {
            Specification::Single(name) => format!("- {}", name),
            Specification::Family { name, members } => [
                format!("- {} family", name),
                members
                    .iter()
                    .map(|member| format!("|- {}", member))
                    .collect::<Vec<String>>()
                    .join("\n"),
            ]
            .join("\n"),
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!("Algorithm count: {}\n{}", count, list)
}

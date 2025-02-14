use std::time::{Duration, Instant};

use crate::{
    algorithms::Algorithm,
    chunked_stream::ChunkedStream,
    cli::algorithms::{Specification, ALGORITHMS},
};

use super::encoding::Encoding;

pub enum Command {
    List,
    Digest {
        algorithm: Box<dyn Algorithm>,
        data: ChunkedStream,
        encoding: Encoding,
        verbose: bool,
    },
}

fn get_formatted_time_taken(duration: Duration) -> String {
    let s = duration.as_secs();
    let ms = duration.as_millis();
    let us = duration.as_micros();

    if s > 2 {
        format!("{}.{}s", s, duration.subsec_millis())
    } else if ms > 2 {
        format!("{}.{}ms", ms, duration.subsec_micros())
    } else {
        format!("{}.{}us", us, duration.subsec_nanos())
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

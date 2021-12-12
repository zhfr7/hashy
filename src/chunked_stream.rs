use std::fs::File;
use std::io::{Result, BufReader, Read};

pub enum ChunkedStream {
    Bytes(Vec<u8>),
    File(BufReader<File>)
}

pub struct ChunkedIter {
    data: ChunkedStream,
    chunk_size: usize
}

impl ChunkedStream {
    pub fn from_string(input: &String) -> Self {
        let bytes = input.as_bytes().to_vec();
        ChunkedStream::Bytes(bytes)
    }

    pub fn from_file(filepath: &String) -> Result<Self> {
        let file = File::open(filepath)?;
        Ok(ChunkedStream::File(BufReader::new(file)))
    }

    pub fn into_iter(self, chunk_size: usize) -> ChunkedIter {
        ChunkedIter {
            data: self,
            chunk_size,
        }
    }
}

impl Iterator for ChunkedIter {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.data {
            ChunkedStream::Bytes(bytes) => 
            {
                if bytes.is_empty() {
                    None
                } else if bytes.len() < self.chunk_size {
                    Some(Ok(bytes.drain(..).collect()))
                } else {
                    Some(Ok(bytes.drain(..self.chunk_size).collect()))
                }
            },

            ChunkedStream::File(reader) => 
            {
                let mut chunk = Vec::with_capacity(self.chunk_size);

                match reader.take(self.chunk_size as u64).read_to_end(&mut chunk) {
                    Ok(0) => None,
                    Ok(_) => Some(Ok(chunk)),
                    Err(err) => Some(Err(err))
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::io::BufReader;

    use super::ChunkedStream;

    #[test]
    fn chunk_iterate_bytes() {
        let data = ChunkedStream::Bytes("Example message".as_bytes().into());
        let expected = vec![
            vec![69, 120, 97, 109],
            vec![112, 108, 101, 32],
            vec![109, 101, 115, 115],
            vec![97, 103, 101]
        ];

        let mut i = 0;
        for chunk in data.into_iter(4) {
            assert!(chunk.is_ok());
            assert_eq!(chunk.unwrap(), expected[i]);
            
            i += 1;
        }
    }

    #[test]
    fn chunk_iterate_file() {
        use std::io::{Write, Seek, SeekFrom};

        // Write to temp file and seek to start
        let mut tmpfile = tempfile::tempfile().unwrap();
        write!(tmpfile, "Example message in file").unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        let data = ChunkedStream::File(BufReader::new(tmpfile));
        let expected = vec![
            vec![69, 120, 97, 109, 112],
            vec![108, 101, 32, 109, 101],
            vec![115, 115, 97, 103, 101],
            vec![32, 105, 110, 32, 102],
            vec![105, 108, 101]
        ];

        let mut i = 0;
        for chunk in data.into_iter(5) {
            assert!(chunk.is_ok());
            assert_eq!(chunk.unwrap(), expected[i]);
            
            i += 1;
        }
    }
}
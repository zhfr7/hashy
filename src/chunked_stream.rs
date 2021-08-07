use std::{fs::File, io::{self, Read}};

enum DataType {
    Bytes(Vec<u8>),
    File(File)
}

struct ChunkedIter {
    data: DataType,
    chunk_size: usize
}

impl DataType {
    fn into_iter(self, chunk_size: usize) -> ChunkedIter {
        ChunkedIter {
            data: self,
            chunk_size,
        } 
    }
}

impl Iterator for ChunkedIter {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.data {
            DataType::Bytes(bytes) => 
            {
                if bytes.is_empty() {
                    None
                } else if bytes.len() < self.chunk_size {
                    Some(Ok(bytes.drain(..).collect()))
                } else {
                    Some(Ok(bytes.drain(..self.chunk_size).collect()))
                }
            },

            DataType::File(file) => 
            {
                let mut chunk = Vec::with_capacity(self.chunk_size);

                match file.by_ref().take(self.chunk_size as u64).read_to_end(&mut chunk) {
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
    use super::DataType;

    #[test]
    fn chunk_iterate_bytes() {
        let data = DataType::Bytes("Example message".as_bytes().into());
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

        let data = DataType::File(tmpfile);
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
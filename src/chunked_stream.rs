use std::{fs::File, io::Read};

enum DataType {
    Bytes(Vec<u8>),
    File(File)
}

struct ChunkedIter {
    data: DataType,
    chunk_size: usize,
    bytes_read: usize
}

impl DataType {
    fn into_iter(self, chunk_size: usize) -> ChunkedIter {
        ChunkedIter {
            data: self,
            chunk_size,
            bytes_read: 0
        } 
    }
}

impl Iterator for ChunkedIter {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.data {
            DataType::Bytes(bytes) => 
            {
                if bytes.is_empty() {
                    None
                } else if bytes.len() < self.chunk_size {
                    self.bytes_read += bytes.len();
                    Some(bytes.drain(..).collect())
                } else {
                    self.bytes_read += self.chunk_size;
                    Some(bytes.drain(..self.chunk_size).collect())
                }
            },

            DataType::File(file) => 
            {
                let mut chunk = Vec::with_capacity(self.chunk_size);

                match file.by_ref().take(self.chunk_size as u64).read_to_end(&mut chunk) {
                    Ok(0) => None,
                    Ok(n) => {
                        self.bytes_read += n;
                        Some(chunk)
                    },
                    Err(_) => panic!("File read interrupted!")
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

        let mut data_iter = data.into_iter(4);
        assert_eq!(data_iter.next(), Some(vec![69, 120, 97, 109]));
        assert_eq!(data_iter.bytes_read, 4);

        assert_eq!(data_iter.next(), Some(vec![112, 108, 101, 32]));
        assert_eq!(data_iter.next(), Some(vec![109, 101, 115, 115]));
        assert_eq!(data_iter.next(), Some(vec![97, 103, 101]));
        assert_eq!(data_iter.bytes_read, 15);

        assert_eq!(data_iter.next(), None);
        assert_eq!(data_iter.bytes_read, 15);
    }

    #[test]
    fn chunk_iterate_file() {
        use std::io::{Write, Seek, SeekFrom};

        // Write to temp file and seek to start
        let mut tmpfile = tempfile::tempfile().unwrap();
        write!(tmpfile, "Example message in file").unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        let data = DataType::File(tmpfile);
        let mut data_iter = data.into_iter(5);

        assert_eq!(data_iter.next(), Some(vec![69, 120, 97, 109, 112]));
        assert_eq!(data_iter.next(), Some(vec![108, 101, 32, 109, 101]));
        assert_eq!(data_iter.bytes_read, 10);

        assert_eq!(data_iter.next(), Some(vec![115, 115, 97, 103, 101]));
        assert_eq!(data_iter.next(), Some(vec![32, 105, 110, 32, 102]));
        assert_eq!(data_iter.next(), Some(vec![105, 108, 101]));
        assert_eq!(data_iter.bytes_read, 23);

        assert_eq!(data_iter.next(), None);
        assert_eq!(data_iter.bytes_read, 23);
    }
}
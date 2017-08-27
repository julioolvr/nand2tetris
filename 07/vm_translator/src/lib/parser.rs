use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Lines;

pub struct Parser<'a> {
    file: &'a File,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a File) -> Parser<'a> {
        Parser { file }
    }

    pub fn lines(&mut self) -> Lines<BufReader<&File>> {
        // Go back to the beginning of the file, in case the cursor advanced earlier
        self.file.seek(SeekFrom::Start(0)).unwrap(); // .unwrap() so it fails on Error

        // Create a buffer reader for it
        let buffer = BufReader::new(self.file);

        // And iterate over the lines
        buffer.lines()
    }
}

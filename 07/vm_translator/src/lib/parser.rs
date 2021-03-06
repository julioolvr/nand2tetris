use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;

use lib::command::Command;

pub struct Parser<'a> {
    file: &'a File,
    filename: String,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a File, filename: String) -> Parser<'a> {
        Parser { file, filename }
    }

    pub fn commands(&mut self) -> Vec<Command> {
        // Go back to the beginning of the file, in case the cursor advanced earlier
        self.file.seek(SeekFrom::Start(0)).unwrap(); // .unwrap() so it fails on Error

        // Create a buffer reader for it
        let buffer = BufReader::new(self.file);

        // And iterate over the commands
        // TODO: Figure out if there's a way to avoid collecting and return the iterator
        buffer
            .lines()
            .filter_map(|line_result| line_result.ok())
            .filter(|line| !line.trim().is_empty())
            .filter(|line| !line.starts_with("//"))
            .enumerate()
            .filter_map(|(index, line)| {
                let command = Command::new(line, self.filename.clone(), index);

                match command {
                    Ok(command) => Some(command),
                    Err(err) => {
                        println!("{}", err);
                        None
                    }
                }
            })
            .collect()
    }
}

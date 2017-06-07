use std;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::BufRead;
use std::io::BufReader;

use lib::command::Command;

pub struct Parser<'a> {
  file: &'a File
}

impl<'a> Parser<'a> {
  pub fn new(file: &'a File) -> Parser<'a> {
    Parser {
      file
    }
  }

  pub fn commands(&mut self) -> CommandIterator<FileIterator<'a>> {
    // Go back to the beginning of the file, in case the cursor advanced earlier
    self.file.seek(SeekFrom::Start(0));

    // Create a buffer reader for it
    let buffer = BufReader::new(self.file);

    // And iterate over the commands
    CommandIterator::new(buffer.lines())
  }
}

pub type FileIterator<'a> = std::io::Lines<std::io::BufReader<&'a File>>;

pub struct CommandIterator<I> {
  iter: I
}

impl<I> CommandIterator<I> {
  pub fn new(iter: I) -> CommandIterator<I> {
    CommandIterator { iter }
  }
}

impl<'a> Iterator for CommandIterator<FileIterator<'a>> {
  type Item = Command;

  fn next(&mut self) -> Option<Command> {
    loop {
      match self.iter.next() {
        Some(line_result) => {
          match line_result {
            Ok(line) => {
              if Command::is_command(&line) {
                return Some(Command::new(line))
              } else {
                continue;
              }
            }

            Err(_) => { continue; }
          }
        }

        None => { return None }
      }
    }
  }
}
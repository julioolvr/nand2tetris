use std;
use std::io::BufRead;
use std::io::BufReader;

use lib::command::Command;

pub struct Parser<T> {
  input: BufReader<T>
}

impl<
  T: std::io::Read
> Parser<T> {
  pub fn new(input: BufReader<T>) -> Parser<T> {
    Parser {
      input
    }
  }

  pub fn iter(self) -> CommandIterator<FileIterator<T>> {
    CommandIterator::new(self.input.lines())
  }
}

pub type FileIterator<T> = std::io::Lines<std::io::BufReader<T>>;

pub struct CommandIterator<I> {
  iter: I
}

impl<I> CommandIterator<I> {
  pub fn new(iter: I) -> CommandIterator<I> {
    CommandIterator { iter }
  }
}

impl<T: std::io::Read> Iterator for CommandIterator<FileIterator<T>> {
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
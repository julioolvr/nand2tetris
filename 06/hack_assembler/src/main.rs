#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::env;
use std::io::Write;
use std::io::BufReader;

mod lib;
use lib::parser::Parser;
use lib::code::command_to_binary;

fn main() {
    let mut args = env::args();
    args.next(); // Program name

    let input_filename = args.next().unwrap();
    let output_filename = args.next().unwrap();

    let input_file = File::open(input_filename).unwrap();

    let parser = Parser::new(BufReader::new(&input_file));

    let hack_file = parser.iter().fold(String::new(), |acc, command| {
        if acc == "" {
            format!("{:016b}", command_to_binary(&command))
        } else {
            format!("{}\n{:016b}", acc, command_to_binary(&command))
        }
    });

    println!("Done parsing, writing to {}", output_filename);
    let mut output_file = File::create(output_filename).unwrap();

    match output_file.write_all(hack_file.as_bytes()) {
        Ok(_) => println!("Wrote file successfully"),
        Err(err) => println!("Error writing file, {}", err)
    }
}

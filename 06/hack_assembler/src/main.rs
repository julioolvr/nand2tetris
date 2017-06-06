#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::env;
use std::io::BufReader;

mod lib;
use lib::parser::Parser;
use lib::code::command_to_binary;

fn main() {
    let mut args = env::args();
    args.next(); // Program name

    let filename = args.next().unwrap();
    let file = File::open(filename).unwrap();

    let parser = Parser::new(BufReader::new(&file));

    for command in parser.iter() {
        println!("Parsed command");

        if let Some(ref symbol) = command.symbol() {
            println!("Had symbol {}", symbol);
        }

        if let Some(ref dest) = command.dest() {
            println!("Had dest {}", dest);
        }

        if let Some(ref comp) = command.comp() {
            println!("Had comp {}", comp);
        }

        println!("Binary representation is {:016b}", command_to_binary(&command));
    }

    println!("Done parsing");
}

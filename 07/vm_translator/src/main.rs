use std::env;

use std::fs::File;

mod lib;
use lib::parser::Parser;

fn main() {
    let mut args = env::args();
    args.next(); // Program name

    let input_name = args.next().unwrap(); // File or directory name
    let input_file = File::open(&input_name).unwrap();

    println!("🛠  Compiling {}", input_name);

    let mut parser = Parser::new(&input_file);

    for line in parser.lines() {
        println!("Read line {}", line);
    }

    println!("✅  Done! (WIP)");
}

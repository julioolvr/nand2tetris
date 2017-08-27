use std::env;

use std::fs::File;

mod lib;
use lib::parser::Parser;
use lib::code::command_to_asm;

fn main() {
    let mut args = env::args();
    args.next(); // Program name

    let input_name = args.next().unwrap(); // File or directory name
    let input_file = File::open(&input_name).unwrap();

    println!("🛠  Compiling {}", input_name);

    let mut parser = Parser::new(&input_file);

    for command in parser.commands() {
        println!("Command {}", command_to_asm(&command));
    }

    println!("✅  Done! (WIP)");
}

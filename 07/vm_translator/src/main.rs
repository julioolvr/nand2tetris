extern crate regex;
use std::env;

use std::fs::File;
use std::io::Write;
use std::path::Path;

mod lib;
use lib::parser::Parser;

fn main() {
    let mut args = env::args();
    args.next(); // Program name

    let input_name = args.next().unwrap(); // File or directory name
    let input_file = File::open(&input_name).unwrap();

    let output_name = if input_name.ends_with(".vm") {
        input_name.replace(".vm", ".asm")
    } else {
        format!("{}.asm", input_name)
    };

    println!("🛠  Compiling {}", input_name);

    let mut parser = Parser::new(&input_file,
                                 String::from(Path::new(&input_name)
                                                  .file_name()
                                                  .expect("Only single files allowed for now")
                                                  .to_str()
                                                  .expect("Filename must be valid UTF-8")));

    let asm_file_contents = parser
        .commands()
        .iter()
        .fold(String::new(), |acc, command| {
            println!("{}", command.to_asm());
            if acc.is_empty() {
                command.to_asm()
            } else {
                format!("{}\n\n{}", acc, command.to_asm())
            }
        });

    let mut output_file = File::create(output_name).unwrap();

    match output_file.write_all(asm_file_contents.as_bytes()) {
        Ok(_) => println!("✅  Done! (WIP)"),
        Err(err) => println!("Error writing file, {}", err),
    };
}

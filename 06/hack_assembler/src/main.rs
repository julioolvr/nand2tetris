#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::env;
use std::io::Write;

mod lib;
use lib::parser::Parser;
use lib::command::CommandType;
use lib::code::command_to_binary;
use lib::symbol_table::SymbolTable;

fn main() {
    let mut args = env::args();
    args.next(); // Program name

    let input_filename = args.next().unwrap();
    let output_filename = args.next().unwrap();

    let input_file = File::open(input_filename).unwrap();

    let mut parser = Parser::new(&input_file);

    // First pass - build the symbols table
    let mut symbol_table = SymbolTable::new();
    let mut instruction_index = 0;

    for command in parser.commands() {
        if command.command_type() == CommandType::LCommand {
            if let Some(symbol) = command.symbol() {
                symbol_table.insert(symbol, instruction_index);
            }
        } else {
            instruction_index += 1;
        }
    }

    // Second pass - generate the binary instructions using the symbols table
    let hack_file = parser.commands().fold(String::new(), |acc, command| {
        let binary_command = command_to_binary(&command, &mut symbol_table);

        match binary_command {
            Some(found_command) => {
                if acc == "" {
                    format!("{:016b}", found_command)
                } else {
                    format!("{}\n{:016b}", acc, found_command)
                }
            }

            None => acc
        }
    });

    println!("Done parsing, writing to {}", output_filename);
    let mut output_file = File::create(output_filename).unwrap();

    match output_file.write_all(hack_file.as_bytes()) {
        Ok(_) => println!("Wrote file successfully"),
        Err(err) => println!("Error writing file, {}", err)
    }
}

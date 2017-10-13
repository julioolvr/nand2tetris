use lib::command_types::{ArithmeticCommand, PushCommand, PopCommand};

fn build_command(line: String) -> Option<Box<CommandType>> {
    if ArithmeticCommand::is_a(&line) {
        Some(Box::new(ArithmeticCommand::new(line)))
    } else if PushCommand::is_a(&line) {
        Some(Box::new(PushCommand::new(line)))
    } else if PopCommand::is_a(&line) {
        Some(Box::new(PopCommand::new(line)))
    } else {
        None
    }
}

pub struct Command {
    command_implementation: Box<CommandType>,
}

impl Command {
    pub fn new(line: String) -> Result<Command, String> {
        if let Some(implementation) = build_command(line.clone()) {
            Ok(Command { command_implementation: implementation })
        } else {
            Err(format!("Found invalid line when parsing command:\n`{}`", line))
        }
    }

    pub fn to_asm(&self) -> String {
        self.command_implementation.to_asm()
    }
}

pub trait CommandType {
    fn to_asm(&self) -> String;
}

pub trait CommandTypeBuilder {
    fn new(line: String) -> Self;
    fn is_a(line: &String) -> bool;
}

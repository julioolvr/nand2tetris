use lib::command::{CommandType, CommandTypeBuilder};

pub struct PopCommand {
    line: String,
}

impl CommandTypeBuilder for PopCommand {
    fn new(line: String) -> Self {
        Self { line }
    }

    fn is_a(line: &String) -> bool {
        line.starts_with("pop")
    }
}

impl CommandType for PopCommand {
    fn to_asm(&self, index: usize) -> String {
        String::from("Pop command")
    }
}
use lib::command::{CommandType, CommandTypeBuilder};

pub struct PushCommand {
    line: String,
}

impl CommandTypeBuilder for PushCommand {
    fn new(line: String) -> Self {
        Self { line }
    }

    fn is_a(line: &String) -> bool {
        line.starts_with("push")
    }
}

impl CommandType for PushCommand {
    fn to_asm(&self) -> String {
        String::from("Push command")
    }
}
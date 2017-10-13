use lib::command::{CommandType, CommandTypeBuilder};

pub struct ArithmeticCommand {
    line: String,
}

impl CommandTypeBuilder for ArithmeticCommand {
    fn new(line: String) -> ArithmeticCommand {
        ArithmeticCommand { line }
    }

    fn is_a(line: &String) -> bool {
        line.starts_with("add") || line.starts_with("sub") || line.starts_with("neg")
    }
}

impl CommandType for ArithmeticCommand {
    fn to_asm(&self) -> String {
        String::from("Arithmetic command")
    }
}

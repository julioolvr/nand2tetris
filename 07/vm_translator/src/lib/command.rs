#[derive(PartialEq)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP, // LABEL,
         // GOTO,
         // IF,
         // FUNCTION,
         // RETURN,
         // CALL
}

pub struct Command {
    line: String,
}

impl Command {
    pub fn new(line: String) -> Command {
        Command { line }
    }

    pub fn get_line(&self) -> &String {
        &self.line
    }

    pub fn command_type(&self) -> CommandType {
        // TODO: Support other command types
        if self.line.starts_with("add") || self.line.starts_with("sub") ||
           self.line.starts_with("neg") {
            CommandType::ARITHMETIC
        } else if self.line.starts_with("push") {
            CommandType::PUSH
        } else {
            CommandType::POP
        }
    }
}

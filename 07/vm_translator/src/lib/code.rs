use lib::command::Command;
use lib::command::CommandType;

pub fn command_to_asm(command: &Command) -> &str {
  match command.command_type() {
    CommandType::ARITHMETIC => arithmetic_to_asm(command),
    CommandType::PUSH => push_to_asm(command),
    CommandType::POP => pop_to_asm(command)
  }
}

fn arithmetic_to_asm(command: &Command) -> &str {
  "ARITHMETIC cmd"
}

fn push_to_asm(command: &Command) -> &str {
  "PUSH cmd"
}

fn pop_to_asm(command: &Command) -> &str {
  "POP cmd"
}
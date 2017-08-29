use regex::Regex;

use lib::command::Command;
use lib::command::CommandType;

pub fn command_to_asm(command: &Command) -> String {
    match command.command_type() {
        CommandType::ARITHMETIC => arithmetic_to_asm(command),
        CommandType::PUSH => push_to_asm(command),
        CommandType::POP => pop_to_asm(command),
    }
}

fn arithmetic_to_asm(command: &Command) -> String {
    "ARITHMETIC cmd".to_string()
}

/// Translate a push command to the corresponding ASM instructions
/// TODO: Support non-constant pushes
///
/// # Syntax
/// `push constant 42`
///
/// # Translation
/// ```
/// @42
/// D=A
/// @SP
/// M=D
/// A=A+1
/// D=A
/// @0
/// M=D
/// ```
///
/// # Steps
/// 1. Load the constant's value in the D register
/// 2. Load that value in the memory address pointed by SP
/// 3. Store in SP the value of SP + 1
fn push_to_asm(command: &Command) -> String {
    // Match with regex to extract constant
    let push_constant_regex = Regex::new(r"^push constant (\d+)$").unwrap();
    let constant = push_constant_regex
        .captures(command.get_line())
        .unwrap()
        .get(1)
        .expect("Constant value for push command not found")
        .as_str();

    // Build vec of lines
    let lines = vec![format!("@{}", constant),
                     "D=A".to_string(),
                     "@SP".to_string(),
                     "M=D".to_string(),
                     "A=A+1".to_string(),
                     "D=A".to_string(),
                     "@0".to_string(),
                     "M=D".to_string()];

    // Join? And return them
    lines.join("\n")
}

fn pop_to_asm(command: &Command) -> String {
    "POP cmd".to_string()
}

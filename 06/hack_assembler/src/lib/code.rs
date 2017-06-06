use lib::command::Command;
use lib::command::CommandType;
use lib::command::JumpMnemonic;

pub fn command_to_binary(command: &Command) -> String {
  match command.command_type() {
    CommandType::ACommand => { a_command_to_binary(command) }
    CommandType::CCommand => { c_command_to_binary(command) }
    CommandType::LCommand => { String::from("LCommand") }
  }
}

fn a_command_to_binary(command: &Command) -> String {
  match command.symbol() {
    // TODO: Support non-literal symbol
    Some(symbol) => { format!("0{:015b}", symbol.parse::<i16>().expect("Assuming valid i16 for A command")) }
    // TODO: This shouldn't be possible, so consider returning Result<String> instead to check for errors
    None => { String::from("0000000000000000") }
  }
}

fn c_command_to_binary(command: &Command) -> String {
  let dest = dest_to_binary(command.dest());
  let comp = comp_to_binary(command.comp());
  let jump = jump_to_binary(command.jump());

  format!("111{}{}{}", comp, dest, jump)
}

fn dest_to_binary(dest: Option<&str>) -> String {
  let mut dest_number = 0;

  if let Some(dest_string) = dest {
    if dest_string.contains("M") { dest_number = dest_number | 0b001 }
    if dest_string.contains("D") { dest_number = dest_number | 0b010 }
    if dest_string.contains("A") { dest_number = dest_number | 0b100 }
  }

  format!("{:03b}", dest_number)
}

fn comp_to_binary(comp: Option<&str>) -> &str {
  "1111111"
}

fn jump_to_binary(jump: Option<JumpMnemonic>) -> &'static str {
  match jump {
    Some(mnemonic) => {
      match mnemonic {
        JumpMnemonic::JGT => "001",
        JumpMnemonic::JEQ => "010",
        JumpMnemonic::JGE => "011",
        JumpMnemonic::JLT => "100",
        JumpMnemonic::JNE => "101",
        JumpMnemonic::JLE => "110",
        JumpMnemonic::JMP => "111"
      }
    }

    None => "000"
  }
}
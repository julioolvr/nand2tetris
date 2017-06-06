use lib::command::Command;
use lib::command::CommandType;
use lib::command::JumpMnemonic;

pub fn command_to_binary(command: &Command) -> u16 {
  match command.command_type() {
    CommandType::ACommand => { a_command_to_binary(command) }
    CommandType::CCommand => { c_command_to_binary(command) }
    CommandType::LCommand => { 0b0000000000000000 } // TODO: Proper handling of the L command
  }
}

fn a_command_to_binary(command: &Command) -> u16 {
  match command.symbol() {
    // TODO: Support non-literal symbol
    Some(symbol) => { symbol.parse::<u16>().expect("Assuming valid u16 for A command") }
    // TODO: This shouldn't be possible, so consider returning Result<String> instead to check for errors
    None => { 0b0000000000000000 }
  }
}

fn c_command_to_binary(command: &Command) -> u16 {
  let dest = dest_to_binary(command.dest());
  let comp = comp_to_binary(command.comp());
  let jump = jump_to_binary(command.jump());

  0b1110000000000000 | (comp as u16) << 6 | (dest as u16) << 3 | jump as u16
}

fn dest_to_binary(dest: Option<&str>) -> u8 {
  let mut dest_number = 0;

  if let Some(dest_string) = dest {
    if dest_string.contains("M") { dest_number = dest_number | 0b001 }
    if dest_string.contains("D") { dest_number = dest_number | 0b010 }
    if dest_string.contains("A") { dest_number = dest_number | 0b100 }
  }

  dest_number
}

fn comp_to_binary(comp: Option<&str>) -> u8 {
  0b1111111
}

fn jump_to_binary(jump: Option<JumpMnemonic>) -> u8 {
  match jump {
    Some(mnemonic) => {
      match mnemonic {
        JumpMnemonic::JGT => 0b001,
        JumpMnemonic::JEQ => 0b010,
        JumpMnemonic::JGE => 0b011,
        JumpMnemonic::JLT => 0b100,
        JumpMnemonic::JNE => 0b101,
        JumpMnemonic::JLE => 0b110,
        JumpMnemonic::JMP => 0b111
      }
    }

    None => 0b000
  }
}
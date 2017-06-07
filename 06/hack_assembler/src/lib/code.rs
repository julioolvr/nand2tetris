use lib::command::Command;
use lib::command::CommandType;
use lib::command::JumpMnemonic;
use lib::symbol_table::SymbolTable;

pub fn command_to_binary(command: &Command, symbol_table: &mut SymbolTable) -> Option<u16> {
  match command.command_type() {
    CommandType::ACommand => Some(a_command_to_binary(command, symbol_table)),
    CommandType::CCommand => Some(c_command_to_binary(command)),
    CommandType::LCommand => None
  }
}

fn a_command_to_binary(command: &Command, symbol_table: &mut SymbolTable) -> u16 {
  match command.symbol() {
    Some(symbol) => {
      match symbol.parse::<u16>() {
        Ok(number) => number,
        Err(_) => {
          // This means that the symbol was a variable.
          // Either it's already in the symbol table, and I have to use that value
          // or it isn't, and I have to add it to the table with a new value and use that
          if !symbol_table.contains_key(&symbol) {
            symbol_table.append(symbol.clone());
          }

          *symbol_table.get(&symbol).unwrap() as u16
        }
      }
    }
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
  if let Some(comp_string) = comp {
    match comp_string {
      "0"   => 0b00101010,
      "1"   => 0b00111111,
      "-1"  => 0b00111010,
      "D"   => 0b00001100,
      "A"   => 0b00110000,
      "!D"  => 0b00001101,
      "!A"  => 0b00110001,
      "-D"  => 0b00001111,
      "-A"  => 0b00110011,
      "D+1" => 0b00011111,
      "A+1" => 0b00110111,
      "D-1" => 0b00001110,
      "A-1" => 0b00110010,
      "D+A" => 0b00000010,
      "D-A" => 0b00010011,
      "A-D" => 0b00000111,
      "D&A" => 0b00000000,
      "D|A" => 0b00010101,
      "M"   => 0b01110000,
      "!M"  => 0b01110001,
      "-M"  => 0b01110011,
      "M+1" => 0b01110111,
      "M-1" => 0b01110010,
      "D+M" => 0b01000010,
      "D-M" => 0b01010011,
      "M-D" => 0b01000111,
      "D&M" => 0b01000000,
      "D|M" => 0b01010101,
      // TODO: Shouldn't happen, handle as an error
      _     => 0b00000000
    }
  } else {
    // TODO: Shouldn't happen, handle as an error
    0b00000000
  }
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
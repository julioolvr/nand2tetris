use lib::command::Command;
use lib::command::CommandType;

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

fn dest_to_binary(dest: Option<&str>) -> &str {
  "000"
}

fn comp_to_binary(comp: Option<&str>) -> &str {
  "1111111"
}

fn jump_to_binary(jump: Option<&str>) -> &str {
  "000"
}
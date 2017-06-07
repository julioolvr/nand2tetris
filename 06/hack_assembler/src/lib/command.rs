use regex;
use regex::Regex;

pub struct Command {
  line: String
}

#[derive(PartialEq)]
pub enum CommandType {
  ACommand,
  CCommand,
  LCommand
}

pub enum JumpMnemonic {
  JGT,
  JEQ,
  JGE,
  JLT,
  JNE,
  JLE,
  JMP
}

lazy_static! {
  // A-instruction regex
  // Structure:
  // @symbol
  // Where `symbol` is either a name or a number in base 10
  static ref A_COMMAND_REGEX: Regex = Regex::new(r"^@(.+)$").unwrap();

  // C-instruction regex
  // Structure:
  // dest=comp;jump
  // if `dest` is empty, `=` is ommited
  // if `jump` is empty, `;` is ommited
  static ref C_COMMAND_REGEX: Regex = Regex::new(r"^(?:(\w+)=)?([\w|\||&|+|\-|!]+)(?:;(\w+))?(?:\s*//.*)?$").unwrap();

  // Fake "L-instruction" regex
  // Structure:
  // (LabelName)
  // Where `LabelName` is a string
  static ref L_COMMAND_REGEX: Regex = Regex::new(r"^\(([^)]+)\)$").unwrap();
}


impl Command {
  pub fn new(line: String) -> Command {
    Command { line }
  }

  pub fn command_type(&self) -> CommandType {
    if A_COMMAND_REGEX.is_match(self.line.trim()) { return CommandType::ACommand }
    if C_COMMAND_REGEX.is_match(self.line.trim()) { return CommandType::CCommand }

    // TODO: Support LCommand properly and don't just assume everything that's not A or C
    // is L
    CommandType::LCommand
  }

  // TODO: Maybe be able to return either a string or a number somehow?
  pub fn symbol(&self) -> Option<String> {
    match self.command_type() {
      CommandType::ACommand => Command::extract_capture(self.a_command_captures(), 1),
      CommandType::LCommand => Command::extract_capture(self.l_command_captures(), 1),
      _ => None
    }.map(|s| String::from(s))
  }

  pub fn dest(&self) -> Option<&str> {
    Command::extract_capture(self.c_command_captures(), 1)
  }

  pub fn comp(&self) -> Option<&str> {
    Command::extract_capture(self.c_command_captures(), 2)
  }

  pub fn jump(&self) -> Option<JumpMnemonic> {
    if let Some(mnemonic) = Command::extract_capture(self.c_command_captures(), 3) {
      match mnemonic {
        "JGT" => Some(JumpMnemonic::JGT),
        "JEQ" => Some(JumpMnemonic::JEQ),
        "JGE" => Some(JumpMnemonic::JGE),
        "JLT" => Some(JumpMnemonic::JLT),
        "JNE" => Some(JumpMnemonic::JNE),
        "JLE" => Some(JumpMnemonic::JLE),
        "JMP" => Some(JumpMnemonic::JMP),
        _ => None
      }
    } else {
      None
    }
  }

  pub fn is_command(line: &String) -> bool {
    let trimmed_line = line.trim();

    // If it's only whitespace, then it's not a command
    if trimmed_line.is_empty() { return false }

    // If it's a comment, then it's not a command
    if trimmed_line.starts_with("//") { return false }

    // For now, everything else IS a command
    return true
  }

  fn a_command_captures(&self) -> Option<regex::Captures> {
    A_COMMAND_REGEX
      .captures(self.line.trim())
  }

  fn c_command_captures(&self) -> Option<regex::Captures> {
    C_COMMAND_REGEX
      .captures(self.line.trim())
  }

  fn l_command_captures(&self) -> Option<regex::Captures> {
    L_COMMAND_REGEX
      .captures(self.line.trim())
  }

  fn extract_capture(captures: Option<regex::Captures>, index: usize) -> Option<&str> {
    captures
      .map(|capture| capture.get(index).map(|m| m.as_str()))
      .unwrap_or(None)
  }
}

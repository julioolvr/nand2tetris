pub struct Command {
  line: String
}

impl Command {
  pub fn new(line: String) -> Command {
    Command { line }
  }

  pub fn get_line(&self) -> &String {
    &self.line
  }
}
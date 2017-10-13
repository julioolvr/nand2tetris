use lib::command::{CommandType, CommandTypeBuilder};

pub struct PushCommand {
    line: String,
}

impl CommandTypeBuilder for PushCommand {
    fn new(line: String) -> Self {
        Self { line }
    }

    fn is_a(line: &String) -> bool {
        line.starts_with("push")
    }
}

impl CommandType for PushCommand {
    fn to_asm(&self) -> String {
        let split_line = self.line.split_whitespace().collect::<Vec<&str>>();
        let segment = split_line[1];

        if segment != "constant" {
            // TODO: Non-constant push support
            unimplemented!();
        }

        let value = split_line[2];

        vec![
          format!("@{}", value).as_str(), // Set the constant value on the A register
          "D=A", // Store that value in the D register
          "@SP", // Load the address of @SP in A
          "A=M", // Load the value stored in @SP in A
          "M=D", // Store the constant value from D where @SP points to
          "A=A+1", // Increase the value stored in A (the memory address stored in @SP)
          "D=A", // Save the new memory address in D
          "@SP", // Load the address of @SP in A again and...
          "M=D" // Update its value to contain the next memory address
        ]
                .join("\n")
    }
}
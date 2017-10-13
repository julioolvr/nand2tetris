use lib::command::{CommandType, CommandTypeBuilder};

pub struct ArithmeticCommand {
    line: String,
}

impl CommandTypeBuilder for ArithmeticCommand {
    fn new(line: String) -> ArithmeticCommand {
        ArithmeticCommand { line }
    }

    fn is_a(line: &String) -> bool {
        line.starts_with("add") || line.starts_with("sub") || line.starts_with("neg")
    }
}

impl CommandType for ArithmeticCommand {
    fn to_asm(&self) -> String {
        if self.line == "add" {
            vec![
              "@SP", // Load the address of @SP in A
              "A=M", // Load the value stored in @SP in A
              // Decrease the value in A by 1 (point to the last element
              // in the stack instead of the next empty space)
              "A=A-1",
              "D=M", // Store the value in Memory[A/@SP] in D
              "A=A-1", // Pop another element
              "M=D+M", // Add the already popped element with the new one
              "D=A+1", // Store the new SP memory address in D
              "@SP", // Point to the SP again and
              "M=D" // Store the new SP memory address at @SP
            ]
                    .join("\n")
        } else {
            unimplemented!();
        }
    }
}

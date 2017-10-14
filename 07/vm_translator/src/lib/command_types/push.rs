use lib::command::{CommandType, CommandTypeBuilder, Context};

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
    fn to_asm(&self, context: &Context) -> String {
        let split_line = self.line.split_whitespace().collect::<Vec<&str>>();
        let segment = split_line[1];
        let value = split_line[2];

        let data_load = match segment {
            "constant" => {
                vec![
                    format!("@{}", value).as_str(), // Set the constant value on the A register
                    "D=A", // Store that value in the D register
                ]
                        .join("\n")
            }
            "local" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@LCL", // Put the memory address of LCL in A
                    "A=M+D", // Put the memory address stored in LCL + the index in A
                    "D=M" // Put the value from the new memory address in the D register
                ]
                        .join("\n")
            }
            "argument" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@ARG", // Put the memory address of ARG in A
                    "A=M+D", // Put the memory address stored in ARG + the index in A
                    "D=M" // Put the value from the new memory address in the D register
                ]
                        .join("\n")
            }
            "this" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@THIS", // Put the memory address of THIS in A
                    "A=M+D", // Put the memory address stored in THIS + the index in A
                    "D=M" // Put the value from the new memory address in the D register
                ]
                        .join("\n")
            }
            "that" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@THAT", // Put the memory address of THAT in A
                    "A=M+D", // Put the memory address stored in THAT + the index in A
                    "D=M" // Put the value from the new memory address in the D register
                ]
                        .join("\n")
            }
            "temp" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@5", // temp values are stored starting at address 5
                    "A=M+D", // Put the memory address stored in 5 + the index in A
                    "D=M" // Put the value from the new memory address in the D register
                ]
                        .join("\n")
            }
            "pointer" => {
                vec![
                    format!("@{}", value).as_str(), // Set the value on the A register
                    "D=A", // Move it to the D register
                    "@3", // (3+0) stores the pointer to THIS, (3+1) the pointer to THAT
                    "A=A+D", // Put the memory address stored in 3 + the index in A
                    "D=M" // Put the value from the new memory address in the D register
                ]
                        .join("\n")
            }
            "static" => {
                vec![
                    // Lookup a variable for this static member
                    format!("@{}.{}", context.filename, value).as_str(),
                    "D=M", // Store its value in the D register
                ]
                        .join("\n")
            }
            _ => unimplemented!(),
        };

        vec![
          data_load.as_str(),
          "@SP", // Load the address of @SP in A
          "A=M", // Load the value stored in @SP in A
          "M=D", // Store the value from D where @SP points to
          "A=A+1", // Increase the value stored in A (the memory address stored in @SP)
          "D=A", // Save the new memory address in D
          "@SP", // Load the address of @SP in A again and...
          "M=D" // Update its value to contain the next memory address
        ]
                .join("\n")
    }
}
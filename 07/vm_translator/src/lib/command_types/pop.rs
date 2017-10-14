use lib::command::{CommandType, CommandTypeBuilder, Context};

pub struct PopCommand {
    line: String,
}

impl CommandTypeBuilder for PopCommand {
    fn new(line: String) -> Self {
        Self { line }
    }

    fn is_a(line: &String) -> bool {
        line.starts_with("pop")
    }
}

impl CommandType for PopCommand {
    fn to_asm(&self, context: &Context) -> String {
        let split_line = self.line.split_whitespace().collect::<Vec<&str>>();
        let segment = split_line[1];
        let value = split_line[2];

        let data_pop = vec![
            "@SP", // Load the address of @SP in A
            "D=M-1", // Decrease whatever is stored in it by one
            "AM=D" // Save the new pointer
        ]
                .join("\n");

        let calculate_mem_address = match segment {
            "local" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@LCL", // Put the memory address of LCL in A
                    "D=M+D" // Put the memory address stored in LCL + the index in D
                ]
                        .join("\n")
            }
            "argument" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@ARG", // Put the memory address of ARG in A
                    "D=M+D" // Put the memory address stored in ARG + the index in D
                ]
                        .join("\n")
            }
            "this" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@THIS", // Put the memory address of THIS in A
                    "D=M+D" // Put the memory address stored in THIS + the index in D
                ]
                        .join("\n")
            }
            "that" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@THAT", // Put the memory address of THAT in A
                    "D=M+D" // Put the memory address stored in THAT + the index in D
                ]
                        .join("\n")
            }
            "temp" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@5", // temp values get stored starting at address 5
                    "D=A+D" // Put the memory address index + 5 in D
                ]
                        .join("\n")
            }
            "pointer" => {
                vec![
                    format!("@{}", value).as_str(), // Set the index on the A register
                    "D=A", // Move it to the D register
                    "@3", // (3+0) stores the pointer to THIS, (3+1) the pointer to THAT
                    "D=A+D" // Put the memory address index + 3 in D
                ]
                        .join("\n")
            }
            "static" => {
                vec![
                    // Create a variable for this static segment
                    format!("@{}.{}", context.filename, value).as_str(),
                    "D=A" // Put the memory address in D
                ]
                        .join("\n")
            }
            _ => unimplemented!(),
        };

        vec![data_pop.as_str(), calculate_mem_address.as_str(), "@pop.address", // Store the final address somewhere
                    "M=D",
                    "@SP", // Read the value pointed by @SP
                    "A=M", "D=M",
                    "@pop.address", // Read the address we stored before
                    "A=M",
                    "M=D" // And save the value from @SP there
                    ].join("\n")
    }
}
use lib::command::{CommandType, CommandTypeBuilder, Context};

pub struct ArithmeticCommand {
    line: String,
}

impl CommandTypeBuilder for ArithmeticCommand {
    fn new(line: String) -> ArithmeticCommand {
        ArithmeticCommand { line }
    }

    fn is_a(line: &String) -> bool {
        line.starts_with("add") || line.starts_with("sub") || line.starts_with("neg") ||
        line.starts_with("eq") ||
        line.starts_with("gt") || line.starts_with("lt") || line.starts_with("not") ||
        line.starts_with("or") || line.starts_with("and")
    }
}

impl CommandType for ArithmeticCommand {
    fn to_asm(&self, context: &Context) -> String {
        match self.line.as_str() {
            "add" => ArithmeticCommand::add(),
            "sub" => ArithmeticCommand::sub(),
            "neg" => ArithmeticCommand::neg(),
            "eq" => ArithmeticCommand::eq(context.index),
            "gt" => ArithmeticCommand::gt(context.index),
            "lt" => ArithmeticCommand::lt(context.index),
            "not" => ArithmeticCommand::not(),
            "or" => ArithmeticCommand::or(),
            "and" => ArithmeticCommand::and(),
            _ => panic!("Invalid arithmetic command {}", self.line),
        }
    }
}

impl ArithmeticCommand {
    fn add() -> String {
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
    }

    fn sub() -> String {
        vec![
              "@SP", // Load the address of @SP in A
              "A=M", // Load the value stored in @SP in A
              // Decrease the value in A by 1 (point to the last element
              // in the stack instead of the next empty space)
              "A=A-1",
              "D=M", // Store the value in Memory[A/@SP] in D
              "A=A-1", // Pop another element
              "M=M-D", // Subtract the new element from the already popped one
              "D=A+1", // Store the new SP memory address in D
              "@SP", // Point to the SP again and
              "M=D" // Store the new SP memory address at @SP
            ]
                .join("\n")
    }

    fn neg() -> String {
        vec![
              "@SP", // Load the address of @SP in A
              "A=M", // Load the value stored in @SP in A
              // Decrease the value in A by 1 (point to the last element
              // in the stack instead of the next empty space)
              "A=A-1",
              "D=M", // Store the value in Memory[A/@SP] in D
              "M=-D", // Store the negated value in memory
              "D=A+1", // Store the new SP memory address in D
              "@SP", // Point to the SP again and
              "M=D" // Store the new SP memory address at @SP
            ]
                .join("\n")
    }

    fn eq(index: usize) -> String {
        vec!["@SP", // Load the address of @SP in A
             "A=M", // Load the value stored in @SP in A
             // Decrease the value in A by 1 (point to the last element
             // in the stack instead of the next empty space)
             "A=A-1",
             "D=M", // Store the value in Memory[A/@SP] in D
             "A=A-1", // Pop another element
             "D=D-M", // Subtract the two elements
             format!("@eq.equal.{}", index).as_str(), // Place to jump to if the values are equal
             "D;JEQ", // Jump if they were equal
             "@SP", // Not equal -- Load the address of @SP in A
             "A=M",
             "A=A-1", // Go back 2 spots
             "D=A-1",
             "@SP", // Store the new SP address
             "M=D",
             "@0", // Prepare to store a 0 in the stack
             "D=A",
             "@SP",
             "A=M",
             "M=D", // Store the 0 (false) value in the stack
             format!("@eq.end.{}", index).as_str(), // Jump to the end
             "0;JMP", // Unconditionally
             format!("(eq.equal.{})", index).as_str(), // If they WERE equal
             "@SP", // Load the address of @SP in A
             "A=M",
             "A=A-1", // Go back 2 spots
             "D=A-1",
             "@SP", // Store the new SP address
             "M=D",
             "@0", // Prepare to store a -1 in the stack
             "D=A-1", // Make that 0 a -1
             "@SP",
             "A=M",
             "M=D", // Store the -1 (true) value in the stack
             format!("(eq.end.{})", index).as_str(),
             "@SP", // Increase the SP again
             "D=M+1",
             "M=D"]
                .join("\n")
    }

    fn gt(index: usize) -> String {
        vec!["@SP", // Load the address of @SP in A
             "A=M", // Load the value stored in @SP in A
             // Decrease the value in A by 1 (point to the last element
             // in the stack instead of the next empty space)
             "A=A-1",
             "D=M", // Store the value in Memory[A/@SP] in D
             "A=A-1", // Pop another element
             "D=M-D", // Subtract the two elements
             format!("@gt.greater.{}", index).as_str(), // Place to jump to if M > D
             "D;JGT", // Jump if they first value (in memory) was greater than the second one (in D)
             "@SP", // Not gt -- Load the address of @SP in A
             "A=M",
             "A=A-1", // Go back 2 spots
             "D=A-1",
             "@SP", // Store the new SP address
             "M=D",
             "@0", // Prepare to store a 0 in the stack
             "D=A",
             "@SP",
             "A=M",
             "M=D", // Store the 0 (false) value in the stack
             format!("@gt.end.{}", index).as_str(), // Jump to the end
             "0;JMP", // Unconditionally
             format!("(gt.greater.{})", index).as_str(), // If gt
             "@SP", // Load the address of @SP in A
             "A=M",
             "A=A-1", // Go back 2 spots
             "D=A-1",
             "@SP", // Store the new SP address
             "M=D",
             "@0", // Prepare to store a -1 in the stack
             "D=A-1", // Make that 0 a -1
             "@SP",
             "A=M",
             "M=D", // Store the -1 (true) value in the stack
             format!("(gt.end.{})", index).as_str(),
             "@SP", // Increase the SP again
             "D=M+1",
             "M=D"]
                .join("\n")
    }

    fn lt(index: usize) -> String {
        vec!["@SP", // Load the address of @SP in A
             "A=M", // Load the value stored in @SP in A
             // Decrease the value in A by 1 (point to the last element
             // in the stack instead of the next empty space)
             "A=A-1",
             "D=M", // Store the value in Memory[A/@SP] in D
             "A=A-1", // Pop another element
             "D=M-D", // Subtract the two elements
             format!("@lt.less.{}", index).as_str(), // Place to jump to if M > D
             "D;JLT", // Jump if they first value (in memory) was less than the second one (in D)
             "@SP", // Not gt -- Load the address of @SP in A
             "A=M",
             "A=A-1", // Go back 2 spots
             "D=A-1",
             "@SP", // Store the new SP address
             "M=D",
             "@0", // Prepare to store a 0 in the stack
             "D=A",
             "@SP",
             "A=M",
             "M=D", // Store the 0 (false) value in the stack
             format!("@lt.end.{}", index).as_str(), // Jump to the end
             "0;JMP", // Unconditionally
             format!("(lt.less.{})", index).as_str(), // If gt
             "@SP", // Load the address of @SP in A
             "A=M",
             "A=A-1", // Go back 2 spots
             "D=A-1",
             "@SP", // Store the new SP address
             "M=D",
             "@0", // Prepare to store a -1 in the stack
             "D=A-1", // Make that 0 a -1
             "@SP",
             "A=M",
             "M=D", // Store the -1 (true) value in the stack
             format!("(lt.end.{})", index).as_str(),
             "@SP", // Increase the SP again
             "D=M+1",
             "M=D"]
                .join("\n")
    }

    fn and() -> String {
        vec![
              "@SP", // Load the address of @SP in A
              "A=M", // Load the value stored in @SP in A
              // Decrease the value in A by 1 (point to the last element
              // in the stack instead of the next empty space)
              "A=A-1",
              "D=M", // Store the value in Memory[A/@SP] in D
              "A=A-1", // Pop another element
              "M=D&M", // Binary `and` the already popped element with the new one
              "D=A+1", // Store the new SP memory address in D
              "@SP", // Point to the SP again and
              "M=D" // Store the new SP memory address at @SP
            ]
                .join("\n")
    }

    fn or() -> String {
        vec![
              "@SP", // Load the address of @SP in A
              "A=M", // Load the value stored in @SP in A
              // Decrease the value in A by 1 (point to the last element
              // in the stack instead of the next empty space)
              "A=A-1",
              "D=M", // Store the value in Memory[A/@SP] in D
              "A=A-1", // Pop another element
              "M=D|M", // Binary `or` the already popped element with the new one
              "D=A+1", // Store the new SP memory address in D
              "@SP", // Point to the SP again and
              "M=D" // Store the new SP memory address at @SP
            ]
                .join("\n")
    }

    fn not() -> String {
        vec![
              "@SP", // Load the address of @SP in A
              "A=M", // Load the value stored in @SP in A
              // Decrease the value in A by 1 (point to the last element
              // in the stack instead of the next empty space)
              "A=A-1",
              "D=M", // Store the value in Memory[A/@SP] in D
              "M=!D", // Store the binary negated value in memory
              "D=A+1", // Store the new SP memory address in D
              "@SP", // Point to the SP again and
              "M=D" // Store the new SP memory address at @SP
            ]
                .join("\n")
    }
}
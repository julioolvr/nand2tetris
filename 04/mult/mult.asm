// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Set R2 to 0
@R2
M=0

// Initialize a counter in R3 to do a conditional JMP
@R3
M=0

(LOOP)
// Compare R3's counter to R0
@R3
D=M
@R0
D=D-M

// If D-M == 0, it means I already summed enough, jump to the end
@END
D;JEQ

// Otherwise, sum R1 to R2 one more time
@R1
D=M
@R2
M=D+M

// Increment R3's counter
@R3
M=M+1

// And repeat
@LOOP
0;JMP

(END)
@END
0;JMP
// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed.
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Main loop
(START)

@KBD
D=M

// Clear if no key is pressed
@CLEAR
D;JEQ

// Paint otherwise
@0
D=!A
@fill
M=D
@FILL
0;JMP

(CLEAR)
@0
D=A
@fill
M=D
@FILL
0;JMP

// Main loop
@START
0;JMP

// ROUTINE: Paint screen with the value on @fill
(FILL)
// Initialize the offset to 0
@fillOffset
M=0

(FILLSTEP)
// Set currentPos to @SCREEN + the current offset
@SCREEN
D=A
@fillOffset
D=D+M
@currentPos
M=D

// Load the @fill value
@fill
D=M

// And set it in @currentPos
@currentPos
A=M
M=D

// Compare the offset it to the max possible offset
@fillOffset
D=M
@8191
D=D-A

// Back to the main loop if I reached the max offset
@START
D;JEQ

// Otherwsie increase the offset
@fillOffset
M=M+1

// And keep filling
@FILLSTEP
0;JMP
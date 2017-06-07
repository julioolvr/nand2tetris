// This file
@123


// has several
D=M


// commands

@456

// Check jumps
(Jumps)
A=D
A=D;JEQ
A=D;JMP
A=D;JGT
A=D;JLE

@someVar

// Check dest
(Destinations)
A=D
D=D
M=D
AD=M
AM=D
DM=A
ADM=A

@otherVar
@someVar

// Check comp
A+1
A-1
D|A
D|M
D
!A
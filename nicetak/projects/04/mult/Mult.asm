// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Set R2 as 0
@R2
M=0

// If R0 == 0, go to (END)
@R0
D=M
@END
D;JEQ

(LOOP)
    
    // Load R1 to D
    @R1
    D=M

    // R2 = R1 + R2
    @R2
    M=D+M

    // Decrement R0
    @R0
    M=M-1
    D=M

    @LOOP
    D;JGT // If R0 > 0, continue the loop
    
(END)
    @END    
    0;JMP // Infinite loop
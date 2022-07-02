// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.
    @2          // A <- 2
    M=0         // M[2] = 0
(BEGIN_WHILE)
    @1          // A <- 1
    D=M         // D <- M[1]
    @END_WHILE
    D;JLE       // Go to END_WHILE if D <= 0
    @0          // A <- 0
    D=M         // D <- M[0]   
    @2          // A <- 2
    M=M+D       // M[2] <- M[2] + D
    @1          // A <- 1
    M=M-1       // M[1] <- M[1] - 1
    @BEGIN_WHILE
    0;JMP       // Go back to BEGIN_WHILE 
(END_WHILE)
    @END_WHILE
    0;JMP       // End script

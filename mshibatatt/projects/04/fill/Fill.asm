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

// Put your code here.
(INFINITE_LOOP)
    @i
    M=1             // M[i] <- D
    @SCREEN         // set screen starting memory address in A register
    D=A
    @loc
    M=D             // M[loc] <- D

    @24576          // set memory address of keyboard in A register
    D=M             // D <- M[A] 
    @SET_WHITE  
    D;JEQ           // set color as white if D == 0 

(SET_BLACK)
    @color
    M=0             // M[color] <- 0
    M=M-1           // M <- -1
    @FILL_SCREEN
    0;JMP           // Go to FILL_SCREEN

(SET_WHITE)
    @color
    M=0             // M[color] <- 0

(FILL_SCREEN)
    @i
    D=M
    @8192
    D=D-A           // D <- i - (256*32)
    @END
    D;JGT           // Go to END if D > 0

    @color
    D=M             // D <- M[color]
    @loc            // set screen starting memory address in A register
    A=M             // A <- M[loc]
    M=D             // M[A] <- D

    @loc
    M=M+1           // loc <- loc+1
    @i
    M=M+1           // i <- i+1
    @FILL_SCREEN
    0;JMP           // Go back to fill screen if D > 0

(END)
    @INFINITE_LOOP
    0;JMP
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


(INIT)

    // 256 * 32 (each row has 32 pixels = 512 pixel / 16 bit )
    @8192
    D=A

    // Initialize index as 8192
    @i
    M=D

(LOOP)

    // Decrement the index
    @i
    M=M-1
    D=M

    // If index < 0, reinitialize. (INIT)
    @INIT
    D;JLT

    // Load keyboard address
    @KBD
    D=M

    // If the keyboard address is 0, it means no key boards are selected (WHITE)
    @WHITE
    D;JEQ

    // Else
    @BLACK
    0;JMP

(BLACK)

    // Load keyboard address
    @SCREEN
    D=A

    // The pixel index is @SCREEN (D) + @i (M)
    @i
    A=D+M
    M=-1 // Set the 16 pixels as black (-1 corresponds to 1111111111111111)

    // Go to (LOOP)
    @LOOP
    0;JMP

(WHITE)

    // Load keyboard address
    @SCREEN
    D=A

    // The pixel index is @SCREEN (D) + @i (M)
    @i
    A=D+M
    M=0 // Set the pixel as white (0)

    // Go to (LOOP)
    @LOOP
    0;JMP

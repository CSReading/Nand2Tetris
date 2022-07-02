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

(LOOP)
  @i
  M=0

(INLOOP)
  @i
  D=M
  @8191
  D=D-A
  @LOOP
  D;JGT

  @KBD
  D=M
  @WHITE
  D;JEQ

  @i
  D=M
  @SCREEN
  A=A+D
  M=-1

  @i
  M=M+1
  @INLOOP
  0;JMP

(WHITE)
  @i
  D=M
  @SCREEN
  A=A+D
  M=0

  @i
  M=M+1
  @INLOOP
  0;JMP


// メモ
//
// while (True) {
//   int i = 0;
//   while (i <= 8191) {
//     screen = SCREEN + i;
//     if (KBD != 0) {
//       Memory[screen] = -1;
//     } else {
//       Memory[screen] = 0;
//     }
//     i++;
//   }
// }

// そもそも無限ループのコードなので最後に無限ループを明示する必要なし

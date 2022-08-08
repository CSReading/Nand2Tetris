@256
D=A
@SP
M=D
@Main.RET_0
D=A
@SP
A=M
M=D
@SP
M=M+1
// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// LCL <- SP
@SP
D=M
@LCL
M=D
// ARG <- SP-n-5
@5
D=D-A
@ARG
M=D
// goto func
@Sys.init
0;JMP
// (return-address)
(Main.RET_0)
(Main.fibonacci)
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
A=A-1
D=M-D
@FALSE_0
D;JGE
@SP
A=M-1
M=-1
@CONTINUE_0
0;JMP
(FALSE_0)
@SP
A=M-1
M=0
(CONTINUE_0)
@SP
M=M-1
A=M
D=M
@Main.fibonacci$IF_TRUE
D;JNE
@Main.fibonacci$IF_FALSE
0;JMP
(Main.fibonacci$IF_TRUE)
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// R13 <- *(LCL - 5)
@LCL
D=M
@5
A=D-A
D=M
@R13
M=D
// *ARG <- pop()
@SP
A=M-1
D=M
@ARG
A=M
M=D
// SP <- ARG + 1
D=A+1
@SP
M=D
// THAT <- *(LCL-1); LCL--
@LCL
AM=M-1
D=M
@THAT
M=D
// THIS <- *(LCL-1); LCL--
@LCL
AM=M-1
D=M
@THIS
M=D
// ARG <- *(LCL-1); LCL--
@LCL
AM=M-1
D=M
@ARG
M=D
// LCL <- *(LCL-1); LCL--
@LCL
AM=M-1
D=M
@LCL
M=D
// goto R13
@R13
A=M
0;JMP
(Main.fibonacci$IF_FALSE)
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
A=A-1
M=M-D
@Main.RET_0
D=A
@SP
A=M
M=D
@SP
M=M+1
// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// LCL <- SP
@SP
D=M
@LCL
M=D
// ARG <- SP-n-5
@6
D=D-A
@ARG
M=D
// goto func
@Main.fibonacci
0;JMP
// (return-address)
(Main.RET_0)
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
A=A-1
M=M-D
@Main.RET_1
D=A
@SP
A=M
M=D
@SP
M=M+1
// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// LCL <- SP
@SP
D=M
@LCL
M=D
// ARG <- SP-n-5
@6
D=D-A
@ARG
M=D
// goto func
@Main.fibonacci
0;JMP
// (return-address)
(Main.RET_1)
@SP
AM=M-1
D=M
A=A-1
M=D+M
// R13 <- *(LCL - 5)
@LCL
D=M
@5
A=D-A
D=M
@R13
M=D
// *ARG <- pop()
@SP
A=M-1
D=M
@ARG
A=M
M=D
// SP <- ARG + 1
D=A+1
@SP
M=D
// THAT <- *(LCL-1); LCL--
@LCL
AM=M-1
D=M
@THAT
M=D
// THIS <- *(LCL-1); LCL--
@LCL
AM=M-1
D=M
@THIS
M=D
// ARG <- *(LCL-1); LCL--
@LCL
AM=M-1
D=M
@ARG
M=D
// LCL <- *(LCL-1); LCL--
@LCL
AM=M-1
D=M
@LCL
M=D
// goto R13
@R13
A=M
0;JMP
@256
D=A
@SP
M=D
@Sys.RET_0
D=A
@SP
A=M
M=D
@SP
M=M+1
// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// LCL <- SP
@SP
D=M
@LCL
M=D
// ARG <- SP-n-5
@5
D=D-A
@ARG
M=D
// goto func
@Sys.init
0;JMP
// (return-address)
(Sys.RET_0)
(Sys.init)
@4
D=A
@SP
A=M
M=D
@SP
M=M+1
@Sys.RET_0
D=A
@SP
A=M
M=D
@SP
M=M+1
// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// LCL <- SP
@SP
D=M
@LCL
M=D
// ARG <- SP-n-5
@6
D=D-A
@ARG
M=D
// goto func
@Main.fibonacci
0;JMP
// (return-address)
(Sys.RET_0)
(Sys.init$WHILE)
@Sys.init$WHILE
0;JMP

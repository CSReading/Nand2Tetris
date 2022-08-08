@256
D=A
@SP
M=D
@Class1.RET_0
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
(Class1.RET_0)
(Class1.set)
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
@Class1.0
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
@ARG
D=M
@1
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
@Class1.1
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
@0
D=A
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
(Class1.get)
@Class1.0
D=M
@SP
A=M
M=D
@SP
M=M+1
@Class1.1
D=M
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
@Class2.RET_0
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
(Class2.RET_0)
(Class2.set)
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
@Class2.0
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
@ARG
D=M
@1
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
@Class2.1
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
@0
D=A
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
(Class2.get)
@Class2.0
D=M
@SP
A=M
M=D
@SP
M=M+1
@Class2.1
D=M
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
@6
D=A
@SP
A=M
M=D
@SP
M=M+1
@8
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
@7
D=D-A
@ARG
M=D
// goto func
@Class1.set
0;JMP
// (return-address)
(Sys.RET_0)
@R5
D=A
@0
D=D+A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
@23
D=A
@SP
A=M
M=D
@SP
M=M+1
@15
D=A
@SP
A=M
M=D
@SP
M=M+1
@Sys.RET_1
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
@7
D=D-A
@ARG
M=D
// goto func
@Class2.set
0;JMP
// (return-address)
(Sys.RET_1)
@R5
D=A
@0
D=D+A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
@Sys.RET_2
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
@Class1.get
0;JMP
// (return-address)
(Sys.RET_2)
@Sys.RET_3
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
@Class2.get
0;JMP
// (return-address)
(Sys.RET_3)
(Sys.init$WHILE)
@Sys.init$WHILE
0;JMP

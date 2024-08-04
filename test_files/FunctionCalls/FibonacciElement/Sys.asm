@256
D=A
@0
M=D
//push returnAddr
@Sys.init$ret.0
D=A
@SP
A=M
M=D
@SP
M=M+1
//push lcl
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
//push arg
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
//push this
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
//push that
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
//arg=sp-5-nargs
@SP
D=M
@5
D=D-A
@0
D=D-A
@ARG
M=D
//lcl=sp
@SP
D=M
@LCL
M=D
//goto
@Sys.init
0; JMP
//label
(Sys.init$ret.0)
//function
@Sys.init
// push constant
@4
D=A
@SP
A=M
M=D
@SP
M=M+1
//push returnAddr
@Main.fibonacci$ret.1
D=A
@SP
A=M
M=D
@SP
M=M+1
//push lcl
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
//push arg
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
//push this
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
//push that
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
//arg=sp-5-nargs
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
//lcl=sp
@SP
D=M
@LCL
M=D
//goto
@Main.fibonacci
0; JMP
//label
(Main.fibonacci$ret.1)
//label
(WHILE)
//goto
@WHILE
0; JMP

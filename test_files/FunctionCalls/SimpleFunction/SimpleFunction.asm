//function
@SimpleFunction.test
//nvars
// push constant
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
//nvars
// push constant
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
//push local
@0
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
//push local
@1
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
//add
@SP
M=M-1
@SP
A=M
D=M
@SP
A=M-1
D=D+M
M=D
@SP
A=M-1
M=!M
//push argument
@0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
//add
@SP
M=M-1
@SP
A=M
D=M
@SP
A=M-1
D=D+M
M=D
//push argument
@1
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@SP
A=M
D=M
@SP
A=M-1
D=M-D
M=D
//frame=LCL
@LCL
D=M
@SP
A=M
M=D
@13
M=D
@SP
M=M-1
//retAddr=*(frame-5
@5
D=A
//frame-5
@R13
D=M-D
A=D
D=M
@14
M=D
//arg=pop()
@SP
A=M
D=M
@ARG
A=M
M=D
//SP=ARG+1
@ARG
D=M
@SP
M=D+1
//THAT=*(frame-1)
@R13
A=M-1
D=M
@THAT
M=D
//THIS=*(frame-2)
@2
D=A
@R13
A=M-D
D=M
@THIS
M=D
//ARG=*(frame-3)
@3
D=A
@R13
A=M-D
D=M
@ARG
M=D
//LCL=*(frame-4)
@4
D=A
@R13
A=M-D
D=M
@LCL
M=D
//goto
@R14
A=M
0; JMP

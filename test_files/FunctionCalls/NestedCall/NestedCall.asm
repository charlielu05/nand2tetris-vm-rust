//function
@Sys.init
//push returnAddr
@Sys.main$ret.0
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
@Sys.main
0; JMP
//label
(Sys.main$ret.0)
//pop temp
// decrement stack pointer
@SP
M=M-1
@1
D=A
@5
// temp address + index
D=A+D
// save to temp register
@R13
M=D
// get value of stack pointer
@SP
A=M
D=M
// save to arg address stored in temp register
@R13
A=M
M=D
//label
(LOOP)
//goto
@LOOP
0; JMP
//function
@Sys.main
// push constant
@123
D=A
@SP
A=M
M=D
@SP
M=M+1
//push returnAddr
@Sys.add12$ret.1
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
@Sys.add12
0; JMP
//label
(Sys.add12$ret.1)
//pop temp
// decrement stack pointer
@SP
M=M-1
@0
D=A
@5
// temp address + index
D=A+D
// save to temp register
@R13
M=D
// get value of stack pointer
@SP
A=M
D=M
// save to arg address stored in temp register
@R13
A=M
M=D
// push constant
@246
D=A
@SP
A=M
M=D
@SP
M=M+1
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
//function
@Sys.add12
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
//nvars
// push constant
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
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
// push constant
@12
D=A
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
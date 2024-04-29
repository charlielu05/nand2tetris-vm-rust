// push constant
@3030
D=A
@SP
A=M
M=D
@SP
M=M+1
//pop pointer
// decrement stack pointer
@SP
M=M-1
// get value of stack pointer
@SP
A=M
D=M
@THIS
M=D
// push constant
@3040
D=A
@SP
A=M
M=D
@SP
M=M+1
//pop pointer
// decrement stack pointer
@SP
M=M-1
// get value of stack pointer
@SP
A=M
D=M
@THAT
M=D
// push constant
@32
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop this
// decrement stack pointer
@SP
M=M-1
@2
D=A
@THIS
// THIS address + index
D=M+D
// save to temp register
@R13
M=D
// get value of stack pointer
@SP
A=M
D=M
// save to THIS address stored in temp register
@R13
A=M
M=D
// push constant
@46
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop that
// decrement stack pointer
@SP
M=M-1
@6
D=A
@THAT
// THAT address + index
D=M+D
// save to temp register
@R13
M=D
// get value of stack pointer
@SP
A=M
D=M
// save to THAT address stored in temp register
@R13
A=M
M=D
//push pointer
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
//push pointer
@THAT
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
D=D+M
M=D
//push this
@2
D=A
@THIS
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
//push that
@6
D=A
@THAT
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
D=D+M
M=D

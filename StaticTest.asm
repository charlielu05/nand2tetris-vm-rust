// push constant
@111
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant
@333
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant
@888
D=A
@SP
A=M
M=D
@SP
M=M+1
//pop static
// decrement stack pointer
@SP
M=M-1
// get value of stack pointer
@SP
A=M
D=M
@StaticTest.8
M=D
//pop static
// decrement stack pointer
@SP
M=M-1
// get value of stack pointer
@SP
A=M
D=M
@StaticTest.3
M=D
//pop static
// decrement stack pointer
@SP
M=M-1
// get value of stack pointer
@SP
A=M
D=M
@StaticTest.1
M=D
//push static
@StaticTest.3
D=M
@SP
A=M
M=D
@SP
M=M+1
//push static
@StaticTest.1
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
//push static
@StaticTest.8
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
@SP
M=M+1

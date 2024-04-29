// push constant
@10
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop local
// decrement stack pointer
@SP
M=M-1
@0
D=A
@LCL
// LCL address + index
D=M+D
// save to temp register
@R13
M=D
// get value of stack pointer
@SP
A=M
D=M
// save to LCL address stored in temp register
@R13
A=M
M=D
// push constant
@21
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant
@22
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop argument
// decrement stack pointer
@SP
M=M-1
@2
D=A
@ARG
// arg address + index
D=M+D
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
// pop argument
// decrement stack pointer
@SP
M=M-1
@1
D=A
@ARG
// arg address + index
D=M+D
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
@36
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
@6
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
@42
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant
@45
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
@5
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
// pop that
// decrement stack pointer
@SP
M=M-1
@2
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
// push constant
@510
D=A
@SP
A=M
M=D
@SP
M=M+1
//pop temp
// decrement stack pointer
@SP
M=M-1
@6
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
//push that
@5
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
//push this
@6
D=A
@THIS
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
//push this
@6
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
D=D+M
M=D
@SP
M=M-1
@SP
A=M
D=M
@SP
A=M-1
D=M-D
M=D
//push temp
@6
D=A
@5
A=A+D
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

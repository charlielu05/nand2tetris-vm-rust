// push constant
@0
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
//label
(LOOP_START)
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
@1
D=A
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
// pop argument
// decrement stack pointer
@SP
M=M-1
@0
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
//if-goto
@SP
AM=M-1
D=M
@LOOP_START
D;JNE
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

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
@0
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
@0
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
@1
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
@1
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
@2
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
//label
(MAIN_LOOP_START)
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
M=M-1
@COMPUTE_ELEMENT
D;JNE
//goto
@END_PROGRAM
0; JMP
//label
(COMPUTE_ELEMENT)
//push that
@0
D=A
@THAT
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
//push that
@1
D=A
@THAT
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
//push pointer
@THAT
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
//goto
@MAIN_LOOP_START
0; JMP
//label
(END_PROGRAM)

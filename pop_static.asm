//setting up SP address
@256
D=A
@SP
M=D
//setting up LCL address
@456
D=A
@LCL
M=D
//setting up ARG address
@756
D=A
@ARG
M=D
//setting up THIS address
@1056
D=A
@THIS
M=D
//setting up THAT address
@1356
D=A
@THAT
M=D
//pop static
// decrement stack pointer
@SP
M=M-1
// get value of stack pointer
@SP
A=M
D=M
@pop_static.555
A=M
M=D
@SP
M=M+1

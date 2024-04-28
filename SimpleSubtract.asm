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
// push constant
@7
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant
@8
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
@SP
M=M+1

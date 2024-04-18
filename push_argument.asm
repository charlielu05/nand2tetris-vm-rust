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
//push argument
@10
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

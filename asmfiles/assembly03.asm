MOV X0, #100            
MOV X1, #5              
MOV X2, #16             
MOV X3, X1, LSL #2
ADD X4, X0, X2, LSR #2
SUB X5, X0, X2, ASR #1
AND X6, X0, X1, ROR #1
MOV X7, X1, LSL #1
MUL X8, X0, X7
MOV X8, #93
MOV X0, #0
SVC #0

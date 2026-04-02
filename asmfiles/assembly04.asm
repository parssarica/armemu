MOV X0, #16
MOV X1, #16
CMP X0, X1
B label2
label1:
MOV X2, #10
B label3
label2:
MOV X3, #10
B label1
label3:
MOV X19, #314
STR X19, [X19]
LDR X16, [X19]

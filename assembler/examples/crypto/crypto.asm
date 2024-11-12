LDI r1 0x69 ; Plain text
LDI r2 0x42 ; Key
OR r1 r2 r3
AND r1 r2 r4
NOT r4 r4
AND r3 r4 r3 ; XOR by boolean algebra to encrypt r1 with r2 into r3
DIS r3
NOP
NOP
NOP
XOR r3 r2 r4 ; XOR r3 with r2 to get back r1 hopefully
DIS r4
HLT


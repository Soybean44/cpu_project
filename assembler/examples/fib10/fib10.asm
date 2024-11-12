DIS r0
LDI r1 0x00 ; fib 1 number
LDI r2 0x01 ; fib 2 number
LDI r3 0x0A ; loop idx
LDI r4 0x02 ; decrement by 2
ADD r1 r2 r1
DIS r1
ADD r1 r2 r2
DIS r2
SUB r3 r4 r3
JNZ r3 0x05
HLT


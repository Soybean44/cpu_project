LDI r1 0x80 ; Setting up the output
STR r1 0x00
STR r0 0x01 ; Outputing 0
LDI r1 0x00
LDI r2 0x01
LDI r3 0x02
ADD r1 r2 r1
STR r1 0x01 ; Outputting r1
JE r3 0x06
HLT

LDI r1 0x80 ; switch to bank 0x80 to access output and input ports
STR r1 0x00  
STR r0 0x01 ; set output to 0
LDI r1 0x05
LDI r2 0x04
ADD r1 r2 r3
STR r3 0x01 ; set output to r3
HLT


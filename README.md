# CPU Project
This project is my attempt at making an 8-bit cpu in logisim, originally derived from matbatwing's redstrone computer series but deviated to become its own thing.

# ISA Description
The ISA description is in the ISA.ods file which can be opened by Libreoffice Calc or any other compatible spreadsheet viewer. 
There all the available instructions exist, but some tricks in using the ISA to its fullest potential are not documented.

### ISA tips and tricks
The only jump instruction is `JE` which is jump if equals. 
It checks the passed in register for flags outputed by the `CMP` instruction and jumps to the specified address if the flag is enabled.
To emulate unconditional jumps, just manually set the second bit of the register, by writing 2 to it, or `CMP r0 r0 rN`.
To emulate a jump if greater or less than, shift the registers from the cmp instruction, (left shift is the same as adding a register to itself).
The CMP instruction formats the first three registers from left to right as "greater than, equals, less than"
Since right shift is a built in to the cpu, it is recomended to use less than whenever possible, flipping the arguments of the `CMP` instruction

### Memory and outputting 
Currently the memory is not intrinsically tied to the cpu, so you can implement whatever memory solution you would like to have.
The cpu can only access 265 bytes of memory at a time, but with banking, it allows the cpu to index up to 64k of memory to use (as of the current setup).
As well with the current setup, you can access one 8 bit output port and one 8 bit input port, however more may be implemented in the future if needed.

# DAHLIA-86
The DIRT-16 Advanced High-Level Instruction Assembler, or "DAHLIA" for brevity, is an assembler for the DIRT-16 fantasy computer. This project is the x86 version, though in Rust it should be portable to anywhere LLVM supports. In the future there will be a version of DAHLIA for the DIRT-16, though it seems likely that that will initially be assembled by this x86 version. Lower level details of DIRT-16 assembly and machine code will be documented in this README file.

## Usage
`DAHLIA [ASM file] -o [binary file]`

# DIRT-16 Assembly
- Syntax
	- INSTR arg0, arg1, arg2
- Argument types
	- rx: r0 | r1 | r2 | r3 
	- rx_n: r0 | r1 | r2 | r3 | null
	- $ADDR: $#NUM | $(#NUM + rx_n) | $rx
	- #NUM: #HEXNUM | d#DECNUM | b#BINNUM
	- {r0, r1, r2, r3, PC, SR, SP}: any of the inner arguments can be omitted, but there must be at least one present
	- Ex: "LDW rx, $ADDR" could be LDW r0, $(#010000 + r1) to do indexed addressing for (0x010000 + r1) -> r0
	- in machine code, $rx is the same as $(#0 + rx)
- ASM Instructions
    - LDB rx_n $ADDR, load byte
    - STB rx_n $ADDR, store byte
    - LDW rx_n $ADDR, load word
    - STW rx_n $ADDR, store word
    - ADD rx_n ry rz, ry + rz -> rx_n, add
    - SUB, rx_n ry rz, subtract
    - MUL, rx_n ry rz, multiply
    - DIV, rx_n ry rz, divide
    - PSH {r0, r1, r2, r3, PC, SR, SP}, push to stack (addressing mode is a 1 or 0 for bit that corresponds to register)
    - POP {r0, r1, r2, r3, PC, SR, SP}, pop from stack (addressing mode same as PSH)
    - NOP, No-operation
    - BRA $ADDR, branch
    - CLx, clear status bit
        - CLP, CLQ, CLR, CLS, CLN, CLV, CLZ, CLC 
    - SEx, set status bit
        - SEP, SEQ, SER, SES, SEN, SEV, SEZ, SEC
    - INC rx, increment register
    - DEC rx, decrement register
    - HLT, stop execution
    - SWP rx ry, swap registers
    - CMP rx ry, compare registers (subtract and set bits, but don't store result)
    - SRR rx_n ry rz, shift register right
    - SRL rx_n ry rz, shift register left
    - AND rx_n ry rz, bitwise AND
    - OR rx_n ry rz, bitwise OR
    - XOR rx_n ry rz, bitwise XOR
    - NOT rx_n ry rz, bitwise NOT
- Conditional execution a-la ARM (BRA, beq as branch and branch if equal)
    - xAL, always executes (normally omitted) (0x0)
    - xEQ, Z set (0x1)
    - xNE, Z clear (0x2)
    - xCS, C set (0x3)
    - xCC, C clear (0x4)
    - xMI, N set (0x5)
    - xPL, N clear (0x6)
    - xVS, V set (0x7)
    - xVC, V clear (0x8)
    - xHI, C set and Z clear (0x9)
    - xLS, C clear or Z set (0xA)
    - xGE, N=V (0xB)
    - xLT, N!=V (0xC)
    - xGT, Z clear, N=V (0xD)
    - xLE, Z set, N!=V (0xE)
- Status Register, 8-bit (PQRSNVZC)
    - IRQ1 disable (P)
    - IRQ2 disable (Q)
    - IRQ3 disable (R)
    - IRQ4 disable (S)
    - Negative (N)
    - Overflow (V)
    - Zero (Z)
    - Carry (C)


# DIRT-16 Machine Code
- Opcode structure
    - 5 bit instruction,  7 bit addressing mode, 4 bit condition
	```
	0000000000000000
	----- instruction
		 ------- addressing mode
				---- conditional execution
	```
- Opcodes (5 bits)
	- HLT: 0x00
	- LDW/LDB: 0x01
	- STW/STB: 0x02
	- SWP: 0x03
	- PSH: 0x04
	- POP: 0x05
	- BRA: 0x06
	- NOP: 0x07
	- CL{P, Q, R, S, N, V, Z, C}: 0x0F
	- ADD: 0x10
	- SUB: 0x11
	- MUL: 0x12
	- DIV: 0x13
	- INC/DEC: 0x14
	- AND: 0x15
	- OR: 0x16
	- XOR: 0x17
	- NOT: 0x18
	- SRR/SRL: 0x19
	- SE{P, Q, R, S, N, V, Z, C}: 0x1F

- Adressing modes (7 bits)
	- rx (INC/DEC)
		- total num of modes: 4 (rx)
			- r0: 0x00
			- r1: 0x01
			- r2: 0x02
			- r3: 0x03
	- rx ry (SWP)
		- total num of modes: 4 (rx) * 4 (ry) = 16
			- bits 0-1: rx
			- bits 2-3: ry
	- $ADDR
		- total num of modes: 1 ($#NUM) + 5 ($#NUM + $rx_n) = 6
			- $#NUM: 0x00
			- $(#NUM + rx\_n): 0x1 + rx_n
				- r0: 0x10
				- r1: 0x11
				- r2: 0x12
				- r4: 0x13
				- null: 0x14
	- rx_n $ADDR (LDW/LDB, STW/STB)
		- rx_n so you can read for the sake of reading
		- total num of modes: 5 (rx_n) * 5 (#NUM + rx_n) = 30)
			- 
	- rx_n, ry, rz (ADD, SUB, MUL, DIV, SRR, SRL)
		- rx_n can be null, do math for the sake of setting SR flags
		- total num of modes: 5 (rx_n) * 4 (ry) * 4 (rz) = 80 -> 7 bits
			- bits 0-2: rx_n
			- bits 3-4: ry
			- bits 5-6: rz
	- {r0, r1, r2, r3, PC, SR, SP} (PSH, POP)
		- each bit in mode corresponds to thing you can pop/push to stack (e.g. POP {r0, r2, r3, SP} -> 0b00101 1011001 0000)
	- Clear/set (CLx, SEx)
		- just enumerate each status flag
		- total num of modes: 8 -> 3 bits
			- P: 0x00
			- Q: 0x01
			- R: 0x02
			- S: 0x03
			- N: 0x04
			- V: 0x05
			- Z: 0x06
			- C: 0x07
	


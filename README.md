# DIRT-16
The DIRT-16 is a 16-bit fantasy computer for recreational computing and retro-inspired game development. Features are described below.
It'll be a part of a fictional universe and made by a company called Generator Inc. whose logo is based off of a dahlia.

## Project Goals
- 16-bit CPU
- Runs on binaries (Will provide assembler and language with compiler)
- Capable of graphics and sound (3d if possible)
- Frontend that emulates screen and data drive interface
- More powerful than the SNES but less than the N64
- Circuit-switched networking a-la dial-up

## Todo list
- [ ] Implement Backend (CPU emulation library and related things)
- [ ] Write assembler for machine (DASM is already taken, maybe a play on wagon like WAGN? Or DAHLIA from Gen. Inc. logo? DIRT16-Assembly-Handling Linking Assembler?)
- [ ] Write compiler for machine (Language called DIRTH and similar to FORTH? Who am I kidding it'll be a lisp, maybe call it stutter or asparagus. ASPRGS is fun because it could be asparagus or aspergers?)
- [ ] Make test binaries for machine
- [ ] Implement Frontend (Computer with screen reading from framebuffer and drive port to place ports in)
- [ ] Create assets for frontend
- [ ] Clearer documentation of technical aspects

## System Specification
- 24MHz master clock, 6MHz CPU clock (i.e. 4 master clock cycles per CPU cycle)
- 16-bit word size, 32-bit addr size
- 512kb memory, 512kb data drive
- 640x480 framebuffer

## CPU Specification
- 6MHz
- 4 registers (r0, r1, r2, r3)
- Program Counter
- Stack Pointer
- Status Register, 8-bit (IONVZCUU)
    - IRQ1 disable (I)
    - IRQ2 disable (O)
    - Negative (N)
    - Overflow (V)
    - Zero (Z)
    - Carry (C)
    - 2 status bits unused (UU)
- Interrupts
    - IRQ1
    - IRQ2
    - NMI (non-maskable)
    - RESET
    - ABORT
- Interrupt vectors at end of ROM a-la 6502
- Conditional execution a-la ARM (b, beq as branch and branch if equal)
    - xEQ, Z set
    - xNE, Z clear
    - xCS, C set
    - xCC, C clear
    - xMI, N set
    - xPL, N clear
    - xVS, V set
    - xVC, V clear
    - xHI, C set and Z clear
    - xLS, C clear or Z set 
    - xGE, N=V
    - xLT, N!=V
    - xGT, Z clear, N=V
    - xLE, Z set, N!=V
    - xAL, always executes (normally omitted)
- Opcode structure
```
    0000000000000000
    ----- instruction
         ------- addressing mode
                ---- conditional execution
```
- Argument types
    - Rx
    - $ADDR
    - $(ADDR+Rx)
    - #HEXNUM
    - d#DECNUM
    - b#BINNUM
- ASM Instructions
    - LDR Rx $ADDR
    - STR Rx $ADDR
    - ADD Rx Ry Rz, Ry + Rz -> Rx
    - SUB, same args as ADD
    - MUL, same args as ADD
    - DIV, same args as ADD
    - POP {Rx, PC, SR, SP}
    - PSH {Rx, PC, SR, SP}
    - NOP
    - BRA $ADDR
    - CLx
        - CLI, CLO, CLN, CLV, CLZ, CLC 
    - SEx
        - SEI, SEO, SEN, SEV, SEZ, SEC
    - INC Rx
    - DEC Rx
    - STP
    - SWP Rx Ry
    - CMP Rx Ry
    - SRR Rx, (shift)
    - SRL Rx
    - three potential opcodes left for 4 bit instruction code

# DIRT-16
DIRT-16 is a 16-bit fantasy computer for recreational computing and retro-inspired game development. Features are described below.
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
- [ ] Write assembler for machine (DIRT16 Advanced High Level Instruction Assembler, or DAHLIA)
- [ ] Write compiler for machine (DIRTH, language based off of FORTH)
- [ ] Make test binaries for machine
- [ ] Implement Frontend (Computer with screen reading from framebuffer and drive port to place ports in)
- [ ] Create assets for frontend
- [ ] Clearer documentation of technical aspects

## System Specification
- 32MHz master clock, 8MHz CPU clock
- Big endian
- 16-bit word size, 24-bit addr size
- 512KB memory, 512KB data drive
- 480x320 8-bit color framebuffer (150KB)
- Hardware blitter for rectangles and sprites

## Low-level Specification

### CPU Spec
- 8MHz
- 4 registers (r0, r1, r2, r3)
- 24-bit Program Counter
- 24-bit Stack Pointer, full descending
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
    - LDR Rx $ADDR (with single byte or full word addressing modes)
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

### Hardware Blitter Spec
    - Does not share memory with CPU, internals locked off from user
    - CPU handles sprite tables
    - Registers
        - BLT_SRC_HB, source addr in mem high byte (8-bit)
        - BLT_SRC_LW, source addr in mem low word (16-bit)
        - BLT_DST, dest addr in mem (16-bit offset from framebuffer start)
        - BLT_W, width of rectangle (16-bit)
        - BLT_H, height of rectangle (16-bit)
        - BLT_COL, colour of rectangle if just filling, colour to make transparent if pasting (8-bit)
        - BLT_CTRL, control flags (8-bit)
            - bits 0-1 are blit mode, 0x0 to fill rect, 0x1 to copy rect, 0x2 to paste rect, 0x3 to paste rect w/ no transparency
            - bits 2-3 are rotation, 00 is no rotation, 01 is 90 degree rotation, 10 is 180 degrees, 11 is 270
            - bits 4-5 are flip 00 is no flip, 01 horizontal, 10 vertical, 11 horizontal and vertical)
        - BLT_CMD, write to start operation (8-bit)
        - BLT_STATUS, read to check completion status (8-bit)
    - Clips automatically if not within framebuffer bounds

### Sound System Spec
- 6x Waveform channels
    - Select pulse, triangle, noise, sawtooth

### Networking Spec
- Current ideas are rough
- Switchboard server program that acts as middleman
- Circuit-switched
- Phone numbers assigned to users
- Bandwidth limit (~4 kbit/s)
- Latency (70ms added)

### Software Toolchain
#### DAHLIA
- DIRT-16 Advanced High Level Instruction Assembler

#### DIRTH
- Programming language based off of FORTH

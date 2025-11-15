; ===============
; Draw a rectangle
; ===============
; Simple program that draws a rectangle on the screen.

#include "../asm/cpu.asm"

start:
    LDI R2 2 ; X step
    LDI R3 1 ; Y step
.sprites:
    LDI R7 0b1111_1111 ; Initialize R7 with 0xF
    LDI_I 0x123;
    ST R7
    INC_I R2
    ST R7
    INC_I R2
    ST R7
    INC_I R2
    ST R7
.draw:
    LDI R0 0x8 ; X
    LDI R1 0x8 ; Y
    LDI_I 0x123
.loop:
    DRAW R0 R1 4
    ADD R0 R2
    ADD R0 R2
    ADD R1 R3
    JMP .loop

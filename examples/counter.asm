; ===============
; Counter program
; ===============
; This program counts from 16 to 256 (overflow) and then halts.

start:
    LDI R0 0x10 ; Initialize counter to 16
    LDI R1 1 ; Initialize increment value to 1
loop:
    ADD R0 R1 ; Increment counter by 1
    JNZ loop ; Jump to loop if counter is not zero
    HALT ; Halt the program

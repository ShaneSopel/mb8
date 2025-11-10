; ============
; Subroutines example
; ============
; This example demonstrates the use of subroutines in mb8 assembly language.

#include "../asm/cpu.asm"

start:
    LDI R0 1 ; Initialize R0 register to 1
    CALL s1 ; Call subroutine s1
    HALT 0 ; Halt the program with status code 0
    ; At the end of the program R0 is still 1 because it was not modified by any subroutine
s1:
    CALL s2 ; Call subroutine s2
    RET ; Return from subroutine s1
    ; The following instructions will never be executed
    LDI R0 2
    HALT 1
s2:
    CALL s3 ; Call subroutine s3
    RET ; Return from subroutine s2
    ; The following instructions will never be executed
    LDI R0 3
    HALT 1
s3:
    RET ; Return from subroutine s3
    ; The following instructions will never be executed
    LDI R0 4
    HALT 1

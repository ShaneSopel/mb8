; ============
; Stack overflow example
; ============
; This example demonstrates a stack overflow by recursively calling a function that adds a new stack frame to the stack.

#include "../asm/cpu.asm"

start:
    JMP new_stack_frame ; jump to new_stack_frame
new_stack_frame:
    CALL new_stack_frame ; call new_stack_frame, it will add a new stack frame to the stack
    RET

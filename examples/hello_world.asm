; ============
; Hello World Example
; ============
; This example demonstrates a simple "Hello World" program that prints the message to the console.

#include "../asm/cpu.asm"

#fn putc(char) => asm {
    LDI R0 { char }
    PUTC R0
}

start:
    #d putc("h")
    #d putc("e")
    #d putc("l")
    #d putc("l")
    #d putc("o")
    #d putc(" ")
    #d putc("w")
    #d putc("o")
    #d putc("r")
    #d putc("l")
    #d putc("d")
    #d putc("\n")
    HALT

# MicroBot-8bit

MicroBot-8bit is a simple, 8-bit virtual machine.

# Running

## Compile assembly

MB8 provides a command-line interface for compiling assembly code into MB8 machine instructions. To compile an assembly file, run the following command:

```
cargo run --bin mb8a <file.asm>
```

Compiled binary files have the extension `.bin`.

## Run VM

To run the compiled binary, use the following command:

```
cargo run --bin mb8 <file.bin>
```

## Assembly

You can see examples of assembly code in the [`examples`](examples) directory.

- [`counter.asm`](examples/counter.asm) - A simple counter program that increments a register from 0x10 to 0xFF and halts.
- [`stack_overflow.asm`](examples/stack_overflow.asm) - A program that demonstrates a stack overflow.
- [`subroutines.asm`](examples/subroutines.asm) - A program that demonstrates a subroutine call.

You can compile any of them and run them using the following commands:

```
cargo run --bin mb8a ./examples/counter.asm && cargo run --bin mb8 ./out.bin
```

# Architecture

## Registers

Virtual machine contains the following registers:

<table>
    <tr>
        <th>register</th>
        <th>mask</th>
        <th>description</th>
    </tr>
    <tr>
        <td>R0 - R7</td>
        <td>0x00 - 0x07</td>
        <td>General purpose registers</td>
    </tr>
    <tr>
        <td>SP</td>
        <td>0x0D</td>
        <td>Stack pointer</td>
    </tr>
    <tr>
        <td>PC</td>
        <td>0x0E</td>
        <td>Program counter</td>
    </tr>
    <tr>
        <td>F</td>
        <td>0x0F</td>
        <td>Flags register</td>
    </tr>
</table>

Registers 0x08 - 0x0C are reserved for future use.

## Flags

Virtual machine handles the following flags:

<table>
    <tr>
        <th>flag</th>
        <th>mask</th>
        <th>description</th>
    </tr>
    <tr>
        <td>Z</td>
        <td>0x01</td>
        <td>Zero flag. If the result of an operation is zero, this flag is set.</td>
    </tr>
    <tr>
        <td>N</td>
        <td>0x02</td>
        <td>Negative flag. If the result of an operation is negative, this flag is set.</td>
    </tr>
    <tr>
        <td>C</td>
        <td>0x04</td>
        <td>Carry flag. If the result of an operation is greater than 255, this flag is set. If the result of an operation is less than 0, this flag is set.</td>
    </tr>
</table>

Flags 0x08, 0x10, 0x20, 0x40, 0x80 are reserved for future use.

## Opcodes

An instruction for mb8 is 16 bits wide, represented as 0xABCD, where:
- A — the most significant nibble (4 bits) — instruction group
- B — sub-opcode, register, or flag, depending on the group
- C — typically a register or the upper 4 bits of an address
- D — typically a register or the lower 4 bits of an address

For example, the binary representation of the instruction `ADD R0, R1` is `0x1101`:
```
0001 0001 0000 0001
```

In branch/load/store operations, the address is 12 bits (XXX in 0xYXXX), similar to CHIP-8, allowing addressing up to 4 KiB of memory.

Virtual machine handles the following opcodes:

<table>
    <tr>
        <th>asm</th>
        <th>opcode</th>
        <th>short description</th>
    </tr>
    <tr>
        <td colspan="3">0x0 GROUP: System operations</td>
    </tr>
    <tr>
        <td>NOP</td>
        <td>0x0000</td>
        <td>No operation</td>
    </tr>
    <tr>
        <td>HALT</td>
        <td>0x01XX</td>
        <td>Stop the execution (in future XX exit code will be added)</td>
    </tr>
    <tr>
        <td colspan="3">0x1 REG-REG GROUP: Operations with two registers. ALU operations</td>
    </tr>
    <tr>
        <td>MOV dst src</td>
        <td>0x10AB</td>
        <td>Move data from B register to A register</td>
    </tr>
    <tr>
        <td>ADD dst src</td>
        <td>0x11AB</td>
        <td>Put the sum of A and B registers into A register</td>
    </tr>
    <tr>
        <td>SUB dst src</td>
        <td>0x12AB</td>
        <td>Put the difference of A and B registers into A register</td>
    </tr>
    <tr>
        <td colspan="3">0x2 LDI</td>
    </tr>
    <tr>
        <td>LDI dst value</td>
        <td>0x2AXX</td>
        <td>Load immediate XX value into A register</td>
    </tr>
    <tr>
        <td colspan="3">0x3 - 0x5 JUMP GROUP: Jump instructions</td>
    </tr>
    <tr>
        <td>JMP addr</td>
        <td>0x3XXX</td>
        <td>Jump to XXX address</td>
    </tr>
    <tr>
        <td>JZ addr</td>
        <td>0x4XXX</td>
        <td>Jump to XXX address if Flag register has zero flag</td>
    </tr>
    <tr>
        <td>JNZ addr</td>
        <td>0x5XXX</td>
        <td>Jump to XXX address if Flag register does not have zero flag</td>
    </tr>
    <tr>
        <td colspan="3">0x6 - 0x7 STACK: Stack and subroutine operations</td>
    </tr>
    <tr>
        <td>CALL addr</td>
        <td>0x6XXX</td>
        <td>Call subroutine at XXX address</td>
    </tr>
    <tr>
        <td>RET</td>
        <td>0x7000</td>
        <td>Return from subroutine</td>
    </tr>
    <tr>
        <td>PUSH dst</td>
        <td>0x71A0</td>
        <td>Push data from A register onto stack</td>
    </tr>
    <tr>
        <td>POP src</td>
        <td>0x72A0</td>
        <td>Pop data from stack into A register</td>
    </tr>
    <tr>
        <td colspan="3">0x8 - 0x9 MEMORY: Memory operations</td>
    </tr>
    <tr>
        <td>LD addr</td>
        <td>0x8XXX</td>
        <td>Load data from memory address XXX into R7 register</td>
    </tr>
    <tr>
        <td>ST addr</td>
        <td>0x9XXX</td>
        <td>Store data from R7 register into memory address XXX</td>
    </tr>
</table>

## Stack

Stack size in mb8 VM is the first 256 bytes in the beginning of the memory.

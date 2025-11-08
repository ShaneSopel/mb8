//! Opcodes for the MB8 ISA.
//! This module defines the opcodes used by the MB8 ISA.

use crate::registers::Register;

/// Full list of MB8 opcodes used in VM.
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /* Control group */
    /// No operation. Instruction does nothing.
    Nop,
    /// Halt the VM.
    Halt,

    /* reg-reg opcodes */
    /// Move value from one register to another.
    Mov {
        dst: Register,
        src: Register,
    },
    /// Add `dst` and `src1` and store the result in `dst`.
    Add {
        dst: Register,
        src: Register,
    },
    /// Subtract `src` from `dst` and store the result in `dst`.
    Sub {
        dst: Register,
        src: Register,
    },

    /* Load */
    Ldi {
        dst: Register,
        value: u8,
    },

    /* Jump instructions */
    /// Jump to address `addr`.
    Jmp {
        addr: u16,
    },
    /// Jump to address `addr` if flag register has zero flag
    Jz {
        addr: u16,
    },
    /// Jump to address `addr` if flag register does not have zero flag
    Jnz {
        addr: u16,
    },
    /* Stack instructions */
    /// Call subroutine at address `addr`.
    Call {
        addr: u16,
    },
    /// Return from subroutine.
    Ret,
    // /// Push value from register `src` onto stack.
    // Push {
    //     src: Register,
    // },
    // /// Pop value from stack into register `dst`.
    // Pop {
    //     dst: Register,
    // },

    /* Memory instructions */
    /// Load value from memory address `addr` into register `dst`.
    Ld {
        dst: Register,
        addr: u16,
    },
    /// Store value from register `src` into memory address `addr`.
    St {
        src: Register,
        addr: u16,
    },
}

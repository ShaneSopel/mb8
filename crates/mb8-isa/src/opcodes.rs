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
}

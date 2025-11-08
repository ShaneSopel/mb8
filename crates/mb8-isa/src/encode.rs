use crate::{opcodes::Opcode, registers::Register};

/// Encode a Register into a 4-bit value.
#[must_use]
pub fn encode_register(register: Register) -> u8 {
    match register {
        Register::R0 => 0x0,
        Register::R1 => 0x1,
        Register::R2 => 0x2,
        Register::R3 => 0x3,
        Register::R4 => 0x4,
        Register::R5 => 0x5,
        Register::R6 => 0x6,
        Register::R7 => 0x7,
        Register::SP => 0xD,
        Register::PC => 0xE,
        Register::F => 0xF,
    }
}

/// Encode a Program into a Vec<u8>.
#[must_use]
pub fn encode_program(program: &[Opcode]) -> Vec<u8> {
    program
        .iter()
        .flat_map(|opcode| encode(opcode).to_be_bytes())
        .collect()
}

/// Encode an Opcode into a 16-bit instruction.
#[must_use]
pub fn encode(opcode: &Opcode) -> u16 {
    match opcode {
        Opcode::Nop => 0x0000,
        Opcode::Halt => 0x0100,
        Opcode::Mov { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1000 | (dst as u16) << 4 | src as u16
        }
        Opcode::Add { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1100 | (dst as u16) << 4 | src as u16
        }
        Opcode::Sub { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1200 | (dst as u16) << 4 | src as u16
        }
        Opcode::Ldi { dst, value } => {
            let dst = encode_register(*dst);
            0x2000 | (dst as u16) << 4 | *value as u16
        }
        Opcode::Jmp { addr } => 0x3000 | (*addr & 0xFFF),
        Opcode::Jz { addr } => 0x4000 | (*addr & 0xFFF),
        Opcode::Jnz { addr } => 0x5000 | (*addr & 0xFFF),
        Opcode::Call { addr } => 0x6000 | (*addr & 0xFFF),
        Opcode::Ret => 0x7000,
        // Opcode::Push { src } => {
        //     let src = encode_register(*src);
        //     0x7000 | src as u16
        // }
        // Opcode::Pop { dst } => {
        //     let dst = encode_register(*dst);
        //     0x7000 | dst as u16
        // }
        Opcode::Ld { addr } => 0x8000 | (*addr & 0xFFF),
        Opcode::St { addr } => 0x9000 | (*addr & 0xFFF),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_register() {
        assert_eq!(encode_register(Register::R0), 0x0);
        assert_eq!(encode_register(Register::R1), 0x1);
        assert_eq!(encode_register(Register::R2), 0x2);
        assert_eq!(encode_register(Register::R3), 0x3);
        assert_eq!(encode_register(Register::R4), 0x4);
        assert_eq!(encode_register(Register::R5), 0x5);
        assert_eq!(encode_register(Register::R6), 0x6);
        assert_eq!(encode_register(Register::R7), 0x7);
        assert_eq!(encode_register(Register::SP), 0xD);
        assert_eq!(encode_register(Register::PC), 0xE);
        assert_eq!(encode_register(Register::F), 0xF);
    }

    #[test]
    fn test_encode_program() {
        let program = vec![Opcode::Nop, Opcode::Halt];
        assert_eq!(encode_program(&program), vec![0x00, 0x00, 0x01, 0x00]);
    }

    #[test]
    fn test_encode_nop() {
        assert_eq!(encode(&Opcode::Nop), 0x0000);
    }

    #[test]
    fn test_encode_halt() {
        assert_eq!(encode(&Opcode::Halt), 0x0100);
    }

    #[test]
    fn test_encode_mov() {
        assert_eq!(
            encode(&Opcode::Mov {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1001
        );
    }

    #[test]
    fn test_encode_add() {
        assert_eq!(
            encode(&Opcode::Add {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1101
        );
    }

    #[test]
    fn test_encode_sub() {
        assert_eq!(
            encode(&Opcode::Sub {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1201
        );
    }

    #[test]
    fn test_encode_ldi() {
        assert_eq!(
            encode(&Opcode::Ldi {
                dst: Register::R0,
                value: 0x12
            }),
            0x2012
        );
    }

    #[test]
    fn test_encode_jmp() {
        assert_eq!(encode(&Opcode::Jmp { addr: 0x123 }), 0x3123);
    }

    #[test]
    fn test_encode_jz() {
        assert_eq!(encode(&Opcode::Jz { addr: 0x123 }), 0x4123);
    }

    #[test]
    fn test_encode_jnz() {
        assert_eq!(encode(&Opcode::Jnz { addr: 0x123 }), 0x5123);
    }

    #[test]
    fn test_encode_call() {
        assert_eq!(encode(&Opcode::Call { addr: 0x123 }), 0x6123);
    }

    #[test]
    fn test_encode_ret() {
        assert_eq!(encode(&Opcode::Ret), 0x7000);
    }

    #[test]
    fn test_encode_ld() {
        assert_eq!(
            encode(&Opcode::Ld { addr: 0x123 }),
            0x8000 | (Register::R0 as u16) << 4 | 0x123
        );
    }

    #[test]
    fn test_encode_st() {
        assert_eq!(
            encode(&Opcode::St { addr: 0x123 }),
            0x9000 | (Register::R0 as u16) << 4 | 0x123
        );
    }
}

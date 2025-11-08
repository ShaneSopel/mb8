use mb8_isa::registers::{Register, OVERFLOW_FLAG};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn sub(&mut self, dst: Register, src: Register) {
        let a = u8::try_from(self.registers.read(dst)).unwrap_or_default();
        let b = u8::try_from(self.registers.read(src)).unwrap_or_default();
        let (result, overflow) = a.overflowing_sub(b);
        self.registers.write(dst, u16::from(result));
        if overflow {
            self.registers.write(Register::F, u16::from(OVERFLOW_FLAG));
        }
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_sub() {
        // VM subtracts two registers and stores the result in a third register
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::R0, 5);
        vm.registers.write(Register::R1, 3);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 2);
    }

    #[test]
    fn test_opcode_sub_overflow() {
        // VM handles subtraction overflow by wrapping around and setting the carry flag
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::R0, 1);
        vm.registers.write(Register::R1, 2);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 255);
        assert_eq!(vm.registers.read(Register::F), 1);
    }
}

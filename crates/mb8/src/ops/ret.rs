use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn ret(&mut self) {
        let stack_pointer = self.registers.read(Register::SP);
        if stack_pointer < 2 {
            self.halted = true;
            return;
        }
        let (addr, stack_pointer) = self.mem.pop_u16(stack_pointer);
        self.registers.write(Register::SP, stack_pointer);
        self.registers.write(Register::PC, addr);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::{opcodes::Opcode, STACK_SIZE};

    use super::*;

    #[test]
    fn test_opcode_ret() {
        // VM returns from a subroutine
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::PC, 0x100);
        vm.execute(&Opcode::Call { addr: 0x100 });
        assert_eq!(vm.registers.read(Register::SP), 2);
        assert_eq!(vm.registers.read(Register::PC), 0x200);
        assert_eq!(vm.mem.read_u16(STACK_SIZE - 2), 0x100);
        vm.execute(&Opcode::Ret);
        assert_eq!(vm.registers.read(Register::SP), 0);
        assert_eq!(vm.registers.read(Register::PC), 0x100);
    }

    #[test]
    fn test_opcode_ret_stack_underflow() {
        // VM returns from a subroutine
        let mut vm = VirtualMachine::new();
        vm.execute(&Opcode::Ret);
        assert_eq!(vm.registers.read(Register::SP), 0);
        assert_eq!(vm.registers.read(Register::PC), STACK_SIZE);
        assert!(vm.halted);
    }
}

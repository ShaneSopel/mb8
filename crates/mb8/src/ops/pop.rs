use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn pop(&mut self, dst: Register) {
        let stack_pointer = self.registers.read(Register::SP);
        if stack_pointer == 0 {
            self.halted = true;
            return;
        }
        let (value, stack_pointer) = self.mem.pop_u8(stack_pointer);
        self.registers.write(Register::SP, stack_pointer);
        self.registers.write(dst, value as u16);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::{opcodes::Opcode, STACK_SIZE};

    use super::*;

    #[test]
    fn test_opcode_pop() {
        // VM pops a value from the stack
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::SP, 1);
        vm.mem.write_u8(STACK_SIZE - 1, 0x45);
        vm.execute(&Opcode::Pop { dst: Register::R0 });
        assert_eq!(vm.registers.read(Register::SP), 0);
        assert_eq!(vm.registers.read(Register::R0), 0x45);
    }

    #[test]
    fn test_opcode_pop_stack_underflow() {
        // VM halts when the stack underflows
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::SP, 0);
        vm.execute(&Opcode::Pop { dst: Register::R0 });
        assert!(vm.halted);
    }
}

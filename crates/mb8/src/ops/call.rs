use mb8_isa::{registers::Register, STACK_SIZE};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn call(&mut self, addr: u16) {
        let mut stack_pointer = self.registers.read(Register::SP);
        let program_counter = self.registers.read(Register::PC);
        if stack_pointer + 2 >= STACK_SIZE {
            self.halted = true;
            return;
        }

        stack_pointer = self.mem.push_u16(stack_pointer, program_counter);

        self.registers.write(Register::SP, stack_pointer);
        self.registers.write(Register::PC, addr + STACK_SIZE);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_call() {
        // VM calls a subroutine at a given address
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::PC, 0x100);
        vm.execute(&Opcode::Call { addr: 0x100 });
        assert_eq!(vm.registers.read(Register::SP), 2);
        assert_eq!(vm.registers.read(Register::PC), 0x200);
        assert_eq!(vm.mem.read_u16(STACK_SIZE - 2), 0x100);
    }

    #[test]
    fn test_opcode_call_stack_overflow() {
        // VM halts when the stack overflows
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::SP, STACK_SIZE - 2);
        vm.execute(&Opcode::Call { addr: 0x456 });
        assert!(vm.halted);
    }
}

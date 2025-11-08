use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn ld(&mut self, addr: u16) {
        let value = self.mem.read_u8(addr);
        self.registers.write(Register::R7, value as u16);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_ld() {
        // VM loads a value from memory into a register
        let mut vm = VirtualMachine::new();
        vm.mem.write_u8(0x200, 0x12);
        vm.execute(&Opcode::Ld { addr: 0x200 });
        assert_eq!(vm.registers.read(Register::R7), 0x12);
    }
}

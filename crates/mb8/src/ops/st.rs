use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn st(&mut self, addr: u16) {
        let value = self.registers.read(Register::R7);
        self.mem.write_u8(addr, value as u8);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_st() {
        // VM save register value to memory
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::R7, 0x12);
        vm.execute(&Opcode::St { addr: 0x200 });
        assert_eq!(vm.mem.read_u8(0x200), 0x12);
    }
}

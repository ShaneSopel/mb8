use mb8_isa::registers::Register;

use crate::{
    mem::regions::MemoryRegion,
    vm::{Role, VirtualMachine},
};

impl VirtualMachine {
    pub fn stg(&mut self, src: Register, bot: Register) {
        let addr = self.registers.read(Register::I);
        let value = self.registers.read(src);
        let mut ctx = if let Role::Bot(id) = self.role {
            self.mem.bot(id).mailbox()
        } else {
            let bot_id = self.registers.read(bot);
            self.mem.bot(bot_id as u8).mailbox()
        };
        ctx.write(addr, value as u8);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_opcode_stg() {
        // VM loads data from memory
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::I, 0x10);
        vm.registers.write(Register::R0, 0x77);
        vm.registers.write(Register::R1, 1);
        vm.execute(&Opcode::Stg {
            src: Register::R0,
            bot: Register::R1,
        });
        assert_eq!(vm.mem.bot(1).mailbox().read(0x10), 0x77);
    }
}

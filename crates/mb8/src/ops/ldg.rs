use mb8_isa::registers::Register;

use crate::{
    mem::regions::MemoryRegion,
    vm::{Role, VirtualMachine},
};

impl VirtualMachine {
    pub fn ldg(&mut self, dst: Register, bot: Register) {
        let addr = self.registers.read(Register::I);
        let ctx = if let Role::Bot(bot_id) = self.role {
            self.mem.bot(bot_id).mailbox()
        } else {
            let bot_id = self.registers.read(bot);
            self.mem.bot(bot_id as u8).mailbox()
        };
        let value = ctx.read(addr);
        self.registers.write(dst, value as u16);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_opcode_ldg() {
        // VM loads data from shared global memory
        let mut vm = VirtualMachine::default();
        vm.mem.bot(1).mailbox().write(0x10, 0x77);
        vm.registers.write(Register::I, 0x10);
        vm.registers.write(Register::R1, 1);
        vm.role = Role::Judge;
        vm.execute(&Opcode::Ldg {
            dst: Register::R0,
            bot: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 0x77);
    }
}

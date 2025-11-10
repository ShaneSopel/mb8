use mb8_isa::{opcodes::Syscall, registers::Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn sys(&mut self, syscall: Syscall, src: Register) {
        match syscall {
            Syscall::Putc => self.sys_putc(src),
        }
    }

    fn sys_putc(&mut self, src: Register) {
        let value = self.registers.read(src);
        print!("{}", value as u8 as char);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_opcode_sys_putc() {
        // VM prints the value of the source register
    }
}

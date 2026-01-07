//write a debug helper that can be used by both architectures.
use crate::tty::{self, Tty};
use mb8::dev::gpu::registers;
use mb8::vm::VirtualMachine;
use minifb::Key;
use minifb::{Window, WindowOptions};

#[derive(Debug)]
pub enum DebugCmd {
    Step,
    Continue,
    Registers,
    Help,
}

#[derive(Debug)]
pub struct Debug {}

impl Debug {
    pub fn new() -> Debug {
        Self {}
    }

    fn tty_write_str(&mut self, vm: &mut VirtualMachine, s: &str) {
        for b in s.bytes() {
            vm.devices.write(registers::GPU_REG_TTY, b);
        }
    }

    pub fn poll_command(&mut self, window: &minifb::Window) -> Option<DebugCmd> {
        // read key events from window
        let keys = window.get_keys_pressed(minifb::KeyRepeat::No);

        for key in keys {
            return match key {
                Key::N => Some(DebugCmd::Step),
                Key::C => Some(DebugCmd::Continue),
                Key::R => Some(DebugCmd::Registers),
                Key::H => Some(DebugCmd::Help),
                _ => None,
            };
        }

        None
    }

    pub fn render_prompt(&mut self, vm: &mut VirtualMachine) {
        // self.tty_write_str(vm, "\nDBG> ");
        vm.devices.write(registers::GPU_REG_TTY, b'D');
    }

    pub fn debug_shell(&mut self) {}

    pub fn execute_next_instruction() {
        //execute the next instruction
    }

    pub fn continue_normal_execution() {}

    pub fn print_registers(&mut self, vm: &mut VirtualMachine) {
        let regs = format!("{:?}\n", vm.registers);
        self.tty_write_str(vm, &regs);
    }

    pub fn print_memory() {}

    pub fn print_help(&mut self, tty: &mut Tty) {
        let help = "\nDebugger commands:\n\
            n - step instruction\n\
            c - continue execution\n\
            r - print registers\n\
            m - print memory\n\
            h - help\n";

        for b in help.bytes() {
            tty.write_byte(b);
        }
    }
}

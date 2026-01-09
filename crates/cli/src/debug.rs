use std::vec;

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

   pub fn handle_debug_byte(&mut self, byte: u8, tty: &mut Tty, vm: &mut VirtualMachine, debug_input: &mut Vec<u8>) {
        match byte {
            b'\n' => {
                tty.write_byte(b'\n');
                self.execute_debug_command(tty, vm, debug_input);
            }
            0x08 => {
                debug_input.pop();
                tty.write_byte(0x08);
            }
            _ => {
                debug_input.push(byte);
                tty.write_byte(byte);
            }
        }
    }

        fn execute_debug_command(&mut self, tty: &mut Tty, vm: &mut VirtualMachine, debug_input: &mut Vec<u8>) {
    let cmd = core::str::from_utf8(&debug_input)
        .unwrap_or("")
        .trim();

    match cmd {
        "s" => {
            vm.step();
            self.print_help(tty);
        }
        "c" => {
            //self.paused = false;
            //self.debug_prompt = false;
        }
        "r" => {
            self.print_registers(vm);
            self.print_help(tty);
        }
        "h" => {
            self.print_help(tty);
        }
        _ => {
            tty.write_byte(b'?');
            tty.write_byte(b'\n');
            self.print_help(tty);
        }
    }

    debug_input.clear();
}


    pub fn map_debug_key(key: Key) -> Option<u8> {
        Some(match key {
            Key::A => b'a',
            Key::B => b'b',
            Key::C => b'c',
            Key::S => b's',
            Key::R => b'r',
            Key::H => b'h',

            Key::Enter => b'\n',
            Key::Backspace => 0x08,

            _ => return None,
        })
    }
}

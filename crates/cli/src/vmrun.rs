use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{filesystem::makefs, keyboard::Keyboard};
use mb8::vm;
use minifb::{Window, WindowOptions};

use crate::debug::{Debug, DebugCmd};
use crate::tty::Tty;

const OPS_PER_FRAME: u32 = 1024;
const RENDER_INTERVAL: u32 = 1000;

#[derive(Debug)]
pub struct VmRun {
    pub vm: vm::VirtualMachine,
    pub tty: Tty,
    ticks: u32,
    width: usize,
    height: usize,
    debug: Debug,
    pub debug_enabled: bool,
}

impl VmRun {
    #[must_use]
    pub fn new(vm: vm::VirtualMachine, tty: Tty, debug: Debug) -> Self {
        Self {
            vm,
            tty,
            ticks: 0,
            width: 320,
            height: 200,
            debug: debug,
            debug_enabled: false,
        }
    }

    pub fn run_desktop(&mut self, kernel: PathBuf, user: Vec<PathBuf>, seed: Option<u16>) {
        let Ok(rom) = std::fs::read(kernel) else {
            return;
        };
        self.vm.load_rom(&rom);

        let Ok(mut window) = Window::new("MB8", 640, 480, WindowOptions::default()) else {
            return;
        };

        let seed = seed.unwrap_or(1);

        self.vm.devices.rand().number = (seed as u8).max(1);

        makefs(user, &mut self.vm);

        let mut buf = vec![0u32; self.width * self.height];
        self.ticks = RENDER_INTERVAL - 1;
        let mut last_render = Instant::now();
        let l_shift = false;
        let r_shift = false;
        let key = &mut Keyboard::new(l_shift, r_shift);

        while !self.vm.halted && window.is_open() {
            self.ticks = self.ticks.wrapping_add(1);

            if self.run_debug(&window) {
                continue;
            }

            Keyboard::key_pressed(key, &window, &mut self.vm);

            Keyboard::key_released(key, &window);

            self.vm_step();

            if last_render.elapsed() >= Duration::from_millis(16) {
                let gpu = self.vm.devices.gpu();
                for &byte in gpu.tty_buffer() {
                    self.tty.write_byte(byte);
                }

                self.tty.render(buf.as_mut_slice(), 320);

                if window.update_with_buffer(&buf, 320, 200).is_err() {
                    return;
                }
                last_render = Instant::now();
            }
        }
    }

    fn vm_step(&mut self) {

          if self.debug_enabled {
        if !self.vm.halted {
            self.vm.step();
        }
        return;
    }
        for _ in 0..OPS_PER_FRAME {
            if self.vm.halted {
                break;
            }

            self.vm.step();
        }
    }

    fn run_debug(&mut self, window: &minifb::Window) -> bool {
        const USER_ENTRY: u16 = 0x4000;

        if self.debug_enabled && self.vm.program_counter == USER_ENTRY {
            self.vm.debug_break = true;
        }

        if !self.debug_enabled || !self.vm.debug_break {
            return false;
        }
        self.debug.render_prompt(&mut self.vm);

        // Wait for a debugger command
        if let Some(cmd) = self.debug.poll_command(window) {
            match cmd {
                DebugCmd::Step => {
                    // Execute exactly one instruction
                    self.vm.debug_break = false;
                    self.vm.step();
                    self.vm.debug_break = true;
                }

                DebugCmd::Continue => {
                    self.vm.debug_break = false;
                }

                DebugCmd::Registers => {
                    self.debug.print_registers(&mut self.vm);
                }

                DebugCmd::Help => {
                    self.debug.print_help(&mut self.tty);
                }
            }
        }

        true // stop this frame
    }
}

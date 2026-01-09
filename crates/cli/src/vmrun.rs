use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{filesystem::makefs, keyboard::Keyboard};
use mb8::dev::gpu::{self, registers};
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
    pub window: Window,
    ticks: u32,
    width: usize,
    height: usize,
    debug: Debug,
    debug_input: Vec<u8>,
    pub debug_enabled: bool,
    pub hit_entry_break: bool,
    pub paused: bool,
    pub debug_prompt: bool,
}

impl VmRun {
    #[must_use]
    pub fn new(vm: vm::VirtualMachine, tty: Tty, debug: Debug) -> Self {

        let window =  Window::new("MB8", 640, 480, WindowOptions::default()).expect("Failed to create window");
        Self {
            vm,
            tty,
            window,
            ticks: 0,
            width: 320,
            height: 200,
            debug: debug,
            debug_input: Vec::new(),
            debug_enabled: false,
            hit_entry_break: false,
            paused: false,
            debug_prompt: false,
        }
    }

    pub fn run_desktop(&mut self, kernel: PathBuf, user: Vec<PathBuf>, seed: Option<u16>) {
        let Ok(rom) = std::fs::read(kernel) else {
            return;
        };
        self.vm.load_rom(&rom);

        let seed = seed.unwrap_or(1);

        self.vm.devices.rand().number = (seed as u8).max(1);

        makefs(user, &mut self.vm);

        let mut buf = vec![0u32; self.width * self.height];
        self.ticks = RENDER_INTERVAL - 1;
        let mut last_render = Instant::now();
        let l_shift = false;
        let r_shift = false;
        let key = &mut Keyboard::new(l_shift, r_shift);

        //refactored the while loop to be more minimalized....
        /*  while !self.vm.halted && window.is_open() {
         self.ticks = self.ticks.wrapping_add(1);

         //if self.run_debug(&window) {
         //    continue;
        // }


        let paused = self.run_debug(&window);

        if !paused{

          Keyboard::key_pressed(key, &window, &mut self.vm);

         Keyboard::key_released(key, &window);

         self.vm_step();

        }

         if last_render.elapsed() >= Duration::from_millis(16) {
             let gpu = self.vm.devices.gpu();
             for &byte in gpu.tty_buffer() {
                 self.tty.write_byte(byte);
             }

             self.tty.render(buf.as_mut_slice(), 320);

            // if window.update_with_buffer(&buf, 320, 200).is_err() {
                // self.has_rendered = true;
            //     return;
            // }

             window.update_with_buffer(&buf, 320, 200).unwrap();
             last_render = Instant::now();


         } */

       /* while !self.vm.halted && self.window.is_open() {
            self.ticks = self.ticks.wrapping_add(1);

            self.render();

            // need to setup keyboard response in debug mode. 
            Keyboard::key_pressed(key, &self.window, &mut self.vm);
            Keyboard::key_released(key, &self.window);
            if !self.paused {
                let paused = self.run_debug();
                if !paused {
                    self.vm_step();
                }
            }
        }
*/

        while !self.vm.halted && self.window.is_open() {
    self.render();

    self.run_debug();

    if self.paused {
        self.poll_debug_keys();
    } else {
        Keyboard::key_pressed(key, &self.window, &mut self.vm);
        Keyboard::key_released(key, &self.window);
        self.vm_step();
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
            println!("PC = {:04X}", self.vm.program_counter);
        }
    }

    fn run_debug(&mut self) -> bool {
        const USER_ENTRY: u16 = 0xE100;

        println!("DEBUG: debug_prompt = {}", self.debug_prompt);

        if self.debug_enabled && !self.hit_entry_break && self.vm.program_counter == USER_ENTRY {
            self.hit_entry_break = true;
            //
            self.paused = true;
        }

        if !self.paused {
            return false;
        }

        println!(
            "DEBUG CHECK: enabled={}, paused={}, pc={:04X}",
            self.debug_enabled, self.paused, self.vm.program_counter
        );

        if self.paused && !self.debug_prompt {
            println!("Writing to GPU TTY");
            //self.print_debug_prompt();

            self.tty.clear();
            self.debug.print_help(&mut self.tty);
        
            self.debug_prompt = true;

            println!("DEBUG: paused = {}", self.paused);
            println!("DEBUG: debug prompt = {}", self.debug_prompt);

        }
        println!("DEBUG: render = {}", self.paused);
        //self.render();

        // Wait for a debugger command
       /*  if let Some(cmd) = self.debug.poll_command(window) {
            match cmd {
                DebugCmd::Step => {
                    // Execute exactly one instruction
                    self.vm.step();
                }

                DebugCmd::Continue => {
                    self.paused = false;
                }

                DebugCmd::Registers => {
                    self.debug.print_registers(&mut self.vm);
                }

                DebugCmd::Help => {
                    self.debug.print_help(&mut self.tty);
                }
            }
        }*/

        true 
    }

    fn render(&mut self) {
        let mut buf = vec![0u32; self.width * self.height];
        let gpu = self.vm.devices.gpu();

        if !self.paused
        {
               for (i, &byte) in gpu.tty_buffer().iter().enumerate() {
        self.tty.write_byte(byte);
    }
        }
       
        println!("We are Rendering");
        self.tty.render(&mut buf, self.width);

        if self.window.update_with_buffer(&buf, 320, 200).is_err() {
            println!("Error updating window buffer");
        }
    }

    fn poll_debug_keys(&mut self) {
        for key in self.window.get_keys_pressed(minifb::KeyRepeat::No) {
            if let Some(byte) = Debug::map_debug_key(key) {
                self.debug.handle_debug_byte(byte, &mut self.tty, &mut self.vm, &mut self.debug_input);
            }
        }
    }
}

use std::path::PathBuf;

use clap::Parser;
use mb8::vm;
use minifb::{Window, WindowOptions};

mod config;

fn run_vm(file: PathBuf) {
    let Ok(source) = std::fs::read(file) else {
        return;
    };

    let mut buf = vec![0u32; 64 * 32];

    let Ok(mut window) = Window::new("MB8", 640, 320, WindowOptions::default()) else {
        return;
    };

    let mut vm = vm::VirtualMachine::default();
    vm.load_rom(&source);

    while !vm.halted && window.is_open() {
        // terminal
        vm.step();

        let gfx = vm.mem.graphic_buffer();

        for y in 0..32 {
            for x in 0..64 {
                let index = y as usize * 64 + x as usize;
                if gfx.get_pixel(x, y) {
                    buf[index] = 0xFFFF_FFFF;
                } else {
                    buf[index] = 0x0000_0000;
                }
            }
        }

        if window.update_with_buffer(&buf, 64, 32).is_err() {
            return;
        }
    }
}

fn main() {
    let cli = config::Cli::parse();

    match cli.command {
        config::Commands::Run { file } => {
            run_vm(file);
        }
    }
}

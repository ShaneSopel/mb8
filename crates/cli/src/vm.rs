use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use mb8::{
    dev::gpu::registers::{TTY_COLS, TTY_ROWS},
    vm,
};

use minifb::{Window, WindowOptions};

use crate::tty::Tty;
use crate::keyboard;

const OPS_PER_FRAME: u32 = 1024;
const RENDER_INTERVAL: u32 = 1000;
#[allow(clippy::too_many_lines)]
pub fn run_vm(kernel: PathBuf, user: Vec<PathBuf>, seed: Option<u16>) {
    let Ok(rom) = std::fs::read(kernel) else {
        return;
    };
    let mut vm = vm::VirtualMachine::default();
    vm.load_rom(&rom);

    let Ok(mut window) = Window::new("MB8", 640, 480, WindowOptions::default()) else {
        return;
    };

    let seed = seed.unwrap_or(1);

    let mut tty = Tty::new(TTY_COLS as usize, TTY_ROWS as usize, 1024);
    vm.devices.rand().number = (seed as u8).max(1);

    // MakeFS
    let mut fs = vec![0u8; 65536];
    let mut blocks = 1;
    let mut files = 0;
    for path in user {
        let Ok(data) = std::fs::read(&path) else {
            continue;
        };
        let Ok(name) = path.file_stem().ok_or("Failed to get file name") else {
            continue;
        };

        let size = (data.len() / 256) + 1;

        // Add to zero block
        let zero_block_start = files * 16;
        fs[zero_block_start] = 1;
        fs[zero_block_start + 1] = blocks;
        fs[zero_block_start + 2] = size as u8;

        let chars = name.as_encoded_bytes();
        if chars.len() > 8 {
            eprintln!(
                "Error: File name {} is too long. Max 8 characters allowed.",
                name.to_string_lossy()
            );
            return;
        }
        for (i, c) in chars.iter().enumerate() {
            fs[zero_block_start + 3 + i] = *c;
        }

        let block_start = blocks as usize * 256;
        for (i, d) in data.iter().enumerate() {
            fs[block_start + i] = *d;
        }

        blocks += size as u8;
        files += 1;
    }

    let Ok(fs) = fs.try_into() else {
        eprintln!("Failed to convert file system");
        return;
    };
    vm.devices.disk().set(fs);

    let mut buf = vec![0u32; 320 * 200];
    let mut ticks = RENDER_INTERVAL - 1;
    let mut last_render = Instant::now();

    while !vm.halted && window.is_open() {
        ticks = ticks.wrapping_add(1);

        for _ in 0..OPS_PER_FRAME {
            if vm.halted {
                break;
            }
            vm.step();
        }

        if last_render.elapsed() >= Duration::from_millis(16) {
            let gpu = vm.devices.gpu();
            for &byte in gpu.tty_buffer() {
                tty.write_byte(byte);
            }

            tty.render(buf.as_mut_slice(), 320);

            if window.update_with_buffer(&buf, 320, 200).is_err() {
                return;
            }
            last_render = Instant::now();
        }
    }
}
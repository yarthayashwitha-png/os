//! Interactive Bare-Metal Kernel Shell

use crate::println;
use crate::print;
use crate::vga::{WRITER, Color};
use crate::keyboard::pop_key;
use crate::pit::{get_ticks, get_uptime_seconds};
use crate::port::outb;
use crate::pagerep;
use crate::snake;

const MAX_CMD_LEN: usize = 64;

pub struct Shell {
    buffer: [char; MAX_CMD_LEN],
    len: usize,
}

impl Shell {
    pub const fn new() -> Self {
        Shell {
            buffer: ['\0'; MAX_CMD_LEN],
            len: 0,
        }
    }

    pub fn print_prompt(&self) {
        let mut w = WRITER.lock();
        w.set_color(Color::LightGreen, Color::Black);
        w.write_string("aether-os> ");
        w.set_color(Color::White, Color::Black);
    }

    pub fn update(&mut self) {
        while let Some(ch) = pop_key() {
            match ch {
                '\n' => {
                    println!();
                    self.execute();
                    self.len = 0;
                    self.print_prompt();
                }
                '\x08' => { // Backspace
                    if self.len > 0 {
                        self.len -= 1;
                        WRITER.lock().backspace();
                    }
                }
                c if self.len < MAX_CMD_LEN - 1 => {
                    self.buffer[self.len] = c;
                    self.len += 1;
                    print!("{}", c);
                }
                _ => {}
            }
        }
    }

    fn execute(&self) {
        if self.len == 0 {
            return;
        }

        let cmd_str = self.as_str();

        if cmd_str == "help" {
            println!("Available AetherOS Shell Commands:");
            println!("  help       - Display this command manual");
            println!("  version    - Show kernel version, architecture & author");
            println!("  clear      - Clear the VGA terminal buffer");
            println!("  uptime     - Display system uptime serviced by PIT timer");
            println!("  ticks      - Print total hardware timer interrupt count");
            println!("  meminfo    - Show physical memory & page frame status");
            println!("  colors     - Display 16-color VGA palette test");
            println!("  pagerep    - Run ACM SAC'17 Page Replacement Algorithm demo");
            println!("  snake      - Play in-kernel bare-metal Retro Arcade Snake game");
            println!("  reboot     - Soft reboot computer via 8042 controller");
            println!("  halt       - Halt CPU in low-power idle state");
        } else if cmd_str == "version" {
            WRITER.lock().set_color(Color::LightCyan, Color::Black);
            println!("===============================================================================");
            println!("  AetherOS Bare-Metal Kernel v0.2.0 (Dual-OS Engineering Project)");
            println!("  Architecture: 32-bit x86 Protected Mode | Freestanding Rust (#![no_std])");
            println!("  Subsystems  : Custom GDT, IDT Interrupts, 8259 PIC, 100Hz PIT, VGA Text Driver");
            println!("  Research Lab: Frequency-Based Page Replacement (Tsai & Lei, ACM SAC'17)");
            println!("===============================================================================");
            WRITER.lock().set_color(Color::White, Color::Black);
        } else if cmd_str == "clear" {
            WRITER.lock().clear_screen();
        } else if cmd_str == "uptime" {
            let secs = get_uptime_seconds();
            let mins = secs / 60;
            let rem_secs = secs % 60;
            println!("Kernel Uptime: {}m {}s ({} total seconds)", mins, rem_secs, secs);
        } else if cmd_str == "ticks" {
            println!("PIT Timer Interrupt Ticks: {}", get_ticks());
        } else if cmd_str == "meminfo" {
            println!("--- Physical & Virtual Memory Subsystem ---");
            println!("  Detected Base RAM   : 640 KB Conventional (0x00000000 - 0x0009FFFF)");
            println!("  VGA Video Buffer    : 32 KB at 0x000B8000");
            println!("  Kernel Base Memory  : 1 MB at 0x00100000");
            println!("  Total Physical RAM  : 128 MB (Simulated in VM/QEMU)");
            println!("  Page Size           : 4096 Bytes (4 KB)");
            println!("  Paging Engine       : Two-tier Page Directory + Page Table (CR3 active)");
            println!("  Algorithm in Lab    : Tsai-Lei Frequency Log Sliding Window");
        } else if cmd_str == "colors" {
            println!("VGA 16-Color Palette Verification:");
            for c in 0..16 {
                let mut w = WRITER.lock();
                w.color_code = crate::vga::ColorCode::new(
                    unsafe { core::mem::transmute(c as u8) },
                    Color::Black,
                );
                w.write_string(" [■ COLOR ");
                let tens = (c / 10) as u8 + b'0';
                let ones = (c % 10) as u8 + b'0';
                if c >= 10 { w.write_byte(tens); }
                w.write_byte(ones);
                w.write_string("] ");
                if c == 7 || c == 15 {
                    w.write_byte(b'\n');
                }
            }
            WRITER.lock().set_color(Color::White, Color::Black);
        } else if cmd_str == "pagerep" {
            pagerep::run_kernel_demo();
        } else if cmd_str == "snake" {
            snake::play();
            println!("\nReturned from Snake game to AetherOS shell.");
        } else if cmd_str == "reboot" {
            println!("Issuing soft reboot command to 8042 keyboard controller...");
            unsafe {
                // Pulse reset line on 8042 controller
                outb(0x64, 0xFE);
            }
        } else if cmd_str == "halt" {
            println!("CPU Halted. Power down the system or press hardware reset button.");
            loop {
                unsafe { core::arch::asm!("hlt"); }
            }
        } else if cmd_str.starts_with("echo ") {
            println!("{}", &cmd_str[5..]);
        } else {
            WRITER.lock().set_color(Color::LightRed, Color::Black);
            print!("Unknown command: '{}'. ", cmd_str);
            WRITER.lock().set_color(Color::White, Color::Black);
            println!("Type 'help' for a list of valid commands.");
        }
    }

    fn as_str(&self) -> &str {
        // Convert array of chars to UTF-8 byte slice safely in no_std
        static mut STR_BUF: [u8; MAX_CMD_LEN] = [0; MAX_CMD_LEN];
        let mut idx = 0;
        for i in 0..self.len {
            let ch = self.buffer[i];
            if ch.is_ascii() {
                unsafe {
                    STR_BUF[idx] = ch as u8;
                }
                idx += 1;
            }
        }
        unsafe {
            core::str::from_utf8_unchecked(&STR_BUF[..idx])
        }
    }
}

//! ==============================================================================
//! AETHER-OS: Bare-Metal 32-bit x86 Operating System Kernel
//! Language: Freestanding Rust (#![no_std], #![no_main])
//! Subsystems: Custom GDT, IDT, Dual 8259 PIC, PIT 100Hz, VGA Color Buffer, PS/2
//! Interactive Kernel Shell with in-kernel Snake game and Academic Research Demo
//! ==============================================================================

#![no_std]
#![no_main]
#![feature(naked_functions)]

pub mod port;
pub mod vga;
pub mod gdt;
pub mod pic;
pub mod pit;
pub mod idt;
pub mod keyboard;
pub mod shell;
pub mod pagerep;
pub mod snake;

use core::panic::PanicInfo;
use vga::{WRITER, Color};
use shell::Shell;

// Multiboot 1 Specification Header for GRUB / QEMU / VirtualBox compatibility
#[repr(C, align(4))]
pub struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

const MULTIBOOT_MAGIC: u32 = 0x1BADB002;
const MULTIBOOT_FLAGS: u32 = 0x00000003; // Align modules + memory info
const MULTIBOOT_CHECKSUM: u32 = (0u32.wrapping_sub(MULTIBOOT_MAGIC.wrapping_add(MULTIBOOT_FLAGS)));

#[used]
#[no_mangle]
#[link_section = ".multiboot"]
pub static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: MULTIBOOT_MAGIC,
    flags: MULTIBOOT_FLAGS,
    checksum: MULTIBOOT_CHECKSUM,
};

/// Freestanding Kernel Entry Point invoked by bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Initialize VGA Color Text Driver
    WRITER.lock().clear_screen();
    WRITER.lock().set_color(Color::LightCyan, Color::Black);

    println!("===============================================================================");
    println!("     _         _   _                   ___  ____                               ");
    println!("    / \\   ___ | |_| |__   ___ _ __    / _ \\/ ___|                              ");
    println!("   / _ \\ / _ \\| __| '_ \\ / _ \\ '__|  | | | \\___ \\                              ");
    println!("  / ___ \\  __/| |_| | | |  __/ |     | |_| |___) |                             ");
    println!(" /_/   \\_\\___| \\__|_| |_|\\___|_|      \\___/|____/  v0.2.0                       ");
    println!("===============================================================================");

    WRITER.lock().set_color(Color::White, Color::Black);
    println!("[*] Booting 32-bit x86 Bare-Metal Kernel in Freestanding Rust...");

    // 2. Initialize GDT (Global Descriptor Table)
    print!("[*] Loading Global Descriptor Table (GDT)... ");
    gdt::init();
    WRITER.lock().set_color(Color::LightGreen, Color::Black);
    println!("[ OK ]");
    WRITER.lock().set_color(Color::White, Color::Black);

    // 3. Remap 8259 PIC (Programmable Interrupt Controller)
    print!("[*] Remapping Dual 8259 PIC (IRQs -> 0x20..0x2F)... ");
    pic::init();
    WRITER.lock().set_color(Color::LightGreen, Color::Black);
    println!("[ OK ]");
    WRITER.lock().set_color(Color::White, Color::Black);

    // 4. Initialize PIT (Programmable Interval Timer) to 100 Hz
    print!("[*] Initializing 8254 PIT (100 Hz / 10ms resolution)... ");
    pit::init();
    WRITER.lock().set_color(Color::LightGreen, Color::Black);
    println!("[ OK ]");
    WRITER.lock().set_color(Color::White, Color::Black);

    // 5. Initialize IDT (Interrupt Descriptor Table) and enable interrupts
    print!("[*] Loading IDT Gates & Enabling CPU Interrupts (STI)... ");
    idt::init();
    WRITER.lock().set_color(Color::LightGreen, Color::Black);
    println!("[ OK ]");
    WRITER.lock().set_color(Color::White, Color::Black);

    println!("[+] Hardware initialization complete. Welcome to AetherOS!");
    println!("[*] Type 'help' to inspect commands, 'pagerep' for Research Lab, or 'snake' to play!\n");

    // 6. Launch Interactive Kernel Shell
    let mut kernel_shell = Shell::new();
    kernel_shell.print_prompt();

    // 7. Main CPU Event Loop
    loop {
        kernel_shell.update();
        // Low-power sleep until next hardware interrupt wakes the CPU
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic Handler for freestanding environment
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = WRITER.lock();
    writer.set_color(Color::White, Color::Red);
    writer.write_string("\n\n!!! KERNEL PANIC !!!\n");
    if let Some(location) = info.location() {
        writer.write_string("Location: ");
        writer.write_string(location.file());
        writer.write_string(":");
        let line = location.line();
        let tens = ((line / 10) % 10) as u8 + b'0';
        let ones = (line % 10) as u8 + b'0';
        writer.write_byte(tens);
        writer.write_byte(ones);
        writer.write_string("\n");
    }
    writer.write_string("System execution halted to preserve data integrity.\n");
    drop(writer);

    loop {
        unsafe {
            core::arch::asm!("cli; hlt");
        }
    }
}

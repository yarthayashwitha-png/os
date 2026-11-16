//! 8254 Programmable Interval Timer (PIT) Driver

use core::sync::atomic::{AtomicU64, Ordering};
use crate::port::outb;

const PIT_CHANNEL_0: u16 = 0x40;
const PIT_COMMAND: u16   = 0x43;
const PIT_BASE_FREQ: u32 = 1193182;
pub const TARGET_HZ: u32 = 100; // 100 Hz = 10 ms per tick

static TICKS: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    let divisor = (PIT_BASE_FREQ / TARGET_HZ) as u16;
    unsafe {
        // Channel 0, Access Mode lobyte/hibyte, Mode 3 (Square Wave Generator), Binary mode
        // Command byte: 00110110b = 0x36
        outb(PIT_COMMAND, 0x36);
        outb(PIT_CHANNEL_0, (divisor & 0xFF) as u8);
        outb(PIT_CHANNEL_0, ((divisor >> 8) & 0xFF) as u8);
    }
}

pub fn handle_interrupt() {
    TICKS.fetch_add(1, Ordering::Relaxed);
}

pub fn get_ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

pub fn get_uptime_seconds() -> u64 {
    get_ticks() / (TARGET_HZ as u64)
}

pub fn sleep_ticks(count: u64) {
    let start = get_ticks();
    while get_ticks() < start + count {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

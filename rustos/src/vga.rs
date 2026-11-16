//! VGA 80x25 Color Text Driver with Hardware Cursor Support

use core::fmt;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::port::{outb, inb};

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
const VGA_MEMORY: *mut u16 = 0xb8000 as *mut u16;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

pub struct VgaWriter {
    pub column: usize,
    pub row: usize,
    pub color_code: ColorCode,
}

impl VgaWriter {
    pub const fn new() -> Self {
        VgaWriter {
            column: 0,
            row: 0,
            color_code: ColorCode::new(Color::LightCyan, Color::Black),
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            0x08 => self.backspace(), // Backspace
            byte => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }

                let offset = self.row * BUFFER_WIDTH + self.column;
                let entry = (self.color_code.0 as u16) << 8 | (byte as u16);
                unsafe {
                    core::ptr::write_volatile(VGA_MEMORY.add(offset), entry);
                }
                self.column += 1;
            }
        }
        self.update_hardware_cursor();
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' | 0x08 => self.write_byte(byte),
                _ => self.write_byte(0xfe), // Unknown character placeholder
            }
        }
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color_code = ColorCode::new(fg, bg);
    }

    pub fn clear_screen(&mut self) {
        let blank = (ColorCode::new(Color::White, Color::Black).0 as u16) << 8 | (b' ' as u16);
        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            unsafe {
                core::ptr::write_volatile(VGA_MEMORY.add(i), blank);
            }
        }
        self.column = 0;
        self.row = 0;
        self.update_hardware_cursor();
    }

    pub fn backspace(&mut self) {
        if self.column > 0 {
            self.column -= 1;
            let offset = self.row * BUFFER_WIDTH + self.column;
            let entry = (self.color_code.0 as u16) << 8 | (b' ' as u16);
            unsafe {
                core::ptr::write_volatile(VGA_MEMORY.add(offset), entry);
            }
        }
    }

    fn new_line(&mut self) {
        if self.row < BUFFER_HEIGHT - 1 {
            self.row += 1;
        } else {
            // Scroll buffer up by one row
            for y in 1..BUFFER_HEIGHT {
                for x in 0..BUFFER_WIDTH {
                    unsafe {
                        let from = VGA_MEMORY.add(y * BUFFER_WIDTH + x);
                        let to = VGA_MEMORY.add((y - 1) * BUFFER_WIDTH + x);
                        core::ptr::write_volatile(to, core::ptr::read_volatile(from));
                    }
                }
            }
            // Clear last row
            let blank = (self.color_code.0 as u16) << 8 | (b' ' as u16);
            for x in 0..BUFFER_WIDTH {
                unsafe {
                    core::ptr::write_volatile(VGA_MEMORY.add((BUFFER_HEIGHT - 1) * BUFFER_WIDTH + x), blank);
                }
            }
        }
        self.column = 0;
    }

    pub fn update_hardware_cursor(&self) {
        let pos = (self.row * BUFFER_WIDTH + self.column) as u16;
        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
        }
    }

    pub fn write_char_at(&mut self, x: usize, y: usize, c: char, fg: Color, bg: Color) {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            let offset = y * BUFFER_WIDTH + x;
            let code = ColorCode::new(fg, bg);
            let entry = (code.0 as u16) << 8 | (c as u8 as u16);
            unsafe {
                core::ptr::write_volatile(VGA_MEMORY.add(offset), entry);
            }
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Simple Spinlock for thread-safe kernel VGA output
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: core::cell::UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
            data: core::cell::UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> core::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

pub static WRITER: SpinLock<VgaWriter> = SpinLock::new(VgaWriter::new());

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

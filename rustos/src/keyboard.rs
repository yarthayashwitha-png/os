//! PS/2 Keyboard Driver (Scan Code Set 1)

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::port::inb;

const KEY_BUFFER_SIZE: usize = 128;
static mut KEY_BUFFER: [char; KEY_BUFFER_SIZE] = ['\0'; KEY_BUFFER_SIZE];
static BUFFER_HEAD: AtomicUsize = AtomicUsize::new(0);
static BUFFER_TAIL: AtomicUsize = AtomicUsize::new(0);
static SHIFT_ACTIVE: AtomicBool = AtomicBool::new(false);

const SCANCODE_LSHIFT: u8 = 0x2A;
const SCANCODE_RSHIFT: u8 = 0x36;
const SCANCODE_LSHIFT_REL: u8 = 0xAA;
const SCANCODE_RSHIFT_REL: u8 = 0xB6;

const SCAN_TO_ASCII: [char; 128] = [
    '\0', '\x1B', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\x08',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    '\0', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    '\0', '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', '\0',
    '*', '\0', ' ', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '\0', '\0', '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
];

const SCAN_TO_ASCII_SHIFT: [char; 128] = [
    '\0', '\x1B', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\x08',
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
    '\0', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~',
    '\0', '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', '\0',
    '*', '\0', ' ', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '\0', '\0', '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
];

pub fn handle_interrupt() {
    let scancode = unsafe { inb(0x60) };

    match scancode {
        SCANCODE_LSHIFT | SCANCODE_RSHIFT => {
            SHIFT_ACTIVE.store(true, Ordering::SeqCst);
        }
        SCANCODE_LSHIFT_REL | SCANCODE_RSHIFT_REL => {
            SHIFT_ACTIVE.store(false, Ordering::SeqCst);
        }
        code if code < 128 => {
            let is_shift = SHIFT_ACTIVE.load(Ordering::SeqCst);
            let ch = if is_shift {
                SCAN_TO_ASCII_SHIFT[code as usize]
            } else {
                SCAN_TO_ASCII[code as usize]
            };

            if ch != '\0' {
                push_key(ch);
            }
        }
        _ => {}
    }
}

fn push_key(ch: char) {
    let head = BUFFER_HEAD.load(Ordering::Relaxed);
    let next_head = (head + 1) % KEY_BUFFER_SIZE;
    let tail = BUFFER_TAIL.load(Ordering::Relaxed);

    if next_head != tail {
        unsafe {
            KEY_BUFFER[head] = ch;
        }
        BUFFER_HEAD.store(next_head, Ordering::Release);
    }
}

pub fn pop_key() -> Option<char> {
    let tail = BUFFER_TAIL.load(Ordering::Relaxed);
    let head = BUFFER_HEAD.load(Ordering::Acquire);

    if head == tail {
        None
    } else {
        let ch = unsafe { KEY_BUFFER[tail] };
        BUFFER_TAIL.store((tail + 1) % KEY_BUFFER_SIZE, Ordering::Release);
        Some(ch)
    }
}

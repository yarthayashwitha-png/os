//! 32-bit Interrupt Descriptor Table (IDT) and Exception Handling

use core::arch::asm;
use crate::pic::{send_eoi, PIC1_OFFSET};
use crate::pit;
use crate::keyboard;
use crate::println;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset_high: u16,
}

impl IdtEntry {
    pub const fn empty() -> Self {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            zero: 0,
            type_attr: 0,
            offset_high: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u32, selector: u16, flags: u8) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.selector = selector;
        self.zero = 0;
        self.type_attr = flags;
        self.offset_high = ((handler >> 16) & 0xFFFF) as u16;
    }
}

#[repr(C, packed)]
pub struct IdtPointer {
    limit: u16,
    base: u32,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::empty(); 256];

pub fn init() {
    unsafe {
        // Exception 0: Divide by zero
        IDT[0].set_handler(isr_divide_by_zero as u32, 0x08, 0x8E);
        // Exception 8: Double fault
        IDT[8].set_handler(isr_double_fault as u32, 0x08, 0x8E);
        // Exception 13: General Protection Fault
        IDT[13].set_handler(isr_general_protection as u32, 0x08, 0x8E);
        // Exception 14: Page Fault
        IDT[14].set_handler(isr_page_fault as u32, 0x08, 0x8E);

        // IRQ 0 (Vector 0x20): PIT Timer
        IDT[PIC1_OFFSET as usize].set_handler(irq0_timer_wrapper as u32, 0x08, 0x8E);
        // IRQ 1 (Vector 0x21): PS/2 Keyboard
        IDT[(PIC1_OFFSET + 1) as usize].set_handler(irq1_keyboard_wrapper as u32, 0x08, 0x8E);

        let idt_ptr = IdtPointer {
            limit: (core::mem::size_of_val(&IDT) - 1) as u16,
            base: IDT.as_ptr() as u32,
        };

        // Load IDT Pointer into IDTR
        asm!("lidt [{}]", in(reg) &idt_ptr, options(readonly, nostack, preserves_flags));
        // Enable CPU Interrupts (STI)
        asm!("sti");
    }
}

// Exception handlers
extern "C" fn isr_divide_by_zero() {
    println!("\n[EXCEPTION] CPU Divide by Zero (INT 0)!");
    loop { unsafe { asm!("hlt"); } }
}

extern "C" fn isr_double_fault() {
    println!("\n[CRITICAL] CPU Double Fault (INT 8)!");
    loop { unsafe { asm!("hlt"); } }
}

extern "C" fn isr_general_protection() {
    println!("\n[EXCEPTION] General Protection Fault (INT 13)!");
    loop { unsafe { asm!("hlt"); } }
}

extern "C" fn isr_page_fault() {
    println!("\n[EXCEPTION] Virtual Memory Page Fault (INT 14)!");
    loop { unsafe { asm!("hlt"); } }
}

#[naked]
unsafe extern "C" fn irq0_timer_wrapper() {
    asm!(
        "pushal",
        "call {handler}",
        "push 0", // IRQ 0
        "call {eoi}",
        "add esp, 4",
        "popal",
        "iretd",
        handler = sym pit::handle_interrupt,
        eoi = sym send_eoi,
        options(noreturn)
    );
}

#[naked]
unsafe extern "C" fn irq1_keyboard_wrapper() {
    asm!(
        "pushal",
        "call {handler}",
        "push 1", // IRQ 1
        "call {eoi}",
        "add esp, 4",
        "popal",
        "iretd",
        handler = sym keyboard::handle_interrupt,
        eoi = sym send_eoi,
        options(noreturn)
    );
}

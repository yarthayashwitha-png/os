//! 8259 Programmable Interrupt Controller (PIC) Driver

use crate::port::{outb, inb, io_wait};

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16    = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16    = 0xA1;

const ICW1_INIT: u8     = 0x11;
const ICW4_8086: u8     = 0x01;
const PIC_EOI: u8       = 0x20;

pub const PIC1_OFFSET: u8 = 0x20; // IRQ 0..7 mapped to 0x20..0x27
pub const PIC2_OFFSET: u8 = 0x28; // IRQ 8..15 mapped to 0x28..0x2F

pub fn init() {
    unsafe {
        // Save interrupt masks
        let mask1 = inb(PIC1_DATA);
        let mask2 = inb(PIC2_DATA);

        // ICW1: Start initialization sequence in cascade mode
        outb(PIC1_COMMAND, ICW1_INIT);
        io_wait();
        outb(PIC2_COMMAND, ICW1_INIT);
        io_wait();

        // ICW2: Vector offsets
        outb(PIC1_DATA, PIC1_OFFSET);
        io_wait();
        outb(PIC2_DATA, PIC2_OFFSET);
        io_wait();

        // ICW3: Tell Master that Slave is connected to IRQ2 (0000 0100b = 4)
        outb(PIC1_DATA, 4);
        io_wait();
        // Tell Slave its cascade identity (2)
        outb(PIC2_DATA, 2);
        io_wait();

        // ICW4: Set 8086/88 mode
        outb(PIC1_DATA, ICW4_8086);
        io_wait();
        outb(PIC2_DATA, ICW4_8086);
        io_wait();

        // Enable IRQ0 (Timer) and IRQ1 (Keyboard), mask others for now
        outb(PIC1_DATA, 0xFC); // 1111 1100b: unmask IRQ0 and IRQ1
        outb(PIC2_DATA, 0xFF); // mask all slave IRQs
    }
}

pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, PIC_EOI);
        }
        outb(PIC1_COMMAND, PIC_EOI);
    }
}

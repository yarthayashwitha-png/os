//! 32-bit Global Descriptor Table (GDT) Management

use core::arch::asm;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    pub const fn new(base: u32, limit: u32, access: u8, gran: u8) -> Self {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (gran & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

#[repr(C, packed)]
pub struct GdtPointer {
    limit: u16,
    base: u32,
}

static mut GDT: [GdtEntry; 3] = [
    // 0x00: Null Descriptor
    GdtEntry::new(0, 0, 0, 0),
    // 0x08: Kernel 32-bit Code Segment (Ring 0, Exec/Read, 4GB limit)
    GdtEntry::new(0, 0xFFFFF, 0x9A, 0xCF),
    // 0x10: Kernel 32-bit Data Segment (Ring 0, Read/Write, 4GB limit)
    GdtEntry::new(0, 0xFFFFF, 0x92, 0xCF),
];

pub fn init() {
    unsafe {
        let gdt_ptr = GdtPointer {
            limit: (core::mem::size_of_val(&GDT) - 1) as u16,
            base: GDT.as_ptr() as u32,
        };

        // Load GDT Pointer into GDTR
        asm!("lgdt [{}]", in(reg) &gdt_ptr, options(readonly, nostack, preserves_flags));

        // Reload data segment registers (0x10) and perform far jump to reload CS (0x08)
        asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            "ljmp $0x08, $2f",
            "2:",
            options(nostack)
        );
    }
}

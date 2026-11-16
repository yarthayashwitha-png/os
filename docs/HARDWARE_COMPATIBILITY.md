# Hardware & Virtualization Compatibility Matrix

## 1. Supported Execution Targets

| Platform | Debian MicroOS | RustOS Bare-Metal | Notes |
| :--- | :---: | :---: | :--- |
| **Oracle VirtualBox 7.x** | ✅ Full Support | ✅ Full Support | Launch via `Start-MicroOS.bat` / `Start-RustOS.bat` |
| **Oracle VirtualBox 6.x** | ✅ Full Support | ✅ Full Support | Compatible with standard SATA/IDE controllers |
| **QEMU (x86_64 / i386)** | ✅ Full Support | ✅ Full Support | Launch via `Run-QEMU.bat` (Pre-installed on host) |
| **VMware Workstation / Player** | ✅ Full Support | ✅ Full Support | Supports EFI and BIOS boot modes |
| **Physical PC (Bare Metal)** | ✅ Full Support | ✅ Full Support | Flash ISO to USB drive with Rufus or Ventoy |

---

## 2. Low-Level Hardware Specifications

### CPU Architecture
- **RustOS**: 32-bit x86 Protected Mode (`i686` or compatible x86_64 processor).
- **Debian MicroOS**: 64-bit x86_64 (`amd64`) architecture with CMPXCHG16B, SSE3 support.

### Memory (RAM) Requirements
- **RustOS**: Minimum 32 MB RAM (configured for 128 MB in VM scripts).
- **Debian MicroOS**: Minimum 1024 MB RAM (recommended 2048 MB for optimal live-boot SquashFS caching).

### Video / Display Driver
- **VGA Text Mode**: Standard IBM PC 80x25 text mode (Color palette 0–15, memory mapped at physical address `0x000B8000`).
- **Graphical Mode (Debian MicroOS)**: Xorg with `modesetting` / `vboxvideo` / `modesetting` fallbacks; supports VESA VBE 2.0+ up to 1920x1080 resolution.

### Interrupt & Timer Subsystem
- **Dual 8259A PIC**: Master at I/O ports `0x20`/`0x21`, Slave at `0xA0`/`0xA1`.
- **8254 PIT**: Channel 0 mapped to IRQ 0, programmed for 100 Hz square wave mode.
- **PS/2 Keyboard Controller**: 8042 controller at I/O port `0x60` (data) and `0x64` (status/command).

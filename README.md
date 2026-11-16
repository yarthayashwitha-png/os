# 👑 Dual-OS Engineering Architecture with Modular Research Laboratory

[![Rust](https://img.shields.io/badge/Rust-1.99+-orange.svg)](https://www.rust-lang.org)
[![Debian](https://img.shields.io/badge/Debian-12%20Bookworm-red.svg)](https://www.debian.org)
[![VirtualBox](https://img.shields.io/badge/Oracle%20VirtualBox-7.x%20Ready-blue.svg)](https://www.virtualbox.org)
[![Research Paper](https://img.shields.io/badge/ACM%20SAC'17-Page%20Replacement-teal.svg)](https://doi.org/10.1145/3019612.3019737)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

An enterprise-grade, dual-operating-system engineering project combining:
1. **Debian MicroOS (Aether Edition)**: An ultra-fast, lightweight (<450 MB) bootable Live Linux distribution featuring a Windows 11-style graphical desktop (XFCE4/LightDM), automated autologin, integrated retro arcade Snake game, and the **Academic Research Paper Experimental Lab**.
2. **RustOS**: A 32-bit bare-metal operating system kernel built from scratch in freestanding Rust (`#![no_std]`, `#![no_main]`), featuring custom GDT, IDT interrupt handling, 8259 PIC remapping, PIT timer scheduling, VGA color text driver, in-kernel Snake game, and an interactive kernel shell.
3. **Automated Windows 1-Click Launchers**: Complete batch & PowerShell scripts that automatically download Oracle VirtualBox 7.x, provision VMs, or fall back to native QEMU with zero manual setup.
4. **Academic Viva & Research Suite**: Comprehensive analysis of *"A Page Replacement Algorithm Based on Frequency Derived From Reference History"* (Tsai & Lei, NTU, ACM SAC'17).

---

## 📂 Repository Architecture

```text
rust-os/
├── .gitignore                      # Strictly excludes >100 MB files (ISO, VDI, EXE)
├── README.md                       # Master engineering documentation
├── Install-VirtualBox.bat          # 1-Click automated Oracle VirtualBox 7.x installer
├── Install-VirtualBox.ps1          # PowerShell installer with Oracle CDN downloader
├── Start-MicroOS.bat               # 1-Click VM provisioner & launcher for Debian MicroOS
├── Start-RustOS.bat                # 1-Click VM provisioner & launcher for RustOS
├── Run-QEMU.bat                    # Instant zero-install launcher via native QEMU
│
├── micro-os/                       # Debian Live Linux Subsystem
│   ├── build_clean.sh              # Headless debootstrap, chroot & hybrid ISO authoring
│   ├── build_wsl.bat               # 1-Click Windows/WSL wrapper for build_clean.sh
│   ├── config/                     # Desktop shortcuts, lightdm.conf, isolinux.cfg
│   └── research-lab/               # Academic Research Lab C engine & Makefile
│       ├── Makefile
│       └── research_lab.c          # Tsai & Lei ACM SAC'17 algorithm evaluation engine
│
├── rustos/                         # Bare-Metal 32-bit x86 Rust Kernel
│   ├── Cargo.toml                  # Freestanding kernel manifest
│   ├── linker.ld                   # Multiboot-compliant 32-bit ELF linker script
│   ├── i686-rustos.json            # Target specification for freestanding x86
│   └── src/
│       ├── main.rs                 # Kernel entry point (_start) & boot orchestrator
│       ├── vga.rs                  # 80x25 VGA text mode driver with color & cursor
│       ├── port.rs                 # x86 I/O port assembly operations (inb, outb)
│       ├── gdt.rs                  # Global Descriptor Table & segment reload
│       ├── pic.rs                  # Dual 8259 PIC initialization & IRQ remapping
│       ├── pit.rs                  # 8254 PIT timer driver (100 Hz scheduling)
│       ├── idt.rs                  # Interrupt Descriptor Table & ISR wrappers
│       ├── keyboard.rs             # PS/2 keyboard controller & scan code decoder
│       ├── pagerep.rs              # In-kernel Tsai & Lei algorithm visualizer
│       ├── snake.rs                # In-kernel bare-metal retro arcade Snake game
│       └── shell.rs                # Interactive command-line shell (aether-os>)
│
├── tools/
│   └── research-lab-cli/           # Standalone Rust research benchmark suite for host
│       ├── Cargo.toml
│       └── src/main.rs
│
└── docs/                           # Academic Documentation & Presentation Suite
    ├── RESEARCH_PAPER_ANALYSIS.md  # Formal paper breakdown (theory, math, results)
    ├── PRESENTATION_GUIDE.md       # 10-minute viva script & top 10 professor Q&A
    ├── BOOT_BENCHMARKS.md          # Boot timing, RAM consumption, and storage metrics
    └── HARDWARE_COMPATIBILITY.md   # CPU, memory, and hypervisor compatibility matrix
```

---

## ⚡ Quick Start: 3 Ways to Run

### Method 1: Oracle VirtualBox (Automated 1-Click)
1. If you don't have Oracle VirtualBox installed, simply run:
   ```cmd
   Install-VirtualBox.bat
   ```
   *This automatically downloads VirtualBox 7.x directly from Oracle and installs it with administrator approval.*
2. Launch the systems:
   * Double-click `Start-MicroOS.bat` to provision and launch Debian MicroOS.
   * Double-click `Start-RustOS.bat` to provision and launch RustOS.

### Method 2: Native QEMU (Instant, No Installation Required)
QEMU is already available on your machine at `C:\Program Files\qemu\qemu-system-x86_64.exe`!
1. Double-click:
   ```cmd
   Run-QEMU.bat
   ```
2. Choose `[1]` for Debian MicroOS or `[2]` for RustOS bare metal.

---

## 🔬 Academic Research Lab (ACM SAC'17)

The integrated research laboratory evaluates the paper:
> **"A Page Replacement Algorithm Based on Frequency Derived From Reference History"**  
> *Hong-Bin Tsai & Chin-Laung Lei (National Taiwan University, ACM SAC '17)*

### Key Theoretical Contributions:
1. **Sliding Window History Queue**: Dynamically tracks recent memory references $W$ and maintains per-page frequency counters $RC(P_k)$ with automatic tail eviction.
2. **Analytical Threshold Window Equation**:
   $$W_{thres} = \left\lceil \frac{hN}{1 - h} \right\rceil \times \lceil hN \times M \rceil \quad \text{where} \quad M = \left( 1 + \frac{hN}{F} \right)^2$$
3. **The "Warm Zone" Discovery**: In 40/60 and 45/55 workloads, multiplying $W_{thres}$ by $2\times$ or $5\times$ matches the hit ratio of an infinite log within $0.007\%$.
4. **Benchmark Proof**: Outperforms standard LRU by **up to 11.20%**, NFU by **9.21%**, and AGING by **8.18%**.

### How to Run the Research Lab:
- **Inside Debian MicroOS**: Double-click the `Research Lab` desktop icon or execute `/usr/local/bin/research-lab`.
- **Inside RustOS Kernel**: In the kernel shell, type `pagerep` to watch the algorithm execute live on bare metal.
- **On the Host Terminal**: Execute `tools/research-lab-cli` with Cargo.

---

## 🦀 RustOS Bare-Metal Kernel Commands

Once booted into the `aether-os> ` prompt:
* `help` - Show all available kernel commands
* `version` - Display kernel version, CPU mode, and architecture
* `uptime` - Read system uptime serviced by PIT timer ticks
* `ticks` - Display exact hardware timer interrupt count
* `meminfo` - Inspect physical memory layout and page directory configuration
* `colors` - Verify 16-color VGA text mode palette
* `pagerep` - Run the in-kernel ACM SAC'17 Page Replacement demonstration
* `snake` - Play the in-kernel retro arcade Snake game
* `reboot` - Soft reboot the system via the 8042 keyboard controller
* `halt` - Halt the CPU in a low-power sleep state

---

## 🛡️ GitHub Push Policy
This repository is configured to guarantee frictionless GitHub pushes:
* All generated ISOs, virtual disks, and installer executables are strictly ignored in `.gitignore`.
* Zero files exceed GitHub's 100 MB hard limit.

# Viva Defense & Academic Presentation Guide

## Project Title
**A Dual-OS Engineering Architecture: Debian MicroOS with Integrated Research Laboratory & Freestanding Bare-Metal RustOS**

---

## 🕒 10-Minute Presentation Blueprint

| Time | Slide / Topic | Key Visuals & Talking Points |
| :--- | :--- | :--- |
| **0:00 - 1:30** | **1. Introduction & Objectives** | - Dual-OS concept: Production Live Linux + Freestanding Bare-Metal Kernel.<br>- Research Focus: Tsai & Lei (ACM SAC'17) Page Replacement Algorithm.<br>- Live 1-Click VM automation for VirtualBox and native QEMU. |
| **1:30 - 3:30** | **2. Research Paper Core Theory** | - Shift from hardware I/O bottlenecks to software paging bottlenecks (PMEM / PCM).<br>- Explain Algorithm 1: Sliding window queue + reference count table.<br>- Highlight the mathematical formula for $W_{thres}$ and the "Warm Zone" (40/60). |
| **3:30 - 5:30** | **3. Experimental Results** | - Hit ratio improvement: **+11.2% over LRU**, **+9.2% over NFU**, **+8.2% over AGING**.<br>- Show reproduction of Paper Figure 4 (Hotness sweep from 10/90 to 45/55).<br>- Explain why LRU fails in non-uniform access loops compared to bounded frequency logs. |
| **5:30 - 7:30** | **4. Live Demonstration** | - **Part A**: Launch Debian MicroOS in VirtualBox/QEMU -> Open Desktop -> Run `Research Lab` tool.<br>- **Part B**: Boot bare-metal RustOS -> Run `pagerep` (in-kernel research demo) -> Run `snake`. |
| **7:30 - 8:30** | **5. Low-Level Architecture (RustOS)** | - `#![no_std]`, `#![no_main]` freestanding execution.<br>- GDT, IDT exception/IRQ dispatching, 8259 PIC remapping, 100Hz PIT scheduling, VGA text buffer driver. |
| **8:30 - 10:00** | **6. Conclusion & Viva Q&A** | - Summary of deliverables, GitHub-ready repository hygiene (<100MB policy), and open floor for faculty questions. |

---

## 🖥️ Live Demonstration Script

### Step 1: Debian MicroOS Live Environment
1. Launch the system:
   ```cmd
   Start-MicroOS.bat
   ```
2. Explain the environment:
   * "The system boots in under 12 seconds from a compressed hybrid SquashFS ISO."
   * "Autologin enters XFCE4 directly as non-root user `user` with sudo privileges."
3. Double-click the **Research Lab (ACM SAC'17)** desktop shortcut:
   * Select Option `1` to run the baseline **40/60 Warm Zone** benchmark.
   * Point out the ASCII bar charts comparing OPTIMAL, Log-Based, LRU, NFU, AGING, and RANDOM.
   * Highlight the **+11% hit ratio delta over LRU**.
4. Double-click the **Retro Snake Game** desktop icon to demonstrate full X11 desktop usability and user interactivity.

### Step 2: RustOS Bare-Metal Kernel
1. Launch RustOS:
   ```cmd
   Start-RustOS.bat
   ```
   *(Or `Run-QEMU.bat` -> Option 2)*
2. In the `aether-os> ` shell, run:
   ```text
   version
   ```
   *Explain*: "Notice the kernel boots without any C runtime or OS abstraction. It initializes GDT, remaps the dual 8259 PIC, programs the PIT timer at 100 Hz, and installs IDT interrupt vectors."
3. Run the in-kernel page replacement demonstration:
   ```text
   pagerep
   ```
   *Explain*: "Here we observe the Tsai & Lei sliding-window eviction logic executing directly in protected mode x86 bare metal, displaying real-time frame replacements and reference counts."
4. Run the in-kernel retro arcade snake game:
   ```text
   snake
   ```
   *Explain*: "This demonstrates hardware timer synchronization (using PIT ticks) and non-blocking PS/2 keyboard scancode processing directly at the bare-metal level."

---

## 🎯 Top 10 Viva Defense Questions & Answers

### Q1: Why does the Tsai & Lei algorithm outperform LRU by up to 11%?
**Answer**:  
LRU relies solely on recency—the timestamp of the single most recent access. In access patterns with a mix of hot and cold pages (such as 40/60 or 30/70), cold pages accessed once can flush hot pages out of memory. Tsai & Lei's algorithm tracks frequency within a bounded history window ($W_{thres}$), ensuring that high-frequency hot pages remain protected even if a cold page was accessed slightly more recently.

### Q2: What is the "Warm Zone" in Section 4.2 of the paper?
**Answer**:  
When the hotness distribution approaches 40/60 or 45/55, the difference between the reference frequency of hot pages and cold pages shrinks to between 2% and 4.2%. Because the boundary between hot and cold pages is blurred, the standard analytical threshold formula slightly underestimates the required window size. Multiplying $W_{thres}$ by 2 (for 40/60) or 5 (for 45/55) stabilizes the hit ratio so it is within 0.007% of an infinite log.

### Q3: Why not use NFU (Not Frequently Used) instead of Log-Based?
**Answer**:  
NFU maintains cumulative global frequency counters that never decay. If a page was heavily referenced during program startup, its counter remains permanently high even after the application enters a completely different phase. Tsai & Lei's sliding log window automatically dequeues stale references from the tail, achieving natural frequency decay without artificial manual aging parameters.

### Q4: Why is 8259 PIC remapping mandatory on x86 protected mode?
**Answer**:  
By default, the legacy IBM PC BIOS maps IRQ 0–7 to CPU interrupts 0x08–0x0F. In protected mode, Intel reserved vectors 0x00–0x1F for CPU exceptions (e.g., INT 8 is Double Fault, INT 13 is General Protection Fault). Remapping the PIC moves IRQ 0–7 to vectors 0x20–0x27 and IRQ 8–15 to 0x28–0x2F, preventing hardware timer and keyboard interrupts from masquerading as catastrophic CPU faults.

### Q5: How does the PIT timer work in your RustOS kernel?
**Answer**:  
We program the 8254 PIT via I/O ports `0x43` (command) and `0x40` (data channel 0) with a divisor of $1193182 / 100 \approx 11931$. This generates a hardware timer interrupt every 10 milliseconds (100 Hz). In the IRQ0 interrupt service routine, we atomically increment a 64-bit tick counter and issue an EOI (`0x20`) to the PIC.

### Q6: What prevents data races in your VGA text driver without an operating system?
**Answer**:  
We implemented a custom `SpinLock<VgaWriter>` using `core::sync::atomic::AtomicBool` with acquire-release memory ordering. When printing, the lock is acquired, ensuring mutual exclusion between kernel shell printing and interrupt handlers.

### Q7: Why does Bélády's OPTIMAL algorithm have a higher hit ratio, and why can't we use it in a real OS?
**Answer**:  
Bélády's MIN looks into the future and evicts the page that will not be used for the longest period of time. It requires complete deterministic future knowledge of all program memory references, which is impossible in general-purpose online systems. It is strictly an offline theoretical upper bound used to measure the efficiency of online algorithms.

### Q8: How did you ensure the Debian MicroOS ISO stays under 450 MB?
**Answer**:  
We used `debootstrap --variant=minbase`, installed only essential X11 and XFCE packages with `--no-install-recommends`, purged documentation, man pages, and apt cache lists, and compressed the root filesystem with high-ratio `mksquashfs -comp xz`.

### Q9: What happens if VirtualBox is not installed on the testing host?
**Answer**:  
Our automated launcher `Start-MicroOS.bat` detects if `VBoxManage.exe` is absent and gives the user two automated paths: 
1. Run `Install-VirtualBox.bat` to automatically download and install Oracle VirtualBox 7.x with admin elevation, OR 
2. Instantly launch the OS using native QEMU (`qemu-system-x86_64`), which requires zero driver installation.

### Q10: How do you handle GitHub's 100 MB file limit?
**Answer**:  
Our repository `.gitignore` strictly excludes large binary artifacts: ISO files (`*.iso`), virtual disks (`*.vdi`, `*.vmdk`), build directories (`build/`, `target/`), and SquashFS files. The repository contains only pure source code and headless automated build recipes, guaranteeing fast, frictionless git pushes.

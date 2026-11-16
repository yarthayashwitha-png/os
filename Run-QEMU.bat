@echo off
setlocal enabledelayedexpansion
title AetherOS / RustOS: Instant QEMU Launcher

echo ========================================================================
echo   Native QEMU Launcher for AetherOS & RustOS
echo ========================================================================

set "QEMU_BIN=C:\Program Files\qemu\qemu-system-x86_64.exe"
if not exist "!QEMU_BIN!" (
    for /f "tokens=*" %%q in ('where qemu-system-x86_64 2^>nul') do set "QEMU_BIN=%%q"
)

if not exist "!QEMU_BIN!" (
    echo [-] QEMU binary not found.
    pause
    exit /b 1
)

echo [1] Launch Debian MicroOS (Live Linux GUI + Snake + Research Lab)
echo [2] Launch RustOS (Bare-Metal 32-bit x86 Kernel + Shell + Snake)
echo.
set /p OS_CHOICE="Select system to boot [1 or 2]: "

if "!OS_CHOICE!"=="1" (
    set "ISO_PATH=%~dp0build\aether-os.iso"
    if not exist "!ISO_PATH!" (
        if exist "%~dp0build\micro-os.iso" set "ISO_PATH=%~dp0build\micro-os.iso"
    )
    if not exist "!ISO_PATH!" (
        echo [!] Warning: build\aether-os.iso not found. Please build it first using micro-os\build_clean.sh.
        pause
        exit /b 1
    )
    echo [*] Booting AetherOS Live ISO in QEMU (2048 MB RAM, 2 Cores)...
    start "" "!QEMU_BIN!" -m 2048 -smp 2 -cdrom "!ISO_PATH!" -vga std -boot d
) else (
    set "KERNEL_BIN=%~dp0rustos\target\rustos-kernel.bin"
    if not exist "!KERNEL_BIN!" (
        set "KERNEL_BIN=%~dp0rustos\target\i686-rustos\debug\rustos"
    )
    echo [*] Booting RustOS Bare-Metal Kernel in QEMU...
    start "" "!QEMU_BIN!" -m 128 -kernel "!KERNEL_BIN!" -vga std
)

echo [+] Emulation running.

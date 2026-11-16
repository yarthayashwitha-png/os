@echo off
setlocal enabledelayedexpansion
title RustOS: 1-Click Kernel Launcher

echo ========================================================================
echo   RustOS: Bare-Metal 32-bit x86 Kernel Launcher
echo ========================================================================

:: 1. Locate VBoxManage
set "VBOX_BIN="
if exist "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe" (
    set "VBOX_BIN=C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
) else if exist "C:\Program Files (x86)\Oracle\VirtualBox\VBoxManage.exe" (
    set "VBOX_BIN=C:\Program Files (x86)\Oracle\VirtualBox\VBoxManage.exe"
) else (
    for /f "tokens=*" %%i in ('where VBoxManage 2^>nul') do set "VBOX_BIN=%%i"
)

:: 2. If VBoxManage not found, prompt for automated install or QEMU
if not defined VBOX_BIN (
    echo [!] Oracle VirtualBox is not currently installed.
    echo.
    echo   Please choose an option:
    echo   [1] Download and install Oracle VirtualBox 7.x automatically
    echo   [2] Launch RustOS immediately with native QEMU (Detected at C:\Program Files\qemu\)
    echo   [3] Cancel
    echo.
    set /p CHOICE="Enter choice [1, 2, or 3]: "
    if "!CHOICE!"=="1" (
        call "%~dp0Install-VirtualBox.bat"
        goto :EOF
    ) else if "!CHOICE!"=="2" (
        goto :LAUNCH_QEMU
    ) else (
        goto :EOF
    )
)

echo [+] Oracle VirtualBox detected at: "!VBOX_BIN!"

:: 3. Provision VirtualBox VM for RustOS
set "VM_NAME=RustOS-BareMetal"
set "ISO_PATH=%~dp0build\rustos.iso"

echo [*] Provisioning VirtualBox VM: "%VM_NAME%"...
"!VBOX_BIN!" controlvm "%VM_NAME%" poweroff >nul 2>&1
timeout /t 1 /nobreak >nul
"!VBOX_BIN!" unregistervm "%VM_NAME%" --delete >nul 2>&1

"!VBOX_BIN!" createvm --name "%VM_NAME%" --ostype "Other" --register
"!VBOX_BIN!" modifyvm "%VM_NAME%" --memory 128 --vram 16 --cpus 1 --boot1 dvd --boot2 disk --ioapic off --acpi on --graphicscontroller vboxvga

if exist "!ISO_PATH!" (
    "!VBOX_BIN!" storagectl "%VM_NAME%" --name "IDE" --add ide
    "!VBOX_BIN!" storageattach "%VM_NAME%" --storagectl "IDE" --port 0 --device 0 --type dvddrive --medium "!ISO_PATH!"
    echo [+] Starting RustOS in VirtualBox...
    "!VBOX_BIN!" startvm "%VM_NAME%" --type gui
) else (
    echo [!] Note: To boot in VirtualBox, build\rustos.iso is required.
    echo [*] Switching to native direct-kernel boot via QEMU...
    goto :LAUNCH_QEMU
)

goto :EOF

:LAUNCH_QEMU
set "QEMU_BIN=C:\Program Files\qemu\qemu-system-x86_64.exe"
if not exist "!QEMU_BIN!" (
    for /f "tokens=*" %%q in ('where qemu-system-x86_64 2^>nul') do set "QEMU_BIN=%%q"
)

if not exist "!QEMU_BIN!" (
    echo [-] Neither VirtualBox nor QEMU could be found.
    pause
    goto :EOF
)

set "KERNEL_BIN=%~dp0rustos\target\rustos-kernel.bin"
if not exist "!KERNEL_BIN!" (
    set "KERNEL_BIN=%~dp0rustos\target\i686-rustos\debug\rustos"
)

echo [+] Launching RustOS bare-metal kernel with QEMU: "!QEMU_BIN!"...
start "" "!QEMU_BIN!" -m 128 -kernel "!KERNEL_BIN!" -vga std
echo [+] QEMU started. Enjoy RustOS!
goto :EOF

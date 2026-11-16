@echo off
setlocal enabledelayedexpansion
title AetherOS Micro: 1-Click VM Launcher

echo ========================================================================
echo   AetherOS Micro: Automated VirtualBox & QEMU Launcher
echo ========================================================================

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo [*] Elevating permissions for VirtualBox setup...
    powershell -Command "Start-Process cmd.exe -ArgumentList '/c \"\"%~f0\"\"' -Verb RunAs"
    exit /b
)

:: 1. Locate VBoxManage
set "VBOX_BIN=C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
if not exist "!VBOX_BIN!" (
    if exist "C:\Program Files (x86)\Oracle\VirtualBox\VBoxManage.exe" (
        set "VBOX_BIN=C:\Program Files (x86)\Oracle\VirtualBox\VBoxManage.exe"
    ) else (
        for /f "tokens=*" %%i in ('where VBoxManage 2^>nul') do set "VBOX_BIN=%%i"
    )
)

if not exist "!VBOX_BIN!" (
    echo [!] VirtualBox not detected. Falling back to native QEMU...
    goto :LAUNCH_QEMU
)

echo [+] Oracle VirtualBox detected at: "!VBOX_BIN!"

set "VM_NAME=AetherOS-Micro"
set "HOST_PROJECT=%~dp0"
if "%HOST_PROJECT:~-1%"=="\" set "HOST_PROJECT=%HOST_PROJECT:~0,-1%"

:: 2. Provision VirtualBox VM
echo [*] Provisioning VirtualBox VM: "%VM_NAME%"...
"!VBOX_BIN!" controlvm "%VM_NAME%" poweroff >nul 2>&1
ping 127.0.0.1 -n 2 >nul
"!VBOX_BIN!" unregistervm "%VM_NAME%" --delete >nul 2>&1

echo [*] Registering Debian 64-bit VM...
"!VBOX_BIN!" createvm --name "%VM_NAME%" --ostype "Debian_64" --register
if %errorlevel% neq 0 (
    start "" "C:\Program Files\Oracle\VirtualBox\VirtualBox.exe"
    ping 127.0.0.1 -n 3 >nul
    "!VBOX_BIN!" createvm --name "%VM_NAME%" --ostype "Debian_64" --register
)

echo [*] Configuring Hardware & Bidirectional Clipboard...
"!VBOX_BIN!" modifyvm "%VM_NAME%" ^
    --memory 2048 ^
    --vram 128 ^
    --cpus 2 ^
    --boot1 dvd --boot2 disk ^
    --ioapic on ^
    --acpi on ^
    --rtcuseutc on ^
    --graphicscontroller vboxsvga ^
    --mouse usbtablet ^
    --audio-enabled on ^
    --clipboard-mode bidirectional ^
    --draganddrop bidirectional ^
    --nic1 nat

:: Port forwarding for SSH
"!VBOX_BIN!" modifyvm "%VM_NAME%" --natpf1 "guestssh,tcp,127.0.0.1,2222,,22"

:: Connect entire host project directory as Shared Folder
echo [*] Attaching Shared Folder: "!HOST_PROJECT!"...
"!VBOX_BIN!" sharedfolder add "%VM_NAME%" ^
    --name "ProjectWorkspace" ^
    --hostpath "!HOST_PROJECT!" ^
    --automount ^
    --auto-mount-point="/home/user/workspace"

:: Attach SATA Controller and ISO
echo [*] Attaching SATA controller and boot ISO...
"!VBOX_BIN!" storagectl "%VM_NAME%" --name "SATA" --add sata --controller IntelAhci --bootable on

set "ISO_PATH=%~dp0build\aether-os.iso"
if not exist "!ISO_PATH!" (
    if exist "%~dp0build\micro-os.iso" set "ISO_PATH=%~dp0build\micro-os.iso"
)

if exist "!ISO_PATH!" (
    "!VBOX_BIN!" storageattach "%VM_NAME%" --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium "!ISO_PATH!"
) else (
    echo [!] Note: No ISO found in build\ yet. Attaching empty drive.
    "!VBOX_BIN!" storageattach "%VM_NAME%" --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium emptydrive
)

echo ========================================================================
echo [+] SUCCESS! Launching AetherOS Micro in VirtualBox...
echo ========================================================================
"!VBOX_BIN!" startvm "%VM_NAME%" --type gui
goto :EOF

:LAUNCH_QEMU
set "QEMU_BIN=C:\Program Files\qemu\qemu-system-x86_64.exe"
if not exist "!QEMU_BIN!" (
    for /f "tokens=*" %%q in ('where qemu-system-x86_64 2^>nul') do set "QEMU_BIN=%%q"
)

if not exist "!QEMU_BIN!" (
    echo [-] Neither VirtualBox nor QEMU could be launched.
    pause
    goto :EOF
)

echo [+] Launching OS using QEMU: "!QEMU_BIN!"...
set "ISO_ARG="
if exist "%~dp0build\aether-os.iso" (
    set "ISO_ARG=-cdrom ^"%~dp0build\aether-os.iso^""
) else if exist "%~dp0build\micro-os.iso" (
    set "ISO_ARG=-cdrom ^"%~dp0build\micro-os.iso^""
)

start "" "!QEMU_BIN!" -m 2048 -smp 2 -vga std !ISO_ARG! -boot d
echo [+] QEMU process started!
goto :EOF

@echo off
setlocal enabledelayedexpansion
title Fix Boot Medium & Launch AetherOS

echo ========================================================================
echo   Fixing VirtualBox Boot Medium & Launching AetherOS
echo ========================================================================

set "VBOX_BIN=C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
set "ISO_PATH=%~dp0build\aether-os.iso"

:: 1. Check if ISO exists; if not, download it automatically
if not exist "!ISO_PATH!" (
    echo [!] Bootable ISO not found in build\aether-os.iso
    echo [*] Downloading bootable Debian Live ISO now... Please wait.
    powershell -ExecutionPolicy Bypass -File "%~dp0Download-Live-ISO.ps1"
)

if not exist "!ISO_PATH!" (
    echo.
    echo [-] Could not automatically download the ISO.
    echo [!] Please manually download any Debian 12 Live ISO and save it as:
    echo     !ISO_PATH!
    echo.
    pause
    exit /b 1
)

echo.
echo [+] Bootable ISO confirmed at: "!ISO_PATH!"

:: 2. Attach ISO to VirtualBox VM
echo [*] Attaching ISO to VirtualBox virtual optical drive...
"!VBOX_BIN!" storageattach "AetherOS-Micro" --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium "!ISO_PATH!"
if %errorlevel% neq 0 (
    echo [!] Reconnecting VM controller...
    call "%~dp0Connect-To-VirtualBox.bat"
    "!VBOX_BIN!" storageattach "AetherOS-Micro" --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium "!ISO_PATH!"
)

:: 3. Configure Network for Wi-Fi Internet Access
echo [*] Ensuring Network NAT is enabled for Laptop Wi-Fi access...
"!VBOX_BIN!" modifyvm "AetherOS-Micro" --nic1 nat

:: 4. Start the VM
echo ========================================================================
echo [+] SUCCESS! Boot medium attached. Starting AetherOS...
echo ========================================================================
"!VBOX_BIN!" startvm "AetherOS-Micro" --type gui

echo.
echo Your OS is now booting in VirtualBox!
pause

@echo off
setlocal enabledelayedexpansion
title Auto-Attach ISO to VirtualBox

echo ========================================================================
echo   Attaching AetherOS Boot ISO to VirtualBox
echo ========================================================================

set "DOWNLOADS_DIR=%USERPROFILE%\Downloads"
set "BUILD_DIR=%~dp0build"
set "TARGET_ISO=%BUILD_DIR%\aether-os.iso"
set "VBOX_BIN=C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"

if not exist "!BUILD_DIR!" mkdir "!BUILD_DIR!"

:: Find the newest .iso in Downloads
set "NEWEST_ISO="
for /f "tokens=*" %%f in ('dir /b /o:-d "%DOWNLOADS_DIR%\*.iso" 2^>nul') do (
    if not defined NEWEST_ISO (
        set "NEWEST_ISO=%DOWNLOADS_DIR%\%%f"
    )
)

if defined NEWEST_ISO (
    echo [+] Detected latest downloaded ISO: "!NEWEST_ISO!"
    echo [*] Copying to "!TARGET_ISO!"...
    copy /y "!NEWEST_ISO!" "!TARGET_ISO!" >nul
)

if not exist "!TARGET_ISO!" (
    echo [-] No .iso file found in Downloads or build folder.
    pause
    exit /b 1
)

echo [+] Confirmed ISO at: "!TARGET_ISO!"

:: Power off existing VM first if running
echo [*] Powering off active VM session...
"!VBOX_BIN!" controlvm "AetherOS-Micro" poweroff >nul 2>&1
ping 127.0.0.1 -n 2 >nul

:: Attach ISO to VirtualBox VM
echo [*] Attaching ISO to VirtualBox VM 'AetherOS-Micro'...
"!VBOX_BIN!" storageattach "AetherOS-Micro" --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium "!TARGET_ISO!"

echo ========================================================================
echo [+] SUCCESS! ISO attached. Powering on VirtualBox...
echo ========================================================================
"!VBOX_BIN!" startvm "AetherOS-Micro" --type gui

echo.
echo VirtualBox is now booting into your Windows-style desktop!
pause

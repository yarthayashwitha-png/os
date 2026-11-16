@echo off
setlocal enabledelayedexpansion
title AetherOS Micro: WSL ISO Automated Builder

echo ========================================================================
echo   AetherOS Micro: Headless ISO Builder (WSL Host)
echo ========================================================================
echo.

set "SCRIPT_DIR=%~dp0"
set "ROOT_DIR=%SCRIPT_DIR%..\"
set "TARGET_ISO=%ROOT_DIR%build\aether-os.iso"

where wsl >nul 2>nul
if %errorlevel% neq 0 (
    echo [-] WSL is not installed on your system.
    pause
    exit /b 1
)

echo [+] WSL detected.
wsl sudo apt-get update
wsl sudo apt-get install -y debootstrap squashfs-tools xorriso isolinux syslinux-common

wsl sed -i "s/\r$//" micro-os/build_clean.sh
wsl sed -i "s/\r$//" micro-os/config/micro-dashboard.sh
wsl sed -i "s/\r$//" micro-os/config/boot-benchmark.sh
wsl sed -i "s/\r$//" micro-os/config/apt-gui.sh

wsl sudo bash micro-os/build_clean.sh

if exist "%TARGET_ISO%" (
    echo.
    echo ========================================================================
    echo [+] SUCCESS! ISO confirmed at: %TARGET_ISO%
    echo ========================================================================
    echo Launching VirtualBox with your new Windows-style OS...
    call "%ROOT_DIR%Start-MicroOS.bat"
) else (
    echo.
    echo [-] Build did not produce %TARGET_ISO%.
)

pause

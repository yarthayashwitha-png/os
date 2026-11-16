@echo off
setlocal
title Download Debian Live ISO
echo ========================================================================
echo   Downloading Debian Live ISO into build\aether-os.iso
echo ========================================================================

powershell -ExecutionPolicy Bypass -File "%~dp0Download-Live-ISO.ps1"

if exist "%~dp0build\aether-os.iso" (
    echo.
    echo [+] ISO is ready! You can now start the VM:
    echo     Start-MicroOS.bat
)
pause

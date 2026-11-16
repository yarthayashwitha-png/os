@echo off
setlocal
title Oracle VirtualBox 7 1-Click Installer
echo ========================================================================
echo   Oracle VirtualBox 7 Automated Setup Wizard
echo ========================================================================

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo [*] Elevating permissions to Administrator for VirtualBox driver setup...
    powershell -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

echo [+] Running with Administrator privileges.
powershell -ExecutionPolicy Bypass -File "%~dp0Install-VirtualBox.ps1"

echo.
echo Press any key to exit...
pause >nul

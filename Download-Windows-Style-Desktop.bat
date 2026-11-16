@echo off
setlocal enabledelayedexpansion
title Download Windows-Style Desktop ISO (Debian XFCE)

echo ========================================================================
echo   Upgrading AetherOS to Full Windows-Style Graphical Desktop
echo ========================================================================
echo.
echo [*] Opening official high-speed download in your browser...
echo.

set "XFCE_URL=https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-12.8.0-amd64-xfce.iso"
set "FALLBACK_URL=https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-12.9.0-amd64-xfce.iso"

start "" "!XFCE_URL!"

echo ------------------------------------------------------------------------
echo  INSTRUCTIONS:
echo  1. Your browser will start downloading: debian-live-12.8.0-amd64-xfce.iso
echo  2. Once downloaded to your Downloads folder, simply run:
echo     Attach-My-ISO.bat
echo ------------------------------------------------------------------------
echo.
pause

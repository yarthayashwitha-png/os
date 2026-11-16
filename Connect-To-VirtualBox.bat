@echo off
setlocal enabledelayedexpansion
title Connect Entire Project to VirtualBox

echo ========================================================================
echo   Connecting AetherOS Project to Oracle VirtualBox
echo ========================================================================

:: 1. Check for Administrator elevation (needed for fresh VirtualBox DCOM access)
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo [*] Requesting Administrator permission for VirtualBox DCOM access...
    powershell -Command "Start-Process cmd.exe -ArgumentList '/c \"\"%~f0\"\"' -Verb RunAs"
    exit /b
)

:: 2. Locate VBoxManage
set "VBOX_BIN=C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
if not exist "!VBOX_BIN!" (
    if exist "C:\Program Files (x86)\Oracle\VirtualBox\VBoxManage.exe" (
        set "VBOX_BIN=C:\Program Files (x86)\Oracle\VirtualBox\VBoxManage.exe"
    ) else (
        for /f "tokens=*" %%i in ('where VBoxManage 2^>nul') do set "VBOX_BIN=%%i"
    )
)

if not exist "!VBOX_BIN!" (
    echo [-] VirtualBox not found. Please install it using Install-VirtualBox.bat.
    pause
    exit /b 1
)

set "VM_NAME=AetherOS-Micro"
set "HOST_PROJECT=%~dp0"
if "%HOST_PROJECT:~-1%"=="\" set "HOST_PROJECT=%HOST_PROJECT:~0,-1%"

echo [+] VirtualBox found: "!VBOX_BIN!"
echo [+] Host Project Path: "!HOST_PROJECT!"

:: 3. Clean any existing instance
echo [*] Checking existing VM status...
"!VBOX_BIN!" controlvm "%VM_NAME%" poweroff >nul 2>&1
ping 127.0.0.1 -n 2 >nul
"!VBOX_BIN!" unregistervm "%VM_NAME%" --delete >nul 2>&1

:: 4. Register VM in VirtualBox
echo [*] Registering VM "%VM_NAME%" in VirtualBox...
"!VBOX_BIN!" createvm --name "%VM_NAME%" --ostype "Debian_64" --register
if %errorlevel% neq 0 (
    echo [-] VirtualBox COM service is initializing. Starting VirtualBox GUI to wake service...
    start "" "C:\Program Files\Oracle\VirtualBox\VirtualBox.exe"
    ping 127.0.0.1 -n 3 >nul
    "!VBOX_BIN!" createvm --name "%VM_NAME%" --ostype "Debian_64" --register
)

if %errorlevel% neq 0 (
    echo [-] Failed to register VM. Please ensure VirtualBox GUI is open and retry.
    pause
    exit /b 1
)

:: 5. Configure Hardware, Bidirectional Clipboard & Drag-and-Drop
echo [*] Configuring Hardware (2048 MB RAM, 2 Cores, 128 MB VRAM, Bidirectional Clipboard)...
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

:: 6. Configure SSH Port Forwarding
echo [*] Configuring NAT Port Forwarding (Host 2222 -^> Guest 22 for SSH)...
"!VBOX_BIN!" modifyvm "%VM_NAME%" --natpf1 "guestssh,tcp,127.0.0.1,2222,,22"

:: 7. Mount Host Project as a VirtualBox Shared Folder
echo [*] Connecting entire project folder as auto-mounting Shared Folder...
"!VBOX_BIN!" sharedfolder add "%VM_NAME%" ^
    --name "ProjectWorkspace" ^
    --hostpath "!HOST_PROJECT!" ^
    --automount ^
    --auto-mount-point="/home/user/workspace"

:: 8. Storage Controller and ISO attachment
echo [*] Configuring SATA Storage Controller...
"!VBOX_BIN!" storagectl "%VM_NAME%" --name "SATA" --add sata --controller IntelAhci --bootable on

set "ISO_PATH=%~dp0build\aether-os.iso"
if not exist "!ISO_PATH!" (
    if exist "%~dp0build\micro-os.iso" set "ISO_PATH=%~dp0build\micro-os.iso"
)

if exist "!ISO_PATH!" (
    echo [+] Attaching OS Boot Disk: "!ISO_PATH!"
    "!VBOX_BIN!" storageattach "%VM_NAME%" --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium "!ISO_PATH!"
) else (
    echo [!] Note: No ISO found in build\ yet. A virtual DVD drive has been created.
    "!VBOX_BIN!" storageattach "%VM_NAME%" --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium emptydrive
)

echo ========================================================================
echo [+] SUCCESS! Your entire project is now connected to VirtualBox!
echo ========================================================================
echo  * VM Name            : %VM_NAME% (visible in VirtualBox GUI)
echo  * Connected Folder   : !HOST_PROJECT!
echo  * Guest Mount Point  : /home/user/workspace
echo  * Clipboard / DnD    : Bidirectional (Copy-Paste between Windows ^& VM)
echo  * SSH Port Access    : localhost:2222
echo ========================================================================
echo.
echo Launching Oracle VirtualBox Manager GUI...
start "" "C:\Program Files\Oracle\VirtualBox\VirtualBox.exe"

echo.
echo Press any key to start this VM right now, or close this window.
pause
"!VBOX_BIN!" startvm "%VM_NAME%" --type gui

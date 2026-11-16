<#
.SYNOPSIS
    Automated Oracle VM VirtualBox 7.x Downloader and Setup Script for Windows.
.DESCRIPTION
    Downloads the official Oracle VirtualBox installer directly from oracle.com,
    verifies the download, and initiates installation with administrative privileges.
#>

[CmdletBinding()]
param (
    [switch]$Silent = $false
)

$ErrorActionPreference = "Stop"

$VBOX_DEFAULT_PATH = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
if (Test-Path $VBOX_DEFAULT_PATH) {
    Write-Host "[+] Oracle VirtualBox is ALREADY installed at: $VBOX_DEFAULT_PATH" -ForegroundColor Green
    & "$VBOX_DEFAULT_PATH" --version
    Write-Host "[+] You can now launch your OS using Start-MicroOS.bat or Start-RustOS.bat!" -ForegroundColor Cyan
    exit 0
}

Write-Host "========================================================================" -ForegroundColor Cyan
echo        AetherOS: Automated Oracle VirtualBox 7.x Setup Wizard             
Write-Host "========================================================================" -ForegroundColor Cyan

$InstallerDir = Join-Path $PSScriptRoot "tools\installers"
if (-not (Test-Path $InstallerDir)) {
    New-Item -ItemType Directory -Path $InstallerDir -Force | Out-Null
}

$InstallerPath = Join-Path $InstallerDir "VirtualBox-Installer.exe"

# Official Oracle VirtualBox 7.1.6 Direct Download Link
$VboxUrl = "https://download.virtualbox.org/virtualbox/7.1.6/VirtualBox-7.1.6-167084-Win.exe"
$VboxFallbackUrl = "https://download.virtualbox.org/virtualbox/7.0.20/VirtualBox-7.0.20-163906-Win.exe"

Write-Host "[*] Target Installer: $InstallerPath" -ForegroundColor Yellow

if (-not (Test-Path $InstallerPath) -or (Get-Item $InstallerPath).Length -lt 50000000) {
    Write-Host "[+] Downloading Oracle VirtualBox 7.x directly from Oracle servers..." -ForegroundColor Cyan
    Write-Host "    URL: $VboxUrl" -ForegroundColor Gray

    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
        $webClient = New-Object System.Net.WebClient
        $webClient.Headers.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        
        Write-Host "    Downloading... (approx. 105 MB). Please wait." -ForegroundColor Yellow
        $webClient.DownloadFile($VboxUrl, $InstallerPath)
        Write-Host "[+] Download completed successfully!" -ForegroundColor Green
    } catch {
        Write-Warning "[-] Primary download link failed: $_. Trying fallback mirror..."
        try {
            $webClient.DownloadFile($VboxFallbackUrl, $InstallerPath)
            Write-Host "[+] Fallback download completed successfully!" -ForegroundColor Green
        } catch {
            Write-Error "[-] Failed to download installer automatically: $_"
            Write-Host "[!] Please download manually from: https://www.virtualbox.org/wiki/Downloads" -ForegroundColor Red
            exit 1
        }
    }
} else {
    Write-Host "[+] Installer already downloaded and cached locally." -ForegroundColor Green
}

Write-Host "[*] Launching Oracle VirtualBox Installer..." -ForegroundColor Cyan
Write-Host "[*] Note: Windows may display a User Account Control (UAC) prompt asking for permission." -ForegroundColor Yellow

if ($Silent) {
    Write-Host "[*] Performing unattended silent installation..." -ForegroundColor Cyan
    Start-Process -FilePath $InstallerPath -ArgumentList "--silent", "--ignore-reboot" -Verb RunAs -Wait
} else {
    Write-Host "[*] Opening standard installer wizard..." -ForegroundColor Cyan
    Start-Process -FilePath $InstallerPath -Verb RunAs -Wait
}

if (Test-Path $VBOX_DEFAULT_PATH) {
    Write-Host "========================================================================" -ForegroundColor Green
    Write-Host " SUCCESS! Oracle VirtualBox is now installed on your system!" -ForegroundColor Green
    Write-Host " Location: $VBOX_DEFAULT_PATH" -ForegroundColor Green
    Write-Host "========================================================================" -ForegroundColor Green
    & "$VBOX_DEFAULT_PATH" --version
} else {
    Write-Host "[!] Installation completed. If prompted to restart your computer, please restart to load kernel drivers." -ForegroundColor Yellow
}

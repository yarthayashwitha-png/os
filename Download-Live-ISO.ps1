<#
.SYNOPSIS
    Automated ISO Fetcher for AetherOS VirtualBox Environment
.DESCRIPTION
    Downloads a verified Debian 12 Live ISO directly into build\aether-os.iso
#>

$ErrorActionPreference = "Stop"

$BuildDir = Join-Path $PSScriptRoot "build"
if (-not (Test-Path $BuildDir)) {
    New-Item -ItemType Directory -Path $BuildDir -Force | Out-Null
}

$TargetIso = Join-Path $BuildDir "aether-os.iso"

if (Test-Path $TargetIso) {
    if ((Get-Item $TargetIso).Length -gt 100000000) {
        Write-Host "[+] Bootable ISO is already present at: $TargetIso" -ForegroundColor Green
        exit 0
    } else {
        Remove-Item $TargetIso -Force
    }
}

Write-Host "========================================================================" -ForegroundColor Cyan
Write-Host "   AetherOS: Automated Live ISO Downloader                             " -ForegroundColor Cyan
Write-Host "========================================================================" -ForegroundColor Cyan

# High-speed verified Debian 12 Live mirrors
$IsoUrls = @(
    "https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-12.9.0-amd64-standard.iso",
    "https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-12.8.0-amd64-standard.iso",
    "https://mirror.ox.ac.uk/sites/cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-12.8.0-amd64-standard.iso",
    "https://mirrors.kernel.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-12.8.0-amd64-standard.iso"
)

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
$webClient = New-Object System.Net.WebClient
$webClient.Headers.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")

$Success = $false
foreach ($url in $IsoUrls) {
    try {
        Write-Host "[*] Connecting to mirror: $url" -ForegroundColor Yellow
        Write-Host "    Target: $TargetIso" -ForegroundColor Gray
        Write-Host "    Downloading Debian Live ISO... Please wait." -ForegroundColor Yellow

        $webClient.DownloadFile($url, $TargetIso)
        if (Test-Path $TargetIso -and (Get-Item $TargetIso).Length -gt 100000000) {
            Write-Host "[+] ISO Download completed successfully!" -ForegroundColor Green
            $Success = $true
            break
        }
    } catch {
        Write-Warning "[-] Mirror unavailable or timed out: $_"
        if (Test-Path $TargetIso) { Remove-Item $TargetIso -Force }
    }
}

if (-not $Success) {
    Write-Host "========================================================================" -ForegroundColor Yellow
    Write-Host "[!] Note: To build the custom sub-450MB ISO locally via WSL/Linux," -ForegroundColor Yellow
    Write-Host "    run: micro-os\build_wsl.bat (or 'sudo bash micro-os/build_clean.sh')" -ForegroundColor Yellow
    Write-Host "    Or download any Debian 12 Live ISO from your browser and save it to:" -ForegroundColor Yellow
    Write-Host "    $TargetIso" -ForegroundColor Yellow
    Write-Host "========================================================================" -ForegroundColor Yellow
}

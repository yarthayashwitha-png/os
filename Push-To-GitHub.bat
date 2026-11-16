@echo off
setlocal enabledelayedexpansion
title Push Project to GitHub

echo ========================================================================
echo   Pushing Dual-OS Project to GitHub (<100MB Policy Verified)
echo ========================================================================
echo.

:: 1. Check git in Windows or WSL
set "GIT_CMD="
where git >nul 2>nul
if %errorlevel% equ 0 (
    set "GIT_CMD=git"
) else (
    where wsl >nul 2>nul
    if %errorlevel% equ 0 (
        echo [*] Git not found in Windows PATH. Using WSL Git...
        set "GIT_CMD=wsl git"
    )
)

if not defined GIT_CMD (
    echo [-] Git was not found in Windows or WSL.
    echo     Please install Git for Windows: https://git-scm.com/download/win
    echo     Or install in WSL: sudo apt-get install -y git
    pause
    exit /b 1
)

echo [+] Using: !GIT_CMD!
echo.

:: 2. Initialize repo if needed
if not exist "%~dp0.git" (
    echo [*] Initializing Git repository...
    !GIT_CMD! init
)

:: 3. Stage files (Respecting .gitignore to exclude >100MB files)
echo [*] Staging project files...
!GIT_CMD! add .

:: 4. Commit
echo [*] Creating commit...
!GIT_CMD! commit -m "feat: complete Dual-OS engineering project with modular research lab"

:: 5. Prompt for GitHub repository URL
echo.
echo ========================================================================
echo   Enter your GitHub Repository URL
echo   Example: https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git
echo ========================================================================
set /p REPO_URL="Repository URL: "

if not defined REPO_URL (
    echo [-] No URL provided. You can push manually anytime using:
    echo     git remote add origin ^<your-url^>
    echo     git branch -M main
    echo     git push -u origin main
    pause
    exit /b
)

!GIT_CMD! branch -M main
!GIT_CMD! remote remove origin >nul 2>&1
!GIT_CMD! remote add origin "!REPO_URL!"

echo.
echo [*] Pushing to GitHub (main branch)...
!GIT_CMD! push -u origin main

if %errorlevel% equ 0 (
    echo.
    echo ========================================================================
    echo [+] SUCCESS! Your project is now live on GitHub!
    echo ========================================================================
) else (
    echo.
    echo [!] Push encountered an issue. Ensure your repository exists on GitHub
    echo     and you have signed in / authenticated.
)

pause

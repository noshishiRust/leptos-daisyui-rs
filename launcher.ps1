#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Leptos-daisyUI Build & Launch Automation Script
.DESCRIPTION
    Interactive menu for all build workflows, tests, and demo application tasks
.EXAMPLE
    .\launcher.ps1
#>

param(
    [string]$Task = ""
)

# Color functions
function Write-Success { param($Message) Write-Host $Message -ForegroundColor Green }
function Write-Info { param($Message) Write-Host $Message -ForegroundColor Cyan }
function Write-Warning { param($Message) Write-Host $Message -ForegroundColor Yellow }
function Write-Error { param($Message) Write-Host $Message -ForegroundColor Red }
function Write-Header { param($Message) Write-Host "`n═══════════════════════════════════════════════════════" -ForegroundColor Magenta; Write-Host " $Message" -ForegroundColor Magenta; Write-Host "═══════════════════════════════════════════════════════`n" -ForegroundColor Magenta }

# Check prerequisites
function Test-Prerequisites {
    $missing = @()
    $warnings = @()

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        $missing += "cargo (Rust)"
    }
    if (-not (Get-Command cargo-make -ErrorAction SilentlyContinue)) {
        $missing += "cargo-make"
    }
    if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
        $missing += "trunk"
    }

    # Check for Node.js (required for demo Tailwind CSS compilation)
    if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
        $warnings += "node (Node.js - required for demo CSS compilation)"
    }
    if (-not (Get-Command npx -ErrorAction SilentlyContinue)) {
        $warnings += "npx (comes with Node.js)"
    }

    if ($missing.Count -gt 0) {
        Write-Error "Missing prerequisites: $($missing -join ', ')"
        Write-Info "Install missing tools:"
        if ($missing -contains "cargo (Rust)") {
            Write-Info "  - Rust: https://rustup.rs/"
        }
        if ($missing -contains "cargo-make") {
            Write-Info "  - cargo-make: cargo install cargo-make"
        }
        if ($missing -contains "trunk") {
            Write-Info "  - trunk: cargo install trunk"
        }
        return $false
    }

    if ($warnings.Count -gt 0) {
        Write-Warning "Optional dependencies missing:"
        Write-Warning "  - Node.js (required for demo app styling)"
        Write-Info "Install Node.js:"
        Write-Info "  - winget: winget install OpenJS.NodeJS"
        Write-Info "  - Download: https://nodejs.org/"
        Write-Warning "`nDemo will run but without proper styling until Node.js is installed."
        Write-Host ""
    }

    return $true
}

# Execute cargo-make task
function Invoke-CargoMake {
    param([string]$TaskName, [string]$Description)

    Write-Header $Description
    Write-Info "Running: cargo make $TaskName"

    $startTime = Get-Date
    cargo make $TaskName
    $exitCode = $LASTEXITCODE
    $duration = (Get-Date) - $startTime

    Write-Host ""
    if ($exitCode -eq 0) {
        Write-Success "✓ Task completed successfully in $($duration.TotalSeconds.ToString('F2'))s"
    } else {
        Write-Error "✗ Task failed with exit code $exitCode"
    }

    return $exitCode
}

# Execute command in new window
function Start-InNewWindow {
    param([string]$Command, [string]$WorkingDir = $PWD, [string]$Title)

    $wtPath = Get-Command wt -ErrorAction SilentlyContinue

    if ($wtPath) {
        # Use Windows Terminal if available
        Start-Process wt -ArgumentList "-d `"$WorkingDir`" --title `"$Title`" pwsh -NoExit -Command `"$Command`""
    } else {
        # Fallback to regular PowerShell
        Start-Process pwsh -ArgumentList "-NoExit", "-Command", "cd '$WorkingDir'; $Command" -WindowStyle Normal
    }

    Write-Success "Launched '$Title' in new window"
}

# Show main menu
function Show-Menu {
    Clear-Host
    Write-Host ""
    Write-Host "╔════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║     Leptos-daisyUI Build & Launch Menu                        ║" -ForegroundColor Cyan
    Write-Host "║     Component Library: 62/62 components (100%)                ║" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""

    Write-Host "  QUICK ACTIONS" -ForegroundColor Yellow
    Write-Host "  ────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "  [1]  Launch Demo App            (trunk serve)" -ForegroundColor Green
    Write-Host "  [2]  Quick Check                (format + fix + test)" -ForegroundColor Green
    Write-Host "  [3]  Fix & Verify               (fix all issues, then verify)" -ForegroundColor Green
    Write-Host ""

    Write-Host "  BUILD & TEST" -ForegroundColor Yellow
    Write-Host "  ────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "  [4]  Build Library              (cargo build)"
    Write-Host "  [5]  Build Release              (cargo build --release)"
    Write-Host "  [6]  Run Tests                  (cargo test with output)"
    Write-Host "  [7]  Check Code                 (cargo check)"
    Write-Host "  [8]  Verify All                 (format + build + test + clippy)"
    Write-Host ""

    Write-Host "  CODE QUALITY" -ForegroundColor Yellow
    Write-Host "  ────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "  [9]  Format Code                (rustfmt)"
    Write-Host "  [10] Clippy Lint                (with warnings as errors)"
    Write-Host "  [11] Clippy Fix                 (auto-fix issues)"
    Write-Host "  [12] Fix All                    (format + clippy-fix)"
    Write-Host "  [13] CI Workflow                (full CI pipeline)"
    Write-Host ""

    Write-Host "  DEMO APPLICATION" -ForegroundColor Yellow
    Write-Host "  ────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "  [14] Demo: Dev Server           (trunk serve on :3000)" -ForegroundColor Green
    Write-Host "  [15] Demo: Build Production     (trunk build --release)"
    Write-Host "  [16] Demo: Clean                (remove build artifacts)"
    Write-Host "  [17] Demo: Open in Browser      (launch http://127.0.0.1:3000)"
    Write-Host ""

    Write-Host "  DOCUMENTATION" -ForegroundColor Yellow
    Write-Host "  ────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "  [18] Build Documentation        (cargo doc)"
    Write-Host "  [19] Open Documentation         (build + open in browser)"
    Write-Host ""

    Write-Host "  UTILITIES" -ForegroundColor Yellow
    Write-Host "  ────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "  [20] Clean All                  (clean library + demo)"
    Write-Host "  [21] List All Tasks             (show all cargo-make tasks)"
    Write-Host "  [22] Show Git Status"
    Write-Host "  [23] Update Dependencies        (cargo update)"
    Write-Host "  [26] Setup Node.js              (install Node.js for demo CSS)" -ForegroundColor Green
    Write-Host ""

    Write-Host "  PARALLEL WORKFLOWS" -ForegroundColor Yellow
    Write-Host "  ────────────────────────────────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "  [24] Dev + Docs                 (demo server + docs in parallel)" -ForegroundColor Magenta
    Write-Host "  [25] Full Build                 (library + demo in parallel)" -ForegroundColor Magenta
    Write-Host ""

    Write-Host "  [0]  Exit" -ForegroundColor Red
    Write-Host ""
}

# Task handlers
function Invoke-Task {
    param([string]$Choice)

    switch ($Choice) {
        "1" {
            Write-Header "Launching Demo Application"
            Write-Info "Starting trunk serve on http://127.0.0.1:3000"
            Write-Info "Press Ctrl+C to stop the server"
            Write-Host ""
            Set-Location demo
            trunk serve
            Set-Location ..
        }
        "2" {
            Invoke-CargoMake "quick-check" "Quick Check (Format + Fix + Test)"
        }
        "3" {
            Invoke-CargoMake "fix-and-verify" "Fix & Verify (Fix all issues, then verify)"
        }
        "4" {
            Invoke-CargoMake "build" "Build Library"
        }
        "5" {
            Invoke-CargoMake "build-release" "Build Release"
        }
        "6" {
            Invoke-CargoMake "test" "Run Tests"
        }
        "7" {
            Invoke-CargoMake "check" "Check Code"
        }
        "8" {
            Invoke-CargoMake "verify-all" "Verify All (Format + Build + Test + Clippy)"
        }
        "9" {
            Invoke-CargoMake "fmt" "Format Code"
        }
        "10" {
            Invoke-CargoMake "clippy" "Clippy Lint"
        }
        "11" {
            Invoke-CargoMake "clippy-fix" "Clippy Fix"
        }
        "12" {
            Invoke-CargoMake "fix-all" "Fix All (Format + Clippy-Fix)"
        }
        "13" {
            Invoke-CargoMake "ci" "CI Workflow"
        }
        "14" {
            Write-Header "Demo Development Server"
            Write-Info "Starting trunk serve on http://127.0.0.1:3000"
            Write-Info "Press Ctrl+C to stop the server"
            Write-Host ""
            Set-Location demo
            trunk serve
            Set-Location ..
        }
        "15" {
            Invoke-CargoMake "demo-build" "Demo: Build Production"
        }
        "16" {
            Invoke-CargoMake "demo-clean" "Demo: Clean"
        }
        "17" {
            Write-Header "Opening Demo in Browser"
            Start-Process "http://127.0.0.1:3000"
            Write-Success "Opened http://127.0.0.1:3000 in browser"
            Write-Warning "Make sure the demo server is running (option 1 or 14)"
        }
        "18" {
            Invoke-CargoMake "doc" "Build Documentation"
        }
        "19" {
            Invoke-CargoMake "doc-open" "Open Documentation"
        }
        "20" {
            Invoke-CargoMake "clean-all" "Clean All"
        }
        "21" {
            Write-Header "All Available cargo-make Tasks"
            cargo make --list-all-steps
        }
        "22" {
            Write-Header "Git Status"
            git status
            Write-Host ""
            git log --oneline -5
        }
        "23" {
            Write-Header "Update Dependencies"
            Write-Info "Running: cargo update"
            cargo update
            Write-Success "Dependencies updated"
        }
        "24" {
            Write-Header "Launching Dev Server + Documentation (Parallel)"
            Start-InNewWindow "cargo make demo-serve" -WorkingDir $PWD -Title "Demo Server"
            Start-Sleep -Milliseconds 500
            Start-InNewWindow "cargo make doc-open" -WorkingDir $PWD -Title "Documentation"
            Write-Success "Launched both demo server and documentation"
        }
        "25" {
            Write-Header "Full Build (Library + Demo in Parallel)"
            $job1 = Start-Job -ScriptBlock { Set-Location $using:PWD; cargo make build }
            $job2 = Start-Job -ScriptBlock { Set-Location $using:PWD; cargo make demo-build }

            Write-Info "Building library and demo in parallel..."
            Wait-Job $job1, $job2 | Out-Null

            Receive-Job $job1
            Receive-Job $job2

            Remove-Job $job1, $job2
            Write-Success "Parallel build completed"
        }
        "26" {
            Write-Header "Node.js Setup for Demo Styling"

            if (Get-Command node -ErrorAction SilentlyContinue) {
                $nodeVersion = node --version
                Write-Success "Node.js is already installed: $nodeVersion"

                if (Get-Command npx -ErrorAction SilentlyContinue) {
                    Write-Success "npx is available"

                    Write-Info "`nCompiling Tailwind CSS for demo..."
                    Set-Location demo
                    npx tailwindcss -i input.css -o output.css
                    Set-Location ..

                    if (Test-Path "demo/output.css") {
                        Write-Success "✓ Tailwind CSS compiled successfully!"
                        Write-Info "You can now run the demo with proper styling."
                    } else {
                        Write-Error "CSS compilation failed"
                    }
                }
            } else {
                Write-Warning "Node.js is not installed."
                Write-Info "`nInstallation options:"
                Write-Info "  1. Using winget (Windows Package Manager):"
                Write-Info "     winget install OpenJS.NodeJS"
                Write-Info "`n  2. Using Chocolatey:"
                Write-Info "     choco install nodejs"
                Write-Info "`n  3. Manual download:"
                Write-Info "     https://nodejs.org/ (LTS version recommended)"
                Write-Info "`nAfter installation, restart this terminal and run option [26] again."

                $response = Read-Host "`nInstall Node.js now using winget? (y/N)"
                if ($response -eq 'y' -or $response -eq 'Y') {
                    Write-Info "Installing Node.js via winget..."
                    winget install OpenJS.NodeJS

                    if ($LASTEXITCODE -eq 0) {
                        Write-Success "Node.js installed! Please restart your terminal."
                        Write-Info "Then run: .\launcher.ps1 -Task dev"
                    } else {
                        Write-Error "Installation failed. Please install manually."
                    }
                }
            }
        }
        "0" {
            Write-Info "Exiting launcher..."
            exit 0
        }
        default {
            Write-Warning "Invalid choice. Please try again."
        }
    }
}

# Main execution
function Main {
    if (-not (Test-Prerequisites)) {
        Write-Host ""
        Read-Host "Press Enter to exit"
        exit 1
    }

    # Handle direct task execution
    if ($Task) {
        switch ($Task.ToLower()) {
            "dev" { Invoke-Task "1" }
            "check" { Invoke-Task "2" }
            "fix" { Invoke-Task "3" }
            "build" { Invoke-Task "4" }
            "test" { Invoke-Task "6" }
            "docs" { Invoke-Task "19" }
            default {
                Write-Error "Unknown task: $Task"
                Write-Info "Available tasks: dev, check, fix, build, test, docs"
                exit 1
            }
        }
        return
    }

    # Interactive menu loop
    while ($true) {
        Show-Menu
        $choice = Read-Host "Select an option"
        Write-Host ""

        Invoke-Task $choice

        if ($choice -ne "0") {
            Write-Host ""
            Read-Host "Press Enter to continue"
        }
    }
}

# Run main function
Main

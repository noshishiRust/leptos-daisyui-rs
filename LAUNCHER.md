# Launcher Script Guide

The `launcher.ps1` PowerShell script provides an interactive menu for all build workflows and application launch tasks.

## Quick Start

### Interactive Menu
```powershell
.\launcher.ps1
```

This displays an interactive menu with all available options organized by category.

### Direct Task Execution
```powershell
.\launcher.ps1 -Task dev      # Launch demo app
.\launcher.ps1 -Task check    # Quick check
.\launcher.ps1 -Task fix      # Fix and verify
.\launcher.ps1 -Task build    # Build library
.\launcher.ps1 -Task test     # Run tests
.\launcher.ps1 -Task docs     # Build and open docs
```

## Prerequisites

The launcher automatically checks for required tools:
- ✅ **Rust/Cargo** - https://rustup.rs/
- ✅ **cargo-make** - `cargo install cargo-make`
- ✅ **trunk** - `cargo install trunk`

## Menu Categories

### 🚀 Quick Actions
Most commonly used tasks for daily development:
- **[1] Launch Demo App** - Start the demo server on http://127.0.0.1:3000
- **[2] Quick Check** - Fast pre-commit validation (format + fix + test)
- **[3] Fix & Verify** - Auto-fix all issues and verify everything passes

### 🔨 Build & Test
Core build and testing workflows:
- **[4] Build Library** - Build the leptos-daisyui-rs library
- **[5] Build Release** - Optimized production build
- **[6] Run Tests** - Execute all tests with output
- **[7] Check Code** - Fast syntax/type checking without building
- **[8] Verify All** - Complete verification pipeline

### ✨ Code Quality
Formatting and linting tools:
- **[9] Format Code** - Run rustfmt on all files
- **[10] Clippy Lint** - Run clippy with warnings as errors
- **[11] Clippy Fix** - Auto-fix clippy issues
- **[12] Fix All** - Format + clippy-fix in one command
- **[13] CI Workflow** - Full CI pipeline (same as GitHub Actions)

### 🎨 Demo Application
Demo-specific tasks:
- **[14] Dev Server** - Start development server with hot-reload
- **[15] Build Production** - Build demo for deployment
- **[16] Clean** - Remove demo build artifacts
- **[17] Open in Browser** - Launch http://127.0.0.1:3000

### 📚 Documentation
Generate and view documentation:
- **[18] Build Documentation** - Generate library docs
- **[19] Open Documentation** - Build and open in browser

### 🛠️ Utilities
Maintenance and information tasks:
- **[20] Clean All** - Remove all build artifacts
- **[21] List All Tasks** - Show all available cargo-make tasks
- **[22] Show Git Status** - View git status and recent commits
- **[23] Update Dependencies** - Run cargo update

### ⚡ Parallel Workflows
Advanced multi-window workflows:
- **[24] Dev + Docs** - Launch demo server and docs in separate windows
- **[25] Full Build** - Build library and demo in parallel

## Features

### Color-Coded Output
- 🟢 **Green** - Success messages and primary actions
- 🔵 **Cyan** - Info messages and headers
- 🟡 **Yellow** - Warnings and category headers
- 🔴 **Red** - Errors and exit option

### Smart Terminal Detection
- Uses **Windows Terminal** (wt) if available for parallel tasks
- Gracefully falls back to PowerShell if wt is not installed

### Task Timing
- Displays execution time for completed tasks
- Shows success/failure status with exit codes

### Error Handling
- Checks prerequisites before running
- Validates task success/failure
- Provides helpful error messages

## Example Workflows

### Daily Development
```powershell
# Start your day
.\launcher.ps1 -Task dev          # Launch demo
# Make changes...
.\launcher.ps1 -Task check        # Quick validation
.\launcher.ps1 -Task fix          # Fix issues before commit
```

### Before Committing
```powershell
.\launcher.ps1
# Select option [3] - Fix & Verify
# This will:
#   1. Format all code
#   2. Auto-fix clippy issues
#   3. Build the library
#   4. Run all tests
#   5. Verify no clippy warnings
```

### Full Development Session
```powershell
.\launcher.ps1
# Select option [24] - Dev + Docs (Parallel)
# Opens two windows:
#   - Demo server on http://127.0.0.1:3000
#   - Library documentation
```

### CI Simulation
```powershell
.\launcher.ps1 -Task check        # Or select [13] in menu
# Runs the same checks as CI:
#   - Format check
#   - Build
#   - Tests
#   - Clippy (warnings as errors)
```

## Tips

1. **Use Direct Tasks for CI/CD**: The `-Task` parameter is perfect for automation
   ```powershell
   .\launcher.ps1 -Task fix && git commit -m "Fixed issues"
   ```

2. **Parallel Development**: Use option [24] to run demo and docs simultaneously
   ```powershell
   # Opens separate windows for demo server and documentation
   ```

3. **Pre-Commit Hook**: Add to `.git/hooks/pre-commit`:
   ```bash
   #!/bin/sh
   pwsh -File launcher.ps1 -Task check
   ```

4. **Quick Status Check**: Option [22] shows git status + last 5 commits

## Troubleshooting

### "Missing prerequisites" Error
Install the missing tools:
```powershell
# Install Rust (if needed)
# Visit: https://rustup.rs/

# Install cargo-make
cargo install cargo-make

# Install trunk
cargo install trunk
```

### Demo Server Won't Start
1. Check if port 3000 is already in use
2. Make sure you're in the project root directory
3. Verify `demo/` directory exists

### Clippy Fails
This is intentional - clippy is configured to treat warnings as errors.
Use option [11] to auto-fix issues, or option [3] for fix + verify.

## Integration with cargo-make

The launcher is a wrapper around `cargo-make` tasks. You can:
- Run any cargo-make task directly: `cargo make <task-name>`
- See all available tasks: `cargo make --list-all-steps`
- Use launcher for convenience and better UX

## Windows Terminal Integration

For the best experience, install Windows Terminal:
```powershell
winget install Microsoft.WindowsTerminal
```

Benefits:
- Better colors and fonts
- Tabs for multiple sessions
- Split panes support
- Used automatically by parallel workflows ([24], [25])

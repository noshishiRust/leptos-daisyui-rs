# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust crate providing type-safe, reactive daisyUI 5 component wrappers for Leptos framework. The library wraps daisyUI components as Leptos components with proper type safety, leveraging Leptos's spread attributes functionality. Currently designed for CSR (Client-Side Rendering).

**Component Coverage: 62/62 (100%)**

## Build Commands

### Using the Launcher Script (Easiest)

The project includes a comprehensive PowerShell launcher with an interactive menu:

```powershell
# Interactive menu
.\launcher.ps1

# Direct task execution
.\launcher.ps1 -Task dev      # Launch demo app
.\launcher.ps1 -Task check    # Quick validation
.\launcher.ps1 -Task fix      # Fix and verify
.\launcher.ps1 -Task build    # Build library
.\launcher.ps1 -Task test     # Run tests
.\launcher.ps1 -Task docs     # Build and open docs
```

**Features:**
- 🎨 Color-coded output for easy reading
- ⚡ Parallel workflows (dev + docs in separate windows)
- ✅ Prerequisite checking
- 📊 Task timing and success/failure reporting
- 🔍 25+ pre-configured tasks organized by category

See [`LAUNCHER.md`](./LAUNCHER.md) for complete guide.

### Using cargo-make (Recommended for CI/Scripts)

The project includes a comprehensive `Makefile.toml` with automated workflows:

```bash
# Quick verification before commit
cargo make quick-check        # Format + clippy-fix + test

# Fix all auto-fixable issues
cargo make fix-all           # Run rustfmt and clippy --fix

# Verify everything passes
cargo make verify-all        # Format check + build + test + clippy

# Fix then verify
cargo make fix-and-verify    # Fix all issues, then verify

# CI workflow
cargo make ci                # Full CI: format + build + test + lint

# Demo development
cargo make dev               # Start demo dev server (alias for demo-serve)
cargo make demo-serve        # Run trunk serve in demo/
cargo make demo-build        # Build demo for production

# Documentation
cargo make doc               # Build library documentation
cargo make doc-open          # Build and open docs in browser

# Clean everything
cargo make clean-all         # Clean library and demo artifacts
```

**Available cargo-make tasks**: Run `cargo make --list-all-steps` to see all available tasks.

### Library Development (Direct cargo commands)
```bash
# Build the library
cargo build

# Run tests
cargo test

# Check without building
cargo check

# Build with release optimizations
cargo build --release
```

### Demo Application
```bash
cd demo

# Development server with hot-reload (Trunk watches ../src and ./src automatically)
trunk serve
# Runs on http://127.0.0.1:3000
# Or use: cargo make dev (from root)

# Build for production
trunk build --release
```

The demo automatically:
- Runs Tailwind CSS compilation via pre_build hook: `npx tailwindcss -i input.css -o output.css`
- Watches both `../src` (library) and `./src` (demo) for changes
- Showcases all 62 daisyUI components with interactive examples

## Architecture

### Core Structure

The crate has two main modules:
- `src/components/` - daisyUI component wrappers (62/62 components - 100% coverage!)
- `src/utils/` - Utility code including `ClassAttributes` for dynamic class management

### Newly Implemented Components (2026-01-28)
- `calendar/` - Styling wrapper for Cally, Pikaday, and React Day Picker
- `fab/` - Floating Action Button with speed dial (flower/vertical layouts)
- `hover_3d/` - 3D tilt effect on mouse hover
- `hover_gallery/` - Interactive image gallery with horizontal hover reveal
- `text_rotate/` - Animated text rotation (up to 6 items)
- `tooltip/` - Contextual hover messages with positioning and colors

### Component Pattern

All components follow a consistent wrapper pattern:

1. **Type-Safe Props**: Each component has enums for styling options (e.g., `ButtonColor`, `ButtonSize`, `ButtonShape`, `ButtonStyle`) that map to daisyUI CSS classes
2. **Spread Attributes**: Components accept Leptos spread attributes (`attr:`, `class:`, `style:`, `on:`, `prop:`) to extend underlying HTML elements
3. **Signal-Based Reactivity**: Props accept `Signal<T>` for reactive updates
4. **NodeRef Support**: Components expose `node_ref` prop for direct DOM access

Example component structure:
```
components/
└── button/
    ├── mod.rs           # Public exports
    ├── component.rs     # Button component implementation
    └── style.rs         # ButtonColor, ButtonSize, etc. enums
```

### Class Management

The `utils/class_attribute.rs` module provides `ClassAttributes` for building dynamic class strings:
- `ClassAttribute::Static(&'static str)` - Compile-time class names
- `ClassAttribute::Dynamic(String)` - Runtime class names
- `merge_classes!` macro - Combines base classes with user-provided classes

This ensures daisyUI classes from style enums properly merge with user-provided `class` prop.

## CSS Configuration

daisyUI class names must be explicitly included in Tailwind CSS input for proper compilation. Each component documents required classes:

```css
/* In demo/input.css or your project's input.css */
@import "tailwindcss";
@plugin "daisyui";
@source "../src/**/*.rs";

/* Example: Button component classes */
@source inline("btn btn-neutral btn-primary ... btn-circle");
```

See `demo/input.css` for the complete list or `stytles/daisyui-components.css` to include everything.

## Component Guidelines

When adding or modifying components:

1. **Style Enums**: Define enums for all daisyUI variants (color, size, style) with `as_str()` method returning CSS class
2. **Props**: Use `#[prop(optional, into)]` for optional reactive props that accept `Signal<T>`
3. **Spread Attributes**: Always include `#[prop(attrs)] attributes: Vec<(&'static str, Attribute)>` and spread onto root element
4. **Documentation**: Include:
   - Component-level doc comment with usage example
   - CSS classes needed in `input.css` via `@source inline(...)`
   - Node reference documentation with MDN link

5. **Module Structure**:
   ```rust
   // component.rs
   #[component]
   pub fn ComponentName(...) -> impl IntoView { ... }

   // style.rs
   #[derive(Clone, Debug, Default)]
   pub enum ComponentColor { ... }

   // mod.rs
   pub use component::*;
   pub use style::*;
   ```

## Development Notes

- **Edition**: Uses Rust Edition 2024
- **Leptos Version**: 0.8 with CSR features
- **daisyUI Version**: Targets daisyUI 5
- **Tailwind CSS**: Version 4 compatibility

### Development Dependencies

**table-rs (Temporary Fork)**: This project currently uses a local fork of table-rs with security and stability fixes.

- **Location**: `../table-rs` (path dependency)
- **Reason**: Applying security/stability fixes and enhancements pending upstream contribution
- **Status**: Temporary - will submit PRs to upstream table-rs project
- **Long-term Goal**: Switch back to official crates.io version once PRs are merged

**Setup Requirements**:
- Clone the table-rs fork to `C:\Development\table-rs` (sibling directory)
- The fork includes security fixes, stability improvements, and functionality enhancements
- Changes in table-rs will be immediately reflected when rebuilding leptos-daisyui-rs

## Known Limitations

- Currently assumes CSR usage only
- CSS classes must be manually added to `input.css` (no automatic purge-safe class inclusion yet)
- Some components cannot be used with alternative HTML elements (e.g., button styles on `<a>` tags) - workaround is creating wrapper components that add classes to children

## Documentation

Component documentation lives in `doc/components/` with markdown files for major components. Reference the daisyUI docs at https://daisyui.com/components/ for design specifications.

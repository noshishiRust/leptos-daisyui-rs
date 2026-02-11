# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust library providing type-safe, reactive wrappers for daisyUI 5 components for the Leptos web framework. Currently implements all 62 daisyUI components with 100% coverage. Primarily designed for CSR (Client-Side Rendering) with WASM target.

## Development Commands

### Main Library

```bash
# Format code
cargo fmt

# Format with leptosfmt (required for CI - specific Leptos formatter)
leptosfmt .

# Check formatting
leptosfmt --check .

# Lint with clippy
cargo clippy -- -D warnings
```

### Demo Application

```bash
cd demo

# Install Node.js dependencies (required for Tailwind CSS build)
npm ci

# Development server (Trunk build tool)
trunk serve

# Production build
trunk build --release
```

## Component Architecture

Each component follows a consistent structure:

```
src/components/{component_name}/
├── mod.rs           # Module exports
├── component.rs     # Component implementation
└── style.rs         # Style enums (Color, Size, Shape, etc.)
```

### Component Implementation Pattern

1. **Props**: All props use `Signal<T>` for reactivity with `#[prop(optional, into)]`
2. **Style Enums**: Implement `as_str()` method returning CSS class strings
3. **Class Merging**: Use `merge_classes!` macro with `ClassAttributes` utility
4. **Spread Attributes**: Full support for Leptos spread attributes (`attr:`, `class:`, `style:`, `on:`, `prop:`)
5. **Node References**: Components expose `NodeRef<T>` for DOM access

Example from Button component (src/components/button/component.rs):
```rust
#[component]
pub fn Button(
    #[prop(optional, into)] color: Signal<ButtonColor>,
    #[prop(optional, into)] node_ref: NodeRef<HTMLButton>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            node_ref=node_ref
            class=move || {
                merge_classes!("btn", color.get().as_str(), class)
            }
        >
            {children()}
        </button>
    }
}
```

### Style Enum Pattern

All style enums in `style.rs` follow this pattern:

```rust
#[derive(Clone, Debug, Default)]
pub enum ComponentColor {
    #[default]
    Default,
    // ... variants
}

impl ComponentColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentColor::Default => "",
            ComponentColor::Neutral => "component-neutral",
            // ...
        }
    }
}
```

## CSS Integration

The library uses Tailwind CSS v4 with daisyUI plugin. CSS classes are inlined using `@source` directives:

```css
@import "tailwindcss";
@plugin "daisyui";
@source "../src/**/*.rs";

/* Include specific component classes */
@source inline("btn btn-neutral btn-primary ...");
```

All daisyUI classes are available in `stytles/daisyui-components.css` for full inclusion.

## Documentation System

Component documentation in `doc/components/` follows a structured format:
- Props tables with types and defaults
- Style variants and enums
- Collapsible code examples (`<details>`/`<summary>`)
- Accessibility guidelines
- Best practices

## Key Dependencies

- **leptos 0.8**: Core web framework
- **Rust 2024 edition**

## CI/CD

GitHub Actions workflow (`.github/workflows/rust.yml`):
- Installs `leptosfmt` via `cargo install leptosfmt`
- Runs `leptosfmt --check .` for formatting
- Runs `cargo clippy -- -D warnings` for linting

Vercel deployment configured via `.github/workflows/depoly.yml`.

## Adding New Components

When adding a new daisyUI component:

1. Create directory: `src/components/{component_name}/`
2. Implement `mod.rs`, `component.rs`, `style.rs` following existing patterns
3. Export from `src/components/mod.rs`
4. Add CSS classes to documentation comment
5. Add markdown documentation in `doc/components/{component_name}.md`
6. Run `leptosfmt .` to format
7. Update `README.md` implementation table

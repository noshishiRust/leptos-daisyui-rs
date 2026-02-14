# daisyUI Leptos Component Rules

This file defines the rules and best practices for implementing daisyUI components in Leptos.

## Component Structure Rules

### Directory Structure

Every component MUST follow this structure:

```
src/components/{component_name}/
├── mod.rs           # Module exports
├── component.rs     # Component implementation
└── style.rs         # Style enums (Color, Size, Shape, etc.)
```

### File Requirements

**mod.rs** MUST:
- Start with module-level documentation: `//! # daisyUI {ComponentName} Components`
- Include reference to official docs: `//! For more information, see: https://daisyui.com/components/{component_name}/`
- Import both `component` and `style` modules
- Re-export all public items from both modules

**style.rs** MUST:
- Create separate enums for different style categories (Color, Size, Shape, Style, etc.)
- ALL style enums MUST implement: `Clone`, `Debug`, and `Default`
- The `Default` variant MUST return `""` from `as_str()`
- All other variants MUST return their CSS class string (e.g., `"btn-primary"`)
- Each enum MUST have a `pub fn as_str(&self) -> &'static str` implementation
- Include rustdoc comments explaining the enum's purpose

**component.rs** MUST:
- Use `use crate::merge_classes;` for class merging
- Import style enums from `super::style::*`
- Include rustdoc with:
  - Component description
  - CSS `@source inline()` directive with all class names
  - Node References section linking to MDN documentation

## Props Rules

### Reactive Props

- ALL style props MUST use `Signal<T>` for reactivity
- MUST include `#[prop(optional, into)]` attribute
- Example: `#[prop(optional, into)] color: Signal<ButtonColor>`

### Required Props

Every component MUST include:
- `class: &'static str` - Additional CSS classes (with `#[prop(optional, into)]`)
- `node_ref: NodeRef<{HtmlElementType}>` - DOM element reference (with `#[prop(optional, into)]`)
- `children: Children` - For components with content

### Props Order

Props should be ordered logically:
1. Behavior props (loading, disabled, active, etc.)
2. Style props (color, size, shape, style, etc.)
3. Node references
4. Additional customization (class, id, etc.)
5. Content (children)

## Class Merging Rules

- MUST use `merge_classes!` macro for all class merging
- Base class goes first
- Style classes follow using `signal.get().as_str()`
- Custom `class` prop goes last
- Example:

```rust
class=move || {
    merge_classes!(
        "btn",
        color.get().as_str(),
        size.get().as_str(),
        class
    )
}
```

## Conditional Classes

For boolean props that add classes conditionally, use Leptos's `class:` syntax:

```rust
class:btn-active=active
class:btn-disabled=disabled
class:loading=loading
```

## Style Enum Patterns

### Naming Convention

- `{Component}{Category}` - e.g., `ButtonColor`, `ButtonSize`, `ButtonShape`
- Use PascalCase for enum names and variants

### Default Variant

ALL enums MUST have a `Default` variant that:
- Is marked with `#[default]`
- Returns empty string `""` from `as_str()`

### Example Pattern

```rust
#[derive(Clone, Debug, Default)]
pub enum ButtonColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    // ... more variants
}

impl ButtonColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonColor::Default => "",
            ButtonColor::Neutral => "btn-neutral",
            ButtonColor::Primary => "btn-primary",
            ButtonColor::Secondary => "btn-secondary",
            // ... more variants
        }
    }
}
```

## Documentation Rules

### Rustdoc Requirements

EVERY component MUST include:
1. Short description of what the component does
2. CSS `@source inline()` directive in a code block
3. "## Node References" section with MDN links

### Example Documentation

```rust
/// # Button Component
///
/// A reactive wrapper for daisyUI's button component with comprehensive styling options
/// including colors, sizes, shapes, and interactive states.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("btn btn-neutral btn-primary ...");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<button>` element ([HTMLButtonElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement))
```

## Type Safety Rules

- ALL style variants MUST use enums, NOT strings
- This provides compile-time type safety
- Enables IDE autocomplete
- Catches typos at compile time

## HTML Element Selection

Choose the most semantically appropriate HTML element:
- `<button>` for actions
- `<a>` for navigation
- `<input>` for form inputs
- `<div>` for layout containers
- etc.

## Module Export Rules

After creating a component, MUST update `src/components/mod.rs`:

```rust
mod {component_name};

pub use {component_name}::*;
```

## Formatting Requirements

After creating/modifying components, ALWAYS run:

```bash
# Format with Leptos-specific formatter (REQUIRED for CI)
leptosfmt .

# Lint with clippy
cargo clippy -- -D warnings
```

## Reference Implementation

Use `src/components/button/` as the canonical example for:
- Component structure
- Style enum patterns
- Documentation style
- Props definition
- Class merging approach

## CSS Class Extraction from Style Guide

When reading `.daisyui/components/{component_name}.md`, extract:

1. **Base component class**: The main class name (e.g., `btn`)
2. **Color variants**: Usually follow pattern `{component}-{color}` (e.g., `btn-primary`, `btn-success`)
3. **Size variants**: Usually follow pattern `{component}-{size}` (e.g., `btn-xs`, `btn-sm`, `btn-lg`)
4. **Style/Modifier classes**: Visual treatments (e.g., `btn-outline`, `btn-ghost`)

Then create corresponding:
- Base HTML element
- Style enums for each category
- Props for each enum type
- CSS `@source inline()` directive with ALL classes

## Spread Attributes Support

Components SHOULD support Leptos spread attributes where applicable:
- `attr:` - HTML attributes
- `class:` - Additional classes
- `style:` - Inline styles
- `on:` - Event handlers
- `prop:` - Element properties

This allows users to add any custom behavior not covered by the component's API.

# Rust Coding Rules for leptos-daisyui-rs

This file defines Rust-specific coding standards and patterns for the leptos-daisyui-rs project.

## Project Context

- **Language**: Rust 2024 edition
- **Framework**: Leptos 0.8
- **Target**: WASM (WebAssembly) for CSR (Client-Side Rendering)
- **Purpose**: Type-safe, reactive wrappers for daisyUI 5 components

## Code Quality Standards

### Formatting

Two formatters are used:

1. **Standard Rust formatting**:
   ```bash
   cargo fmt
   ```

2. **Leptos-specific formatter** (REQUIRED for CI):
   ```bash
   leptosfmt .
   leptosfmt --check .  # CI check
   ```
   - This is REQUIRED by `.github/workflows/rust.yml`
   - Must pass before committing

### Linting

```bash
cargo clippy -- -D warnings
```

- CI runs with `-D warnings` (treat warnings as errors)
- ALL clippy warnings must be addressed before committing

## Leptos-Specific Patterns

### Component Definition

All components follow this pattern:

```rust
use leptos::prelude::*;

#[component]
pub fn ComponentName(
    // Props here
) -> impl IntoView {
    view! {
        // JSX-like syntax
    }
}
```

### Props Declaration Rules

1. **Use `#[prop(optional, into)]` for reactive props**:
   ```rust
   #[prop(optional, into)]
   color: Signal<ButtonColor>,
   ```

2. **Use `#[prop(optional)]` for optional non-reactive props**:
   ```rust
   #[prop(optional)]
   name: Option<&'static str>,
   ```

3. **Required props have no attributes**:
   ```rust
   children: Children,  // Required, no attribute
   ```

### Signal Types

- **ALL style-related props MUST use `Signal<T>`** for reactivity
- `Signal<T>` is a reactive getter that can be called multiple times
- Use `.get()` to access the current value

```rust
#[prop(optional, into)]
color: Signal<ButtonColor>,

// Usage in view!:
class=move || {
    merge_classes!("btn", color.get().as_str(), class)
}
```

### Reactive Closures

- Use `move ||` closures in `view!` for reactive updates
- The closure captures signals and re-computes when signals change

```rust
class=move || {  // Reactive - recomputes when signals change
    merge_classes!(
        "btn",
        color.get().as_str(),  // .get() reads current value
        class
    )
}
```

### Node References

Components MUST provide `NodeRef<T>` for DOM access:

```rust
#[prop(optional, into)]
node_ref: NodeRef<HTMLButton>,

// In view!:
<button node_ref=node_ref />
```

**Rules**:
- Use `#[prop(optional, into)]` for all node_ref props
- Type parameter should match the HTML element type
- Document the element type with MDN link in rustdoc

### HTML Element Types

Import specific HTML element types from `leptos::html`:

```rust
use leptos::{
    html::{Div, Button, Input, A},
    prelude::*,
};
```

Common types:
- `Div` - `<div>` elements
- `Button` - `<button>` elements
- `Input` - `<input>` elements
- `A` - `<a>` anchor elements
- `Span` - `<span>` elements

## Macro Usage

### merge_classes! Macro

Custom macro defined in `src/utils/class_attribute.rs`:

```rust
use crate::merge_classes;

class=move || {
    merge_classes!(
        "base-class",
        style.get().as_str(),
        color.get().as_str(),
        class
    )
}
```

**Rules**:
- Base class goes first
- Style signals follow with `.get().as_str()`
- Custom `class` prop goes last
- Handles empty strings automatically

### view! Macro

Leptos's JSX-like syntax:

```rust
view! {
    <div class="my-class">
        {children()}
    </div>
}
```

**Rules**:
- Use curly braces `{variable}` to embed Rust expressions
- Use `move ||` closures for reactivity
- Use `class:name=value` syntax for conditional classes
- Use `attr:key=value` for HTML attributes
- Use `on:event=handler` for event listeners

## Type Safety Patterns

### Enums for Style Variants

NEVER use strings for style variants:

```rust
// ❌ BAD
fn set_color(color: &str) { ... }

// ✅ GOOD
fn set_color(color: ButtonColor) { ... }
```

Benefits:
- Compile-time type checking
- IDE autocomplete
- Typos caught at compile time
- Easy refactoring

### as_str() Pattern

All style enums implement `as_str()`:

```rust
pub enum ButtonColor {
    #[default]
    Default,
    Primary,
}

impl ButtonColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonColor::Default => "",
            ButtonColor::Primary => "btn-primary",
        }
    }
}
```

## Imports Organization

### Standard Order

```rust
// 1. Local imports (super, crate)
use super::style::{ButtonColor, ButtonSize};
use crate::merge_classes;

// 2. Leptos HTML element types
use leptos::{
    html::{Div, Button},
    prelude::*,
};
```

## Documentation Standards

### Rustdoc Comments

ALL public items must have rustdoc comments:

```rust
/// # Component Name
///
/// A brief description of what this component does.
///
/// Additional details about usage, behavior, or important notes.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("class-names-here");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<element>` ([ElementType](https://developer.mozilla.org/en-US/docs/Web/API/ElementType))
#[component]
pub fn ComponentName(...) -> impl IntoView {
    // ...
}
```

### Props Documentation

Each prop should be documented:

```rust
/// Component color variant
#[prop(optional, into)]
color: Signal<ButtonColor>,

/// Node reference for the button element
#[prop(optional, into)]
node_ref: NodeRef<HTMLButton>,
```

## Module Structure

### File Organization

```
src/
├── lib.rs              # Library root
├── components/
│   ├── mod.rs          # Component module exports
│   └── {component_name}/
│       ├── mod.rs       # Component module (exports + imports)
│       ├── component.rs # Component implementations
│       └── style.rs    # Style enums
└── utils/
    ├── mod.rs          # Utility module exports
    └── class_attribute.rs  # Class merging utilities
```

### Module Exports

`mod.rs` files should:
1. Import submodules with `mod`
2. Re-export public items with `pub use`

```rust
mod component;
mod style;

pub use component::*;
pub use style::*;
```

## CSS Integration

### @source Directives

Components document their CSS classes:

```rust
/// ### Add to `input.css`
/// ```css
/// @source inline("btn btn-primary btn-secondary ...");
/// ```
```

Users add these to their CSS:

```css
@import "tailwindcss";
@plugin "daisyui";
@source "../src/**/*.rs";

/* Button */
@source inline("btn btn-neutral btn-primary btn-secondary btn-accent btn-info btn-success btn-warning btn-error btn-outline btn-dash btn-soft btn-ghost btn-link btn-xs btn-sm btn-md btn-lg btn-xl btn-wide btn-block btn-square btn-circle btn-active btn-disabled loading");
```

## Spread Attributes Support

Components MUST support Leptos spread attributes:

```rust
// User can add:
attr:data-id="my-button"      // HTML attributes
class:hover=true              // Conditional classes
style:color="red"             // Inline styles
on:click=move |_| { ... }    // Event listeners
prop:disabled=true             // Element properties
```

This is automatic with Leptos - no extra code needed. Just document it.

## Error Handling

- Prefer `Option<T>` over `Result<T, E>` for missing values
- Use `#[prop(optional)]` for optional props
- Avoid `unwrap()` and `expect()` - use pattern matching or `?` operator

## Performance Considerations

### Signal Granularity

- Use `Signal<T>` for reactive props (fine-grained reactivity)
- Avoid `Signal<Struct>` - prefer multiple signals
- This minimizes re-renders to only affected components

### Clone Strategy

- Signals are cheap to clone (use `.clone()` freely)
- `NodeRef` is cheap to clone
- Large data structures: consider `Arc<T>` or `Rc<T>`

## Testing Patterns

Currently no tests defined, but when added:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_component_renders() {
        // Test implementation
    }
}
```

## Common Pitfalls to Avoid

### ❌ Don't use strings for styles

```rust
// BAD
color: &'static str,  // "primary", "secondary", etc.

// GOOD
color: Signal<ButtonColor>,  // Type-safe!
```

### ❌ Don't forget move closures

```rust
// BAD - won't react to signal changes
class=|| merge_classes!("btn", color.get().as_str())

// GOOD - reactive
class=move || merge_classes!("btn", color.get().as_str())
```

### ❌ Don't use class prop for variants

```rust
// BAD
<Button class="btn-primary" />

// GOOD
<Button color=ButtonColor::Primary />
```

## Dependencies

From `Cargo.toml`:
- `leptos = { version = "0.8" }` - Core framework
- No other runtime dependencies

## WASM Target Considerations

- Code compiles to `wasm32-unknown-unknown`
- CSR-only (Client-Side Rendering)
- No SSR (Server-Side Rendering) support currently
- All code must be WASM-compatible (no std::fs, std::net, etc.)

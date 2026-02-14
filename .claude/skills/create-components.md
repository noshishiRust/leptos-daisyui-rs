---
name: create-components
description: Creates Leptos component implementations from daisyUI style guide documentation
---

This skill creates Leptos components based on the daisyUI style guide in `.daisyui` directory.

## Prerequisites

1. **Check for `.daisyui` directory**: Verify that the `.daisyui` directory exists in the project root.
   - If it does NOT exist, recommend the user runs the `install-component-guide` skill first:
     - "The `.daisyui` style guide directory is not found. Please run the `install-component-guide` skill first to fetch and set up the daisyUI documentation."

2. **Read the rules**: Review `.claude/rules/components.md` for detailed rules and best practices

3. **Read the style guide**: Reference the relevant component documentation from `.daisyui/components/{component_name}.md`

## Implementation Steps

### 1. Create Directory Structure

```bash
mkdir -p src/components/{component_name}
```

### 2. Create mod.rs

```rust
//! # daisyUI {ComponentName} Components
//!
//! For more information, see: https://daisyui.com/components/{component_name}/

mod component;
mod style;

pub use component::*;
pub use style::*;
```

### 3. Create style.rs

Extract class names from `.daisyui/components/{component_name}.md` and create style enums:

```rust
/// # {ComponentName} Color Variants
#[derive(Clone, Debug, Default)]
pub enum {ComponentName}Color {
    #[default]
    Default,
    // Add variants based on .daisyui documentation
}

impl {ComponentName}Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            {ComponentName}Color::Default => "",
            // Map variants to CSS class strings
        }
    }
}
```

**Refer to `.claude/rules/components.md` for complete style enum rules**

### 4. Create component.rs

```rust
use super::style::{ComponentColor, ComponentSize};
use crate::merge_classes;
use leptos::prelude::*;

/// # {ComponentName} Component
///
/// A reactive wrapper for daisyUI's {component_name} component.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("{class_names}");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<{html_element}>` element
#[component]
pub fn {ComponentName}(
    #[prop(optional, into)]
    color: Signal<ComponentColor>,

    #[prop(optional, into)]
    node_ref: NodeRef<{HtmlElementType}>,

    #[prop(optional, into)]
    class: &'static str,

    children: Children,
) -> impl IntoView {
    view! {
        <{html_element}
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "{base_class}",
                    color.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </{html_element}>
    }
}
```

**Refer to `.claude/rules/components.md` for props rules, class merging, and documentation requirements**

### 5. Update src/components/mod.rs

```rust
mod {component_name};
pub use {component_name}::*;
```

### 6. Format and Validate

```bash
leptosfmt .
cargo clippy -- -D warnings
```

### 7. Add CSS Classes

Add the component's CSS classes using `@source inline()` in the appropriate CSS file:

```css
@source inline("component-name component-name-color1 component-name-color2 ...");
```

## Quick Reference

**Component**: `src/components/button/` - Canonical example implementation
**Rules**: `.claude/rules/components.md` - Complete rules and best practices
**Style Guide**: `.daisyui/components/{component_name}.md` - daisyUI documentation

## Example Workflow

1. Read style guide: `cat .daisyui/components/alert.md`
2. Extract: base class, color variants, size variants, modifier classes
3. Create style enums for each category
4. Implement component following the rules
5. Update module exports
6. Format and validate
7. Add CSS classes

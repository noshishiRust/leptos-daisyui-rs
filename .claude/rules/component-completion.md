# Component Completion Checklist

Use this checklist when creating or modifying components.

## File Structure

For each component, verify:

```
src/components/{component_name}/
├── mod.rs           # Required
├── component.rs     # Required
└── style.rs         # Required
```

## Module Exports

Verify `src/components/mod.rs` includes:

```rust
mod {component_name};
pub use {component_name}::*;
```

**Test**: `cargo build` will fail if module not declared

## Style Enum Completeness

Check `.daisyui/components/{component_name}.md` for reference:

Required for each style category:
- [ ] Enum defined with all variants from documentation
- [ ] `#[default]` attribute on one variant
- [ ] `Default` variant returns `""` from `as_str()`
- [ ] All other variants return CSS class strings

Example:

```rust
#[derive(Clone, Debug, Default)]
pub enum ComponentColor {
    #[default]
    Default,  // Returns ""
    Neutral,   // Returns "component-neutral"
    Primary,   // Returns "component-primary"
}
```

## Component Props

Each component must have:

Required props:
- [ ] `class: &'static str` with `#[prop(optional, into)]`
- [ ] `node_ref: NodeRef<T>` with `#[prop(optional, into)]`
- [ ] `children: Children` (for components with content)

Style props (use `Signal<T>`):
- [ ] All style props use `Signal<EnumType>`
- [ ] All have `#[prop(optional, into)]`

## Documentation

Each component must include:

### Component Documentation

```rust
/// # Component Name
///
/// Brief description of purpose and usage.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("all css classes here");
/// ```
///
/// ## Node References
/// - `node_ref` - References element type ([ElementType](MDN link))
```

Required sections:
- [ ] Description paragraph
- [ ] CSS `@source inline()` directive
- [ ] Node References with MDN links

### Props Documentation

Each prop should have a comment:

```rust
/// Component color variant
#[prop(optional, into)]
color: Signal<ButtonColor>,
```

## CSS Classes

Verify component documentation lists ALL daisyUI classes:

```bash
# Reference the daisyUI doc
cat .daisyui/components/{component_name}.md
```

Include in rustdoc:
- Base component class (e.g., `btn`)
- All color variants (e.g., `btn-primary`, `btn-success`)
- All size variants (e.g., `btn-xs`, `btn-lg`)
- All modifiers (e.g., `btn-outline`, `btn-ghost`)

## Spread Attributes Support

Components should support Leptos spread attributes:

Documentation should mention:
- `attr:` - HTML attributes
- `class:` - Conditional classes
- `style:` - Inline styles
- `on:` - Event listeners
- `prop:` - Element properties

This is automatic with Leptos - no extra code needed.

## Component Sub-Parts

For components with multiple parts (e.g., Accordion):

- [ ] Each sub-component documented
- [ ] Each sub-component has `node_ref` prop
- [ ] Each sub-component has `class` prop
- [ ] Logical naming (e.g., `AccordionTitle`, `AccordionContent`)

## Quick Verification

```bash
# Check files exist
ls src/components/{component_name}/

# Verify module exports
grep "{component_name}" src/components/mod.rs

# Build check
cargo build

# Format check
leptosfmt --check .
```

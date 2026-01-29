# RadialProgress component fails to compile - Invalid style attribute syntax

## Severity
🔴 **CRITICAL** - Library does not compile

## Description
The `RadialProgress` component uses invalid Leptos syntax for setting CSS custom properties, causing compilation failure.

## Error
```
error[E0277]: the trait bound `(&str, {closure...}): IntoStyle` is not satisfied
  --> src/components/radial_progress/component.rs:57:19
```

## Problem Code
`src/components/radial_progress/component.rs` lines 56-57:

```rust
style=("--value", move || value.get().to_string())
style=("--thickness", move || thickness.get())
```

This syntax is invalid in Leptos 0.8. Multiple `style=` attributes cannot be used, and the tuple format is incorrect.

## Solution
Use the `style:` directive for CSS custom properties:

```rust
style:--value=move || value.get().to_string()
style:--thickness=move || thickness.get()
```

## Impact
- ❌ Library cannot be compiled
- ❌ `cargo build` fails
- ❌ Cannot use the library at all
- ❌ Blocks all development

## To Reproduce
```bash
git clone https://github.com/noshishiRust/leptos-daisyui-rs.git
cd leptos-daisyui-rs
cargo build
```

**Expected:** Clean build
**Actual:** Compilation error in `radial_progress/component.rs`

## Environment
- Leptos: 0.8.x
- Rust: 2024 edition

## Fix Available
I have the fix ready and can submit a PR immediately if desired.

## Priority
This should be the highest priority fix as it's a **blocking issue** that prevents anyone from using the library.

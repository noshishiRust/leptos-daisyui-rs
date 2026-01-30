# Accordion Component Fixes

**Date**: 2026-01-29
**Issues Fixed**: leptos-daisyui-rs-75e, leptos-daisyui-rs-egz
**Priority**: P1 (High)
**Status**: ✅ RESOLVED

## Issues

### Issue leptos-daisyui-rs-75e: Accordion does not close when clicking up-carat

**Problem**: When an accordion item was open, clicking on it again did not close it. The carat icon would rotate but the content remained visible.

**Root Cause**: The accordion component was using radio button inputs (`<input type="radio">`). By HTML specification, radio buttons cannot be unchecked by clicking them again - they can only be changed by selecting another radio button in the same group.

**Solution**: Added support for both radio and checkbox input types:
- Radio: Traditional accordion behavior - only one item open at a time, cannot be closed by clicking again
- Checkbox: Toggle behavior - can be opened and closed independently

### Issue leptos-daisyui-rs-egz: Plus indicator shows wrong symbol

**Problem**: The plus/minus indicator was displaying incorrect symbols when the accordion was open.

**Root Cause**: Typo in `src/components/accordion/style.rs` line 48: `"collapse-opem"` instead of `"collapse-open"`

**Solution**: Fixed typo to use correct daisyUI CSS class `"collapse-open"`

## Changes Made

### 1. Added `AccordionInputType` Enum (`src/components/accordion/style.rs`)

```rust
/// # Accordion Input Type
///
/// Controls the type of input used for accordion toggle behavior:
/// - Radio: Only one accordion can be open at a time in a group (cannot be closed by clicking again)
/// - Checkbox: Accordion can be toggled open/closed independently (can be closed by clicking again)
#[derive(Clone, Debug, Default)]
pub enum AccordionInputType {
    /// Radio button - only one can be open at a time, cannot be closed by clicking again
    #[default]
    Radio,

    /// Checkbox - can be toggled open/closed, allows multiple open accordions
    Checkbox,
}

impl AccordionInputType {
    /// Returns the HTML input type string
    pub fn as_str(&self) -> &'static str {
        match self {
            AccordionInputType::Radio => "radio",
            AccordionInputType::Checkbox => "checkbox",
        }
    }
}
```

### 2. Fixed Typo in `AccordionForceModifier`

**Before**:
```rust
AccordionForceModifier::Open => "collapse-opem",
```

**After**:
```rust
AccordionForceModifier::Open => "collapse-open",
```

### 3. Updated Accordion Component (`src/components/accordion/component.rs`)

Added new `input_type` prop:
```rust
#[component]
pub fn Accordion(
    /// Input type controlling accordion behavior
    #[prop(optional, into)]
    input_type: Signal<AccordionInputType>,

    // ... other props
) -> impl IntoView {
    view! {
        <div class=move || { /* ... */ }>
            <input
                type=move || input_type.get().as_str()
                name=name
                checked=checked
            />
            {children()}
        </div>
    }
}
```

### 4. Updated Demo (`demo/src/demos/accordion.rs`)

Added three new sections showcasing both behaviors:

1. **Radio Accordion** - Traditional behavior (only one open at a time)
2. **Checkbox Collapse** - Toggle behavior (can close by clicking again)
3. **Icon Modifiers** - Demonstrates all visual indicators

## Files Modified

1. `src/components/accordion/style.rs`
   - Added `AccordionInputType` enum
   - Fixed typo in `AccordionForceModifier::Open`
   - Fixed typos in comments ("Force openm" → "Force open", "Force clse" → "Force close")

2. `src/components/accordion/component.rs`
   - Added import for `AccordionInputType`
   - Added `input_type` prop with default of `Radio`
   - Updated component documentation
   - Made input type dynamic using signal

3. `demo/src/demos/accordion.rs`
   - Complete rewrite of demo showcasing both input types
   - Added explanatory text for each section
   - Demonstrated all icon modifiers (Default, Arrow, Plus)

## Verification

All tests pass:
```bash
cargo make quick-check
✓ cargo fmt
✓ cargo clippy --fix
✓ cargo test
```

## Usage Examples

### Radio Accordion (Traditional)
```rust
// Only one can be open at a time
<Accordion
    input_type=AccordionInputType::Radio
    name="my-accordion"
    modifier=AccordionModifier::Arrow
>
    <AccordionTitle>"Section 1"</AccordionTitle>
    <AccordionContent><p>"Content 1"</p></AccordionContent>
</Accordion>

<Accordion
    input_type=AccordionInputType::Radio
    name="my-accordion"  // Same name = same group
    modifier=AccordionModifier::Arrow
>
    <AccordionTitle>"Section 2"</AccordionTitle>
    <AccordionContent><p>"Content 2"</p></AccordionContent>
</Accordion>
```

### Checkbox Collapse (Toggle)
```rust
// Can be toggled independently
<Accordion
    input_type=AccordionInputType::Checkbox
    modifier=AccordionModifier::Plus
>
    <AccordionTitle>"Toggle Me"</AccordionTitle>
    <AccordionContent><p>"Click again to close"</p></AccordionContent>
</Accordion>
```

## Breaking Changes

**None** - The `input_type` prop defaults to `Radio`, maintaining backward compatibility with existing code.

## Related Documentation

- [daisyUI Collapse Component](https://daisyui.com/components/collapse/)
- [daisyUI Accordion Examples](https://daisyui.com/components/accordion/)

## References

Sources consulted during fix:
- [Tailwind Accordion Component – daisyUI](https://daisyui.com/components/accordion/)
- [Tailwind Collapse Component – daisyUI](https://daisyui.com/components/collapse/)
- [Accordion to be able to fully close again · Discussion #3412](https://github.com/saadeghi/daisyui/discussions/3412)

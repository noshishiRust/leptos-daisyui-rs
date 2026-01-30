# Drawer Close Animation Fix

**Issue ID:** leptos-daisyui-rs-8jv
**Date:** 2026-01-29
**Status:** RESOLVED

## Problem Description

The Drawer component's close animation was showing at the wrong position. The drawer was not animating from its open position when closing, but instead appearing to animate from an incorrect location.

## Root Cause Analysis

The issue was caused by a mismatch between two state management systems:

1. **Visual State**: Controlled by `class:drawer-open` reactive class binding (via Signal)
2. **Animation State**: Controlled by CSS selector `.drawer-toggle:checked` (checkbox state)

The daisyUI drawer component relies on the **checkbox's `:checked` pseudo-class** to trigger CSS transitions. When the `DrawerToggle` checkbox component was not synchronized with the `open` Signal, the following occurred:

- The `Drawer` component would apply/remove the `drawer-open` class based on the Signal
- The underlying checkbox element remained in an unchecked state
- CSS transitions keyed on `:checked` state would not trigger correctly
- The close animation would start from the wrong transform position

### CSS Animation Flow

From `demo/output.css`:

```css
/* Initial state - drawer off-screen */
.drawer-side > *:not(.drawer-overlay) {
  will-change: transform;
  transition: translate 0.3s ease-out;
  translate: -100%; /* Left drawer starts off-screen left */
}

/* When checkbox is checked - drawer visible */
.drawer-toggle:checked ~ .drawer-side > *:not(.drawer-overlay) {
  translate: 0%; /* Slides to visible position */
}

/* For right-side drawer */
.drawer-end > .drawer-toggle ~ .drawer-side > *:not(.drawer-overlay) {
  translate: 100%; /* Right drawer starts off-screen right */
}
```

The animation depends on the checkbox state changing from `checked` to `unchecked` to properly transition the `translate` property.

## Solution

Added `checked` prop to `DrawerToggle` components to synchronize the checkbox DOM property with the reactive Signal:

### 1. Demo Drawer Component

**File:** `demo/src/demos/drawer.rs`

```rust
// BEFORE
<DrawerToggle
    id="drawer-demo"
    on:click=move |_| set_drawer_open.update(|open| *open = !*open)
/>

// AFTER
<DrawerToggle
    id="drawer-demo"
    checked=drawer_open  // ← Added: Syncs checkbox with Signal
    on:click=move |_| set_drawer_open.update(|open| *open = !*open)
/>
```

### 2. Main Layout Drawer

**File:** `demo/src/core/layout.rs`

```rust
// BEFORE
<Drawer open=breakpoints.ge(BreakpointsTailwind::Lg)>
    <DrawerToggle id="drawer-toggle" />

// AFTER
<Drawer open=breakpoints.ge(BreakpointsTailwind::Lg)>
    <DrawerToggle
        id="drawer-toggle"
        checked=breakpoints.ge(BreakpointsTailwind::Lg)  // ← Added
    />
```

## Technical Details

### DrawerToggle Component

The `DrawerToggle` component (in `src/components/drawer/component.rs`) uses `prop:checked` to set the checkbox's **DOM property** (not just the HTML attribute):

```rust
#[component]
pub fn DrawerToggle(
    id: &'static str,
    #[prop(optional, into)]
    checked: Signal<bool>,  // ← Accepts reactive Signal
    #[prop(optional)]
    node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            id=id
            type="checkbox"
            class="drawer-toggle"
            prop:checked=checked  // ← Sets DOM property reactively
        />
    }
}
```

The `prop:checked` directive ensures the checkbox's `.checked` property updates reactively, which triggers the CSS `:checked` pseudo-class correctly.

## Verification

Build and run commands:

```bash
# Build library
cargo build --release

# Build demo
cd demo && cargo build

# Run demo server
trunk serve
# or
cargo make dev
```

Navigate to the Drawer component demo and verify:
1. Opening drawer slides in smoothly from the correct side
2. Closing drawer slides out smoothly to the correct side
3. No position jumping or incorrect animation origins
4. Main layout drawer (responsive sidebar) also animates correctly

## Impact

- **Severity**: Medium (visual bug affecting user experience)
- **Components Affected**: Drawer, DrawerToggle
- **Files Modified**:
  - `demo/src/demos/drawer.rs`
  - `demo/src/core/layout.rs`
- **Breaking Changes**: None (backward compatible)

## Best Practices

When using the Drawer component with reactive state:

1. **Always sync the checkbox with the Signal**:
   ```rust
   let (drawer_open, set_drawer_open) = signal(false);

   <Drawer open=drawer_open>
       <DrawerToggle id="my-drawer" checked=drawer_open />
       // ...
   </Drawer>
   ```

2. **Use the same Signal for both `open` and `checked`**:
   - `open` controls the `drawer-open` class on the container
   - `checked` controls the checkbox's `:checked` state for CSS animations

3. **For responsive drawers**, use the same condition:
   ```rust
   let is_large = breakpoints.ge(BreakpointsTailwind::Lg);

   <Drawer open=is_large>
       <DrawerToggle id="drawer" checked=is_large />
   ```

## Related Documentation

- [daisyUI Drawer Documentation](https://daisyui.com/components/drawer/)
- [Leptos Spread Attributes](https://book.leptos.dev/view/05_forms.html)
- [CSS :checked Selector](https://developer.mozilla.org/en-US/docs/Web/CSS/:checked)

## Issue Resolution

✅ **RESOLVED** - Drawer animations now work correctly in both demo and main layout.

**Tested on**: Windows with Chrome DevTools
**Related Issues**: None
**Follow-up Tasks**: None

# Testing Guidelines

Testing standards and manual verification for this project.

## Current Testing Status

⚠️ **No automated tests currently exist**

All verification is manual. When adding tests, follow these patterns.

## Manual Testing Checklist

### Visual Testing

For each component, verify:

- [ ] Component renders with default props
- [ ] All color variants display correctly
- [ ] All size variants display correctly
- [ ] All style variants (outline, ghost, etc.) work
- [ ] Component appearance matches daisyUI examples
- [ ] No visual artifacts or layout issues

**Reference**: Compare with https://daisyui.com/components/{component_name}/

### Interactive Testing

- [ ] Click events fire correctly
- [ ] Hover states display as expected
- [ ] Focus states display on keyboard navigation
- [ ] Disabled state prevents interaction
- [ ] Loading state displays (when applicable)
- [ ] Reactive props update view correctly
- [ ] Multiple instances work independently

### Reactive Behavior Testing

Verify signals trigger updates:

```rust
#[component]
fn TestReactiveProps() -> impl IntoView {
    let (color, set_color) = signal(ButtonColor::Primary);

    view! {
        <div>
            <Button color=color />
            <button on:click=move |_| set_color.update(|c| *c = ButtonColor::Secondary)>
                "Change Color"
            </button>
        </div>
    }
}
```

Expected: Button color changes when clicked.

### Responsive Testing

- [ ] Component works on mobile viewport (320px+)
- [ ] Component works on tablet viewport (768px+)
- [ ] Component works on desktop viewport (1024px+)
- [ ] Component doesn't break at different sizes
- [ ] Layout is flexible and adapts

### Cross-Browser Testing

Test in:
- [ ] Chrome/Chromium
- [ ] Firefox
- [ ] Safari
- [ ] Edge

### Accessibility Testing

- [ ] Keyboard navigation works (Tab, Enter, Space, Arrow keys)
- [ ] Focus indicators are visible
- [ ] Screen reader announces component correctly
- [ ] ARIA attributes are present where needed
- [ ] Color contrast meets WCAG standards
- [ ] Interactive elements are reachable by keyboard

## Browser Testing Workflow

### Using the Demo Application

```bash
cd demo
npm ci
trunk serve
```

Then:
1. Open browser to `http://localhost:8080`
2. Navigate to component test page
3. Verify all variants and interactions
4. Check browser console for errors
5. Test keyboard navigation
6. Test on different viewport sizes

### Manual Test Component Pattern

```rust
#[component]
fn ComponentTest() -> impl IntoView {
    view! {
        <div class="p-4 space-y-4">
            <h1>"{component_name} Test"</h1>

            <h2>"Default"</h2>
            <ComponentName />

            <h2>"All Colors"</h2>
            <ComponentName color=ComponentColor::Neutral />
            <ComponentName color=ComponentColor::Primary />
            <ComponentName color=ComponentColor::Secondary />
            // ... more variants

            <h2>"All Sizes"</h2>
            <ComponentName size=ComponentSize::Xs />
            <ComponentName size=ComponentSize::Sm />
            <ComponentName size=ComponentSize::Md />
            // ... more variants

            <h2>"Interactive"</h2>
            <ComponentTestInteractive />
        </div>
    }
}
```

## Future: Automated Testing

When adding automated tests, use this structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_component_renders() {
        // Test implementation
    }

    #[test]
    fn test_reactive_props_update_view() {
        // Test signal reactivity
    }
}
```

### Test Categories to Implement

- [ ] Unit tests for utility functions
- [ ] Component rendering tests
- [ ] Reactivity tests (signals trigger updates)
- [ ] Style enum `as_str()` tests
- [ ] Class merging tests

## Regression Testing

After bug fixes:
- [ ] Create test case that failed before fix
- [ ] Verify test passes after fix
- [ ] Document the bug and fix

## Performance Testing

For performance concerns:
- [ ] Profile rendering time
- [ ] Check for unnecessary re-renders
- [ ] Verify Signal granularity is appropriate
- [ ] Test with large numbers of components

## Browser Console Testing

Always check browser console:
- [ ] No JavaScript errors
- [ ] No console warnings
- [ ] No missing CSS warnings
- [ ] Verify no 404s for assets

## Testing Quick Reference

```bash
# Start dev server
cd demo && trunk serve

# Check for WASM compilation issues
cargo build --target wasm32-unknown-unknown

# Check component exports
cargo doc --no-deps --open
```

## Test Documentation

When documenting component behavior:
- [ ] Document tested browsers/versions
- [ ] Note any browser-specific issues
- [ ] Document keyboard shortcuts
- [ ] List known limitations

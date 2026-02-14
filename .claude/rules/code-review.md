# Code Review Guidelines

Guidelines for reviewing code in this project.

## Review Checklist

Use this checklist when reviewing or before requesting review:

### Functionality

- [ ] Code compiles without errors
- [ ] Code achieves stated purpose
- [ ] Edge cases are handled
- [ ] Error handling is appropriate
- [ ] No obvious bugs or logic errors

### Type Safety

- [ ] No stringly-typed APIs (strings for style variants)
- [ ] Enums used for all style variants
- [ ] `Signal<T>` used for reactive props
- [ ] `Option<T>` used for optional values
- [ ] Correct HTML element types referenced

### API Design

- [ ] Props follow naming conventions
- [ ] Required vs optional props are logical
- [ ] `class` and `node_ref` props present on components
- [ ] Consistent with similar components
- [ ] Public API is intuitive

### Performance

- [ ] No unnecessary clones
- [ ] Signal granularity is appropriate
- [ ] Expensive operations are minimized
- [ ] Reactive closures use `move` correctly

### Documentation

- [ ] All public items documented
- [ ] Rustdoc comments are clear
- [ ] CSS classes documented
- [ ] Examples are accurate
- [ ] MDN links are correct

### Style

- [ ] Code passes `leptosfmt --check .`
- [ ] Code passes `cargo clippy -- -D warnings`
- [ ] Naming is consistent with codebase
- [ ] No commented-out code
- [ ] No TODOs without issues

## Component-Specific Reviews

### For New Components

- [ ] File structure is correct (mod.rs, component.rs, style.rs)
- [ ] All style variants from daisyUI are implemented
- [ ] Default variants return empty string
- [ ] Node refs are properly typed
- [ ] CSS @source directive includes all classes

### For Component Modifications

- [ ] Changes don't break existing API
- [ ] Breaking changes are documented
- [ ] Backward compatibility considered
- [ ] Migration path provided for breaking changes

## Review Process

### Before Requesting Review

1. Run all pre-commit checks
2. Test your changes manually
3. Review your own diff
4. Write clear commit messages
5. Update documentation

### During Review

Focus on:
- **Intent**: Does the code achieve its goal?
- **Correctness**: Does it work as intended?
- **Clarity**: Is it easy to understand?
- **Maintainability**: Will it be easy to update later?

Be constructive and specific:
- Point out specific lines with issues
- Explain why something is problematic
- Suggest improvements
- Ask questions to understand intent

### Review Response

For review feedback:
- Address all comments
- Push follow-up commits
- Request re-review when done
- Close the loop on all feedback

## Common Issues to Look For

### Type Safety Violations

```rust
// ❌ BAD: String-based API
fn set_color(color: &str) { ... }

// ✅ GOOD: Enum-based
fn set_color(color: ButtonColor) { ... }
```

### Missing Reactivity

```rust
// ❌ BAD: Won't update when signal changes
class=|| format!("btn {}", color.get().as_str())

// ✅ GOOD: Reactive closure
class=move || merge_classes!("btn", color.get().as_str())
```

### Unnecessary Clones

```rust
// ❌ BAD: Unnecessary clone
let sig = signal.clone();
view! {
    <div prop=sig />
}

// ✅ GOOD: Signal is cheap to clone, but document why
// Signals are cheap to clone - this is fine
let sig = signal.clone();
```

### Missing Documentation

```rust
// ❌ BAD: No documentation
#[component]
pub fn MyComponent(...) -> impl IntoView { ... }

// ✅ GOOD: Documented
/// # MyComponent
///
/// Brief description of what it does and when to use it.
#[component]
pub fn MyComponent(...) -> impl IntoView { ... }
```

## Approval Criteria

Approve when:
- [ ] All review comments addressed
- [ ] No blocking issues remain
- [ ] Code meets all style standards
- [ ] Documentation is complete
- [ ] Tests pass (when tests exist)

## Review Etiquette

- Be respectful and constructive
- Focus on code, not the person
- Ask questions to understand intent
- Suggest, don't dictate
- Acknowledge good work
- Consider alternative approaches

## Requesting Reviews

When requesting a review:
- Describe what was changed and why
- Link to related issues or PRs
- Note specific areas of concern
- Keep changes focused and reviewable

Example review request:
```
"Added Button component with full daisyUI variant support.

This implements all 62 daisyUI Button variants including:
- Colors (9 variants)
- Sizes (5 variants)
- Styles (6 variants)
- Shapes (4 variants)

Special attention requested on:
- CSS class documentation completeness
- Signal usage in reactive closures

Refs: #12, #15"
```

---
name: check-style-guide
description: Validates existing components against daisyUI style guide documentation
disable-model-invocation: true
---

Validates that existing component implementations match the official daisyUI style guide.

## Prerequisites

1. Verify `.daisyui` directory exists (run `install-component-guide` skill if missing)
2. Review `.claude/rules/components.md` for implementation standards

## Validation Steps

### 1. Read daisyUI Documentation

```bash
cat .daisyui/components/{component_name}.md
```

Extract from "Class names" section:
- Base component class (e.g., `btn`)
- Color variants (e.g., `btn-primary`, `btn-secondary`)
- Size variants (e.g., `btn-xs`, `btn-sm`)
- Style/shape variants (e.g., `btn-outline`, `btn-ghost`)

### 2. Read Implementation

```bash
# Style enums
cat src/components/{component_name}/style.rs

# Component code
cat src/components/{component_name}/component.rs
```

### 3. Compare Implementation

Create comparison table:

| Category | daisyUI Classes                     | Implemented Enums                                     | Coverage |
| -------- | ----------------------------------- | ----------------------------------------------------- | -------- |
| Colors   | `btn-primary`, `btn-secondary`, ... | `ButtonColor::Primary`, `ButtonColor::Secondary`, ... | ✅/❌      |
| Sizes    | `btn-xs`, `btn-sm`, ...             | `ButtonSize::Xs`, `ButtonSize::Sm`, ...               | ✅/❌      |
| Styles   | `btn-outline`, `btn-ghost`, ...     | `ButtonStyle::Outline`, `ButtonStyle::Ghost`, ...     | ✅/❌      |

### 4. Check Implementation Quality

#### Style Enums (`style.rs`)

- [ ] All variants from daisyUI doc are present
- [ ] Each enum has `#[default]` variant returning `""`
- [ ] CSS class names match daisyUI exactly (e.g., `"btn-primary"`, not `"primary"`)

#### Component Code (`component.rs`)

- [ ] Base class used in `merge_classes!`
- [ ] All style enums used as `Signal<T>` props
- [ ] `class` and `node_ref` props present
- [ ] Documentation includes `@source inline()` with ALL classes
- [ ] Documentation references correct HTML element type with MDN link

### 5. Common Issues to Check

#### Missing Variants

```
❌ Missing: Color variant "warning" (btn-warning)
✅ Action: Add to style.rs enum
```

#### Incorrect Class Names

```rust
// ❌ WRONG - missing prefix
ButtonColor::Primary => "primary"

// ✅ CORRECT - full class name
ButtonColor::Primary => "btn-primary"
```

#### Incomplete Documentation

```rust
// ❌ INCOMPLETE - missing classes
/// @source inline("btn btn-primary");

// ✅ COMPLETE - all classes
/// @source inline("btn btn-neutral btn-primary btn-secondary ...");
```

### 6. Generate Report

#### Passing Component

```
Component: button
Status: ✅ PASS
Coverage: 100% (28/28 variants)
```

#### Component with Issues

```
Component: {name}
Status: ⚠️ NEEDS ATTENTION
Coverage: 75% (15/20 variants)

Missing:
- Color: "warning" ({class-warning})
- Size: "xl" ({class-xl})

Incorrect:
- Style::Link returns "link" not "btn-link"

Fix:
- src/components/{name}/style.rs - add missing variants, fix class names
- src/components/{name}/component.rs - update @source inline()
```

## Quick Commands

```bash
# Compare daisyUI with implementation
echo "=== daisyUI ==="
cat .daisyui/components/button.md | grep -A 20 "Class names"

echo "=== Implemented ==="
grep -E "impl.*Color" src/components/button/style.rs -A 20 | grep '=> "'

echo "=== Documentation ==="
grep "@source inline" src/components/button/component.rs
```

## Batch Validation

```bash
for component in src/components/*/; do
    name=$(basename "$component")
    if [ -f ".daisyui/components/$name.md" ]; then
        echo "Validating: $name"
        # validation here
    fi
done
```

## Reference

- `.claude/rules/components.md` - Implementation standards
- `.claude/rules/rust.md` - Rust coding standards

---
name: create-component-docs
description: Generates component documentation from implementation for demo-macros parsing
---

Creates markdown documentation for existing components to be parsed by demo-macros.

## Prerequisites

1. **Verify component exists**: `src/components/{component_name}/` has implementation
2. **Read standards**: `doc/README.md` for supported elements and structure
3. **Read rules**: `.claude/rules/documentation.md` for doc standards

## Quick Steps

### 1. Read Implementation

```bash
cat src/components/{component_name}/style.rs
cat src/components/{component_name}/component.rs
```

Extract:
- Public components (marked with `#[component]`)
- All props with types and defaults
- Sub-components
- Style enum variants

### 2. Create File

```bash
doc/components/{component_name}.md
```

Use kebab-case matching module name from `src/components/`.

### 3. Document Structure

```markdown
# {ComponentName}

One sentence description.

## Description

2-4 sentences: What it provides, when to use it, common cases.

## Examples

### Basic Usage

```rust
<Component prop1=value1 prop2=value2>
    {children()}
</Component>
```

### With Variants

```rust
// Realistic example showing different configurations
```

## Props

| Prop        | Type               | Default   | Description          |
| ----------- | ------------------ | --------- | -------------------- |
| `prop_name` | `Signal<PropType>` | `default` | One-line description |

## Sub Components

### {SubComponent}

Brief description (1-2 sentences).

| Prop   | Type   | Default   | Description |
| ------ | ------ | --------- | ----------- |
| `prop` | `Type` | `default` | Description |
```

### 4. Supported Elements (demo-macros)

Only these markdown elements are parsed:

- **Headings** (h1-h3): `#`, `##`, `###`
- **Paragraphs**: Plain text blocks
- **Code blocks**: ```rust ... ``` (executed as live components)
- **Tables**: Pipe-separated tables → daisyUI Table component

**NOT supported**:
- ❌ Lists, blockquotes, horizontal rules, images, links

### 5. Props Table Rules

- Alphabetical order for props
- Show `Signal<T>` wrapping explicitly
- Use `-` for required props (no default)
- Defaults: `""` for strings, `false`/`true` for bool, `-` for `Option<T>`

### 6. Sub Components

For components with child parts, follow same props table format.

### 7. Validate

```bash
# Check macro parsing
cargo build

# Verify in demo
cd demo && trunk serve
```

Check:
- [ ] File path matches component name exactly
- [ ] All code blocks valid Rust
- [ ] Tables have proper header/body rows
- [ ] Only supported elements used
- [ ] All public props documented

## Quick Reference

```bash
# Read implementation
cat src/components/button/component.rs

# Check existing doc (if updating)
cat doc/components/button.md

# Validate
cargo build
```

## Reference

- `doc/README.md` - Full documentation system guide
- `.claude/rules/documentation.md` - Documentation standards
- `doc/components/accordion.md` - Canonical example

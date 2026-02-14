# Checkbox

A boolean selection input component with support for indeterminate state and comprehensive styling options.

## Description

The Checkbox component provides a type-safe, reactive wrapper for native checkbox inputs with daisyUI styling. It supports multiple color themes and size variants while maintaining full accessibility and keyboard navigation. Perfect for forms, settings panels, task lists, and multi-select interfaces.

## Examples

### Color Variants

```rust
<div class="form-control">
    <label class="label cursor-pointer">
        <Checkbox color=CheckboxColor::Primary />
        <span class="label-text">"Primary checkbox"</span>
    </label>

    <label class="label cursor-pointer">
        <Checkbox color=CheckboxColor::Success />
        <span class="label-text">"Success checkbox"</span>
    </label>

    <label class="label cursor-pointer">
        <Checkbox color=CheckboxColor::Warning />
        <span class="label-text">"Warning checkbox"</span>
    </label>

    <label class="label cursor-pointer">
        <Checkbox color=CheckboxColor::Error />
        <span class="label-text">"Error checkbox"</span>
    </label>
</div>
```

### Size Options

```rust
<div class="form-control">
    <label class="label cursor-pointer">
        <Checkbox size=CheckboxSize::Xs />
        <span class="label-text">"Extra Small"</span>
    </label>

    <label class="label cursor-pointer">
        <Checkbox size=CheckboxSize::Sm />
        <span class="label-text">"Small"</span>
    </label>

    <label class="label cursor-pointer">
        <Checkbox size=CheckboxSize::Md />
        <span class="label-text">"Medium"</span>
    </label>

    <label class="label cursor-pointer">
        <Checkbox size=CheckboxSize::Lg />
        <span class="label-text">"Large"</span>
    </label>

    <label class="label cursor-pointer">
        <Checkbox size=CheckboxSize::Xl />
        <span class="label-text">"Extra Large"</span>
    </label>
</div>
```

### Task List

```rust
<div class="form-control">
    <label class="label cursor-pointer justify-start gap-4">
        <Checkbox color=CheckboxColor::Accent class="checkbox-xs" />
        <span class="label-text">"Review documentation"</span>
    </label>

    <label class="label cursor-pointer justify-start gap-4">
        <Checkbox color=CheckboxColor::Accent class="checkbox-xs" />
        <span class="label-text">"Complete component implementation"</span>
    </label>

    <label class="label cursor-pointer justify-start gap-4">
        <Checkbox color=CheckboxColor::Accent class="checkbox-xs" />
        <span class="label-text">"Add unit tests"</span>
    </label>

    <label class="label cursor-pointer justify-start gap-4">
        <Checkbox color=CheckboxColor::Accent class="checkbox-xs" />
        <span class="label-text">"Update README"</span>
    </label>
</div>
```

## Props

| Prop       | Type                    | Default   | Description            |
| ---------- | ----------------------- | --------- | ---------------------- |
| `class`    | `&'static str`          | `""`      | Additional CSS classes |
| `color`    | `Signal<CheckboxColor>` | `Default` | Color variant          |
| `node_ref` | `NodeRef<HtmlInput>`    | -         | Node reference         |
| `size`     | `Signal<CheckboxSize>`  | `Md`      | Size variant           |

# Divider

A visual separator component for dividing content sections with optional text and semantic coloring.

## Description

The Divider component provides a flexible way to visually separate content with horizontal or vertical orientations. It supports text labels, semantic colors, and configurable placement. Perfect for section breaks, content grouping, timeline separators, and visual organization.

## Examples

### Horizontal Dividers

```rust
<div class="space-y-4">
    <p>"Section 1 content"</p>
    <Divider />
    <p>"Section 2 content"</p>
    <Divider>"OR"</Divider>
    <p>"Section 3 content"</p>
</div>
```

### Horizontal Divider

```rust
<div class="flex h-32 gap-4">
    <div class="flex-1 bg-base-200 rounded-lg p-4">"Content 1"</div>
    <Divider direction=DividerDirection::Horizontal />
    <div class="flex-1 bg-base-300 rounded-lg p-4">"Content 2"</div>
</div>
```

### Colors and Placement

```rust
<div class="space-y-4">
    <Divider color=DividerColor::Primary>"Primary Section"</Divider>
    <Divider color=DividerColor::Success placement=DividerPlacement::Start>"Start Aligned"</Divider>
    <Divider color=DividerColor::Warning placement=DividerPlacement::End>"End Aligned"</Divider>
    <Divider color=DividerColor::Info>"Info Divider"</Divider>
</div>
```

## Props

| Prop        | Type                       | Default      | Description            |
| ----------- | -------------------------- | ------------ | ---------------------- |
| `children`  | `Option<Children>`         | -            | Optional text content  |
| `class`     | `&'static str`             | `""`         | Additional CSS classes |
| `color`     | `Signal<DividerColor>`     | `Default`    | Semantic color variant |
| `direction` | `Signal<DividerDirection>` | `Horizontal` | Orientation of divider |
| `node_ref`  | `NodeRef<Div>`             | -            | Node reference         |
| `placement` | `Signal<DividerPlacement>` | `Default`    | Text positioning       |

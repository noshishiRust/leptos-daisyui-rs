# Badge

A compact status indicator component for displaying labels, counters, and contextual information.

## Description

The Badge component provides a flexible way to display small pieces of information like status indicators, notification counts, or category labels. It supports multiple sizes, semantic colors, and visual styles to fit various UI contexts. Perfect for notification counts, status indicators, category labels, and metadata tags.

## Examples

### Semantic Colors

```rust
<Badge color=BadgeColor::Primary>"New"</Badge>
<Badge color=BadgeColor::Success>"Active"</Badge>
<Badge color=BadgeColor::Warning>"Pending"</Badge>
<Badge color=BadgeColor::Error>"Failed"</Badge>
```

### Sizes and Styles

```rust
<Badge size=BadgeSize::Xs color=BadgeColor::Info>"XS"</Badge>
<Badge size=BadgeSize::Sm color=BadgeColor::Info>"Small"</Badge>
<Badge size=BadgeSize::Md color=BadgeColor::Info>"Medium"</Badge>
<Badge size=BadgeSize::Lg color=BadgeColor::Info>"Large"</Badge>

<Badge style=BadgeStyle::Outline color=BadgeColor::Primary>"Outline"</Badge>
<Badge style=BadgeStyle::Soft color=BadgeColor::Primary>"Soft"</Badge>
<Badge style=BadgeStyle::Ghost color=BadgeColor::Primary>"Ghost"</Badge>
```

### Notification Counters

```rust
<div class="relative">
    <Button>"Messages"</Button>
    <Badge size=BadgeSize::Xs color=BadgeColor::Error class="absolute -top-2 -right-2">"5"</Badge>
</div>

<div class="relative">
    <Button>"Notifications"</Button>
    <Badge size=BadgeSize::Sm color=BadgeColor::Success class="absolute -top-2 -right-2">"12"</Badge>
</div>
```

## Props

| Prop     | Type                   | Default   | Description                    |
| -------- | ---------------------- | --------- | ------------------------------ |
| `children` | `Children`           | -         | Badge content               |
| `class`  | `&'static str`        | `""`      | Additional CSS classes      |
| `color`  | `Signal<BadgeColor>`  | `Default` | Semantic color scheme      |
| `node_ref` | `NodeRef<Div>`      | -         | Node reference              |
| `size`   | `Signal<BadgeSize>`  | `Md`      | Badge size                 |
| `style`  | `Signal<BadgeStyle>` | `Default` | Visual style variant       |

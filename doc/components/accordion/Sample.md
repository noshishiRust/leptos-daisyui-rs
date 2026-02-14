# Accordion

A collapsible content component that allows users to expand and collapse sections of information.

## Description

The Accordion component provides an organized way to display large amounts of content in a compact space. Users can expand sections they're interested in while keeping other sections collapsed, making it perfect for FAQs, settings panels, and content organization.

## Examples

### Basic Accordion

```rust
accordion#basic
```

### Modifiers

```rust
accordion#modifiers
```

## Props

| Prop         | Type               | Default | Description                            |
| ------------ | ------------------ | ------- | -------------------------------------- |
| `open`       | `Signal<bool>`     | `false` | Whether the accordion item is expanded |
| `arrow`      | `Signal<bool>`     | `true`  | Whether to show expand/collapse arrow  |
| `plus_minus` | `Signal<bool>`     | `false` | Use plus/minus icons instead of arrow  |
| `class`      | `&'static str`     | `""`    | Additional CSS classes                 |
| `on_toggle`  | `Option<Callback>` | -       | Callback when accordion is toggled     |
| `children`   | `Children`         | -       | Accordion content                      |

## Sub Components

### AccordionTitle
The clickable header section that toggles the accordion.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `children` | `Children`     | -       | Title content          |

### AccordionContent
The collapsible content area of the accordion.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `children` | `Children`     | -       | Content to show/hide   |

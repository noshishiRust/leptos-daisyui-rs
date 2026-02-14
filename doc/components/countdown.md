# Countdown

A numeric display component with smooth animated transitions for values between 0-99.

## Description

The Countdown component provides an animated number display that smoothly transitions between values. Each CountdownValue animates independently when its numeric value changes, making it perfect for timers, countdowns, score displays, and any animated numeric interface.

## Examples

### Basic Countdown

```rust
<Countdown>
    <CountdownValue value=23 />
    <span>":"</span>
    <CountdownValue value=59 />
    <span>":"</span>
    <CountdownValue value=99 />
</Countdown>
```

### Large Display

```rust
<div class="text-6xl font-bold">
    <Countdown>
        <CountdownValue value=5 />
        <CountdownValue value=3 />
    </Countdown>
</div>
```

### Color Coded Values

```rust
<Countdown>
    <CountdownValue value=12 class="text-error" />
    <span>"h "</span>
    <CountdownValue value=45 class="text-warning" />
    <span>"m "</span>
    <CountdownValue value=30 class="text-success" />
    <span>"s"</span>
</Countdown>
```

## Props

| Prop       | Type            | Default | Description             |
| ---------- | --------------- | ------- | ----------------------- |
| `children` | `Children`      | -       | CountdownValue elements |
| `class`    | `&'static str`  | `""`    | Additional CSS classes  |
| `node_ref` | `NodeRef<Span>` | -       | Node reference          |

## Sub Components

### CountdownValue

Individual animated number within the countdown display.

| Prop         | Type                     | Default | Description                           |
| ------------ | ------------------------ | ------- | ------------------------------------- |
| `aria_label` | `Signal<Option<String>>` | -       | Accessible label (defaults to value)  |
| `children`   | -                        | -       | Not used - value is rendered directly |
| `class`      | `&'static str`           | `""`    | Additional CSS classes                |
| `node_ref`   | `NodeRef<Span>`          | -       | Node reference                        |
| `value`      | `Signal<u8>`             | 0       | Numeric value to display (0-99)       |

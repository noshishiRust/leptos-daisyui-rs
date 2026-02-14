# Button

An interactive button component with comprehensive styling options including colors, sizes, shapes, and states.

## Description

The Button component provides a versatile, type-safe way to create buttons with various visual styles, semantic colors, and interactive states. It supports loading spinners, active states, disabled states, and multiple shape options. Perfect for form actions, navigation triggers, and interactive controls throughout your application.

## Examples

### Color Variants

```rust
<Button color=ButtonColor::Primary>"Primary"</Button>
<Button color=ButtonColor::Secondary>"Secondary"</Button>
<Button color=ButtonColor::Success>"Success"</Button>
<Button color=ButtonColor::Warning>"Warning"</Button>
<Button color=ButtonColor::Error>"Error"</Button>
```

### Sizes and Shapes

```rust
<Button size=ButtonSize::Xs>"Extra Small"</Button>
<Button size=ButtonSize::Sm>"Small"</Button>
<Button size=ButtonSize::Md>"Medium"</Button>
<Button size=ButtonSize::Lg>"Large"</Button>

<Button shape=ButtonShape::Wide>"Wide Button"</Button>
<Button shape=ButtonShape::Square>"🔍"</Button>
<Button shape=ButtonShape::Circle>"+"</Button>
```

### States and Styles

```rust
<Button style=ButtonStyle::Outline>"Outline"</Button>
<Button style=ButtonStyle::Ghost>"Ghost"</Button>
<Button style=ButtonStyle::Soft>"Soft"</Button>

<Button loading=true>"Loading..."</Button>
<Button active=true>"Active"</Button>
<Button disabled=true>"Disabled"</Button>
```

## Props

| Prop       | Type                    | Default   | Description                          |
| ---------- | ----------------------- | --------- | ------------------------------------ |
| `active`   | `Signal<bool>`          | `false`   | Active state appearance              |
| `children` | `Children`              | -         | Button content                       |
| `class`    | `&'static str`          | `""`      | Additional CSS classes              |
| `color`    | `Signal<ButtonColor>`   | `Default` | Semantic color scheme               |
| `disabled` | `Signal<bool>`          | `false`   | Disabled state                      |
| `loading`  | `Signal<bool>`          | `false`   | Show loading spinner                |
| `node_ref` | `NodeRef<HTMLButton>`   | -         | Node reference                       |
| `shape`    | `Signal<ButtonShape>`   | `Default` | Button shape/layout                 |
| `size`     | `Signal<ButtonSize>`    | `Md`      | Button size                        |
| `style`    | `Signal<ButtonStyle>`   | `Default` | Visual style variant                |

## Sub Components

### LinkButton

An anchor element styled as a button for navigation actions.

| Prop       | Type                    | Default   | Description                          |
| ---------- | ----------------------- | --------- | ------------------------------------ |
| `children` | `Children`              | -         | Link content                         |
| `class`    | `&'static str`          | `""`      | Additional CSS classes              |
| `color`    | `Signal<ButtonColor>`   | `Default` | Semantic color scheme               |
| `href`     | `&'static str`          | `"#"`     | URL to navigate to                  |
| `node_ref` | `NodeRef<A>`            | -         | Node reference                       |
| `shape`    | `Signal<ButtonShape>`   | `Default` | Button shape/layout                 |
| `size`     | `Signal<ButtonSize>`    | `Md`      | Button size                        |
| `style`    | `Signal<ButtonStyle>`   | `Default` | Visual style variant                |

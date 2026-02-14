# Alert

A prominent notification component for displaying important messages, warnings, and contextual feedback to users.

## Description

The Alert component provides a flexible way to present messages with different severity levels and visual styles. It supports semantic coloring for different message types, visual style variations for emphasis, and layout direction options. Perfect for notifications, form validation feedback, system messages, and contextual alerts throughout your application.

## Examples

### Basic Alert

```rust
<Alert color=AlertColor::Info>
    <p>"New updates available for your account."</p>
</Alert>
```

### Semantic Colors

```rust
<Alert color=AlertColor::Success>
    <p>"Your changes have been saved successfully!"</p>
</Alert>

<Alert color=AlertColor::Warning>
    <p>"Your session will expire in 5 minutes."</p>
</Alert>

<Alert color=AlertColor::Error>
    <p>"Failed to save your changes. Please try again."</p>
</Alert>
```

### Visual Styles

```rust
<Alert style=AlertStyle::Outline color=AlertColor::Info>
    <p>"This alert has an outline style."</p>
</Alert>

<Alert style=AlertStyle::Dash color=AlertColor::Warning>
    <p>"This alert has a dashed border."</p>
</Alert>

<Alert style=AlertStyle::Soft color=AlertColor::Success>
    <p>"This alert has a soft, subtle appearance."</p>
</Alert>
```

### Layout Directions

```rust
<Alert direction=AlertDirection::Vertical color=AlertColor::Info>
    <span class="text-2xl">"ℹ"</span>
    <div>
        <h3 class="font-bold">"Information"</h3>
        <p>"Vertical layout with stacked content."</p>
    </div>
</Alert>

<Alert direction=AlertDirection::Horizontal color=AlertColor::Success>
    <span class="text-2xl">"✓"</span>
    <div>
        <h3 class="font-bold">"Success"</h3>
        <p>"Horizontal layout with inline content."</p>
    </div>
</Alert>
```

## Props

| Prop       | Type                    | Default   | Description                            |
| ---------- | ----------------------- | --------- | -------------------------------------- |
| `class`    | `&'static str`          | `""`      | Additional CSS classes                 |
| `children` | `Children`              | -         | Alert content (text, icons, buttons)   |
| `color`    | `Signal<AlertColor>`    | `Default` | Semantic color for message type       |
| `direction` | `Signal<AlertDirection>` | `Default` | Layout direction of alert content     |
| `node_ref` | `NodeRef<Div>`          | -         | Node reference for alert element       |
| `style`    | `Signal<AlertStyle>`    | `Default` | Visual style of alert appearance      |

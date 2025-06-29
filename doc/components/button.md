# Button

A versatile button component with multiple styles, sizes, and states. Built on top of daisyUI's button classes with full reactive support.

## Description

The Button component provides a clean, accessible way to handle user interactions. It supports various visual styles, sizes, loading states, and can be disabled or made responsive to user actions.

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `style` | `Signal<ButtonStyle>` | `ButtonStyle::Default` | Visual style of the button |
| `color` | `Signal<ButtonColor>` | `ButtonColor::Default` | Color theme of the button |
| `size` | `Signal<ButtonSize>` | `ButtonSize::Default` | Size of the button |
| `shape` | `Signal<ButtonShape>` | `ButtonShape::Default` | Shape style of the button |
| `outline` | `Signal<bool>` | `false` | Whether button has outline style |
| `disabled` | `Signal<bool>` | `false` | Whether button is disabled |
| `loading` | `Signal<bool>` | `false` | Whether button shows loading state |
| `wide` | `Signal<bool>` | `false` | Whether button takes full width |
| `glass` | `Signal<bool>` | `false` | Whether button has glass effect |
| `no_animation` | `Signal<bool>` | `false` | Whether to disable button animations |
| `class` | `&'static str` | `""` | Additional CSS classes |
| `on:click` | `Option<EventHandler>` | - | Click event handler |
| `children` | `Children` | - | Button content |

## Style Variants

### ButtonStyle
- `Default` - Standard button appearance
- `Primary` - Primary action button
- `Secondary` - Secondary action button
- `Accent` - Accent color button
- `Ghost` - Transparent button
- `Link` - Link-styled button

### ButtonColor
- `Default` - Default color
- `Neutral` - Neutral color
- `Primary` - Primary color
- `Secondary` - Secondary color
- `Accent` - Accent color
- `Info` - Info color
- `Success` - Success color
- `Warning` - Warning color
- `Error` - Error color

### ButtonSize
- `Default` - Standard size
- `Large` - Large button
- `Small` - Small button
- `ExtraSmall` - Extra small button

### ButtonShape
- `Default` - Standard shape
- `Square` - Square button
- `Circle` - Circular button

## Examples

### Basic Usage

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn BasicButtons() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Button>"Default"</Button>
            <Button style=Signal::derive(|| ButtonStyle::Primary)>
                "Primary"
            </Button>
            <Button style=Signal::derive(|| ButtonStyle::Secondary)>
                "Secondary"
            </Button>
        </div>
    }
}
```

</details>

### Colors

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn ColoredButtons() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Button color=Signal::derive(|| ButtonColor::Primary)>
                "Primary"
            </Button>
            <Button color=Signal::derive(|| ButtonColor::Success)>
                "Success"
            </Button>
            <Button color=Signal::derive(|| ButtonColor::Warning)>
                "Warning"
            </Button>
            <Button color=Signal::derive(|| ButtonColor::Error)>
                "Error"
            </Button>
        </div>
    }
}
```

</details>

### Sizes

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn SizedButtons() -> impl IntoView {
    view! {
        <div class="space-x-2 items-end flex">
            <Button size=Signal::derive(|| ButtonSize::ExtraSmall)>
                "XS"
            </Button>
            <Button size=Signal::derive(|| ButtonSize::Small)>
                "Small"
            </Button>
            <Button>
                "Normal"
            </Button>
            <Button size=Signal::derive(|| ButtonSize::Large)>
                "Large"
            </Button>
        </div>
    }
}
```

</details>

### Shapes

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn ShapedButtons() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Button>"Default"</Button>
            <Button shape=Signal::derive(|| ButtonShape::Square)>
                "□"
            </Button>
            <Button shape=Signal::derive(|| ButtonShape::Circle)>
                "○"
            </Button>
        </div>
    }
}
```

</details>

### States

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn StatefulButtons() -> impl IntoView {
    let (loading, set_loading) = signal(false);
    
    view! {
        <div class="space-x-2">
            <Button disabled=Signal::derive(|| true)>
                "Disabled"
            </Button>
            <Button loading=Signal::derive(move || loading.get())>
                "Loading"
            </Button>
            <Button 
                on:click=move |_| set_loading.update(|l| *l = !*l)
            >
                "Toggle Loading"
            </Button>
        </div>
    }
}
```

</details>

### Outline Style

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn OutlineButtons() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Button 
                outline=Signal::derive(|| true)
                color=Signal::derive(|| ButtonColor::Primary)
            >
                "Outline Primary"
            </Button>
            <Button 
                outline=Signal::derive(|| true)
                color=Signal::derive(|| ButtonColor::Success)
            >
                "Outline Success"
            </Button>
            <Button 
                outline=Signal::derive(|| true)
                color=Signal::derive(|| ButtonColor::Error)
            >
                "Outline Error"
            </Button>
        </div>
    }
}
```

</details>

### Interactive Example

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn InteractiveButton() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (loading, set_loading) = signal(false);
    
    let handle_click = move |_| {
        set_loading.set(true);
        set_timeout(move || {
            set_count.update(|c| *c += 1);
            set_loading.set(false);
        }, Duration::from_millis(1000));
    };
    
    view! {
        <div class="space-y-4">
            <Button 
                style=Signal::derive(|| ButtonStyle::Primary)
                loading=Signal::derive(move || loading.get())
                on:click=handle_click
            >
                {move || if loading.get() { 
                    "Loading...".to_string() 
                } else { 
                    format!("Clicked {} times", count.get()) 
                }}
            </Button>
            <p class="text-sm text-gray-600">
                "Click count: " {move || count.get()}
            </p>
        </div>
    }
}
```

</details>

## Accessibility

- Supports all standard button ARIA attributes
- Proper focus management with keyboard navigation
- Loading state announced to screen readers
- Disabled state properly communicated

## Best Practices

1. Use semantic button text that describes the action
2. Provide loading states for async operations
3. Use appropriate colors for different actions (success, error, warning)
4. Consider button hierarchy in your interface
5. Ensure sufficient contrast for accessibility
6. Use outline style for secondary actions

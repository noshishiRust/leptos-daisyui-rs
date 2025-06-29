# Card

A flexible content container with support for headers, bodies, actions, and various styling options.

## Description

The Card component provides a structured way to display related information. It can contain images, text, buttons, and other components in a visually appealing container with consistent spacing and styling.

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `style` | `Signal<CardStyle>` | `CardStyle::Default` | Visual style of the card |
| `image_placement` | `Signal<CardImagePlacement>` | `CardImagePlacement::Default` | Where to place the card image |
| `compact` | `Signal<bool>` | `false` | Whether card uses compact spacing |
| `bordered` | `Signal<bool>` | `false` | Whether card has a border |
| `side` | `Signal<bool>` | `false` | Whether card uses side layout |
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Card content |

## Style Variants

### CardStyle
- `Default` - Standard card appearance
- `Bordered` - Card with border
- `Compact` - Card with reduced padding
- `Side` - Horizontal card layout

### CardImagePlacement
- `Default` - No special image placement
- `Top` - Image at the top of the card
- `End` - Image at the end of the card

## Subcomponents

### CardTitle
Title section of the card with appropriate typography.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Title content |

### CardBody
Main content area of the card with proper spacing.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Body content |

### CardActions
Action area typically containing buttons or interactive elements.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Action elements |

## Examples

### Basic Card

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn BasicCard() -> impl IntoView {
    view! {
        <Card class="w-96 bg-base-100 shadow-xl">
            <CardBody>
                <CardTitle>"Card Title"</CardTitle>
                <p>"This is a basic card with some content. Cards are great for organizing related information."</p>
                <CardActions class="justify-end">
                    <Button style=Signal::derive(|| ButtonStyle::Primary)>
                        "Action"
                    </Button>
                </CardActions>
            </CardBody>
        </Card>
    }
}
```

</details>

### Card with Image

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn ImageCard() -> impl IntoView {
    view! {
        <Card class="w-96 bg-base-100 shadow-xl">
            <figure>
                <img src="https://via.placeholder.com/400x200" alt="Placeholder" />
            </figure>
            <CardBody>
                <CardTitle>"Featured Article"</CardTitle>
                <p>"This card includes an image at the top. Perfect for blog posts, products, or any content that benefits from visual context."</p>
                <CardActions class="justify-end">
                    <Button style=Signal::derive(|| ButtonStyle::Ghost)>
                        "Read More"
                    </Button>
                    <Button style=Signal::derive(|| ButtonStyle::Primary)>
                        "Share"
                    </Button>
                </CardActions>
            </CardBody>
        </Card>
    }
}
```

</details>

### Compact Card

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn CompactCard() -> impl IntoView {
    view! {
        <Card 
            compact=Signal::derive(|| true)
            class="w-96 bg-base-100 shadow-xl"
        >
            <CardBody>
                <CardTitle>"Compact Card"</CardTitle>
                <p>"This card uses compact spacing for a more condensed appearance."</p>
            </CardBody>
        </Card>
    }
}
```

</details>

### Side Card

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn SideCard() -> impl IntoView {
    view! {
        <Card 
            side=Signal::derive(|| true)
            class="w-96 bg-base-100 shadow-xl"
        >
            <figure>
                <img 
                    src="https://via.placeholder.com/150x150" 
                    alt="Profile"
                    class="w-24 h-24 rounded-lg"
                />
            </figure>
            <CardBody>
                <CardTitle>"John Doe"</CardTitle>
                <p>"Software Developer with 5+ years of experience in web development."</p>
                <CardActions>
                    <Button size=Signal::derive(|| ButtonSize::Small)>
                        "Connect"
                    </Button>
                </CardActions>
            </CardBody>
        </Card>
    }
}
```

</details>

### Bordered Card

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn BorderedCard() -> impl IntoView {
    view! {
        <Card 
            bordered=Signal::derive(|| true)
            class="w-96 bg-base-100"
        >
            <CardBody>
                <CardTitle>"Bordered Card"</CardTitle>
                <p>"This card has a border instead of a shadow for a different visual style."</p>
                <CardActions class="justify-end">
                    <Button style=Signal::derive(|| ButtonStyle::Outline)>
                        "Cancel"
                    </Button>
                    <Button style=Signal::derive(|| ButtonStyle::Primary)>
                        "Confirm"
                    </Button>
                </CardActions>
            </CardBody>
        </Card>
    }
}
```

</details>

### Interactive Card Grid

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn CardGrid() -> impl IntoView {
    let cards = vec![
        ("Design", "Create beautiful user interfaces", "🎨"),
        ("Development", "Build robust applications", "💻"),
        ("Testing", "Ensure quality and reliability", "🧪"),
        ("Deploy", "Ship your application", "🚀"),
    ];
    
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {cards.into_iter().map(|(title, description, icon)| {
                view! {
                    <Card class="bg-base-100 shadow-xl hover:shadow-2xl transition-shadow">
                        <CardBody>
                            <CardTitle class="flex items-center gap-2">
                                <span class="text-2xl">{icon}</span>
                                {title}
                            </CardTitle>
                            <p>{description}</p>
                            <CardActions class="justify-end">
                                <Button 
                                    size=Signal::derive(|| ButtonSize::Small)
                                    style=Signal::derive(|| ButtonStyle::Primary)
                                >
                                    "Learn More"
                                </Button>
                            </CardActions>
                        </CardBody>
                    </Card>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

</details>

## Accessibility

- Proper semantic structure with appropriate heading levels
- Support for keyboard navigation to interactive elements
- Screen reader friendly content organization
- Maintains focus management within card actions

## Best Practices

1. Use CardTitle for proper heading hierarchy
2. Keep card content focused and related
3. Place primary actions in CardActions for consistency
4. Use appropriate card width for content readability
5. Consider hover states for interactive cards
6. Group related cards in grids or lists for better organization
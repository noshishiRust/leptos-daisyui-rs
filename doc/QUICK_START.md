# Quick Start Guide

Get up and running with leptos-daisyui-rs in minutes.

## Installation

Add leptos-daisyui-rs to your project:

```bash
cargo add leptos-daisyui-rs
```

## Setup Tailwind CSS

Create or update your `input.css`:

```css
@import "tailwindcss";
@plugin "daisyui";
@source "../src/**/*.rs";
@source inline("btn btn-primary btn-secondary btn-accent btn-ghost");
```

## Basic Usage

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="p-8">
            <h1 class="text-4xl font-bold mb-4">"Hello daisyUI!"</h1>
            <Button color=ButtonColor::Primary>"Click Me"</Button>
        </div>
    }
}
```

## With Theming

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::theme::ThemeProvider;
use leptos_daisyui_rs::components::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <ThemeProvider load_from_storage=true>
            <div class="min-h-screen p-8">
                <h1 class="text-4xl font-bold">"Themed App"</h1>
                <BaseThemeSelector />
            </div>
        </ThemeProvider>
    }
}
```

## Next Steps

- [Theming System Guide](./THEMING.md) - Comprehensive theming documentation
- [Component Documentation](./components/) - Individual component guides
- [Demo Application](../demo/) - Interactive examples


# Theming System Guide

The leptos-daisyui-rs theming system provides comprehensive runtime theme customization capabilities, allowing you to create unique designs while leveraging daisyUI's design system.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Architecture Overview](#architecture-overview)
3. [ThemeProvider Setup](#themeprovider-setup)
4. [Base Theme Selection](#base-theme-selection)
5. [Color Customization](#color-customization)
6. [Typography Customization](#typography-customization)
7. [Border & Spacing Customization](#border--spacing-customization)
8. [Component-Specific Overrides](#component-specific-overrides)
9. [Import & Export](#import--export)
10. [Theme Sharing](#theme-sharing)
11. [API Reference](#api-reference)
12. [Best Practices](#best-practices)

---

## Quick Start

Get started with theming in just a few lines:

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::theme::ThemeProvider;
use leptos_daisyui_rs::components::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <ThemeProvider load_from_storage=true>
            <div class="min-h-screen p-8">
                <h1 class="text-4xl font-bold">"My App"</h1>
                <BaseThemeSelector />
                <ColorCustomizer />
            </div>
        </ThemeProvider>
    }
}
```

This setup provides:
- ✅ Automatic theme persistence in localStorage
- ✅ Reactive theme updates across all components
- ✅ Base theme selection from 32 daisyUI themes
- ✅ Real-time color customization

For complete documentation, see the sections below or visit our [demo application](../demo/).


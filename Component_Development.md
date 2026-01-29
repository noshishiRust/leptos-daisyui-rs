# Component Development Strategy

This document explains how to create custom components with unique presentation and functionality while fully participating in daisyUI's styling system and themes.

## Table of Contents

1. [Overview](#overview)
2. [Architecture Foundations](#architecture-foundations)
3. [Theme Integration](#theme-integration)
4. [Component Development Pattern](#component-development-pattern)
5. [Example: Collapsible Drawer](#example-collapsible-drawer)
6. [Best Practices](#best-practices)
7. [Testing & Documentation](#testing--documentation)

---

## Overview

### What Makes a Good Custom Component?

A well-designed custom component in this library should:

- ✅ Use daisyUI's CSS variables and theme system
- ✅ Follow Leptos reactive patterns with Signals
- ✅ Support spread attributes for extensibility
- ✅ Provide type-safe prop enums for styling options
- ✅ Include comprehensive documentation
- ✅ Expose NodeRef for direct DOM access
- ✅ Work seamlessly with daisyUI's theme switching

### When to Create a Custom Component

Create custom components when you need:

- **Unique Functionality**: Behavior not provided by daisyUI (e.g., collapsible drawers, multi-step forms, custom data visualizations)
- **Composite Patterns**: Combinations of existing daisyUI components with specific interaction patterns
- **Specialized Layouts**: Custom arrangements that leverage daisyUI's design tokens

---

## Architecture Foundations

### Component Structure

All components in this library follow a consistent structure:

```
src/components/your_component/
├── mod.rs           # Public exports
├── component.rs     # Component implementation
└── style.rs         # Style enums (optional)
```

### The Three-Layer Pattern

```rust
// Layer 1: Style Enums (style.rs)
// Define type-safe variants that map to CSS classes

#[derive(Clone, Debug, Default)]
pub enum ComponentVariant {
    #[default]
    Default,
    Compact,
    Wide,
}

impl ComponentVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentVariant::Default => "",
            ComponentVariant::Compact => "compact",
            ComponentVariant::Wide => "wide",
        }
    }
}

// Layer 2: Component Props (component.rs)
// Define the component interface with reactive Signals

#[component]
pub fn YourComponent(
    /// Reactive variant selector
    #[prop(optional, into)]
    variant: Signal<ComponentVariant>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for DOM access
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content
    children: Children,
) -> impl IntoView {
    // Layer 3: Implementation
}
```

---

## Theme Integration

### Understanding daisyUI Themes

daisyUI uses CSS variables that automatically update when themes change. Always use these variables instead of hardcoded colors.

#### Core Theme Variables

```css
/* Colors */
--p    /* primary */
--s    /* secondary */
--a    /* accent */
--n    /* neutral */
--b1   /* base-100 (background) */
--b2   /* base-200 */
--b3   /* base-300 */
--bc   /* base-content (text) */

/* Semantic Colors */
--in   /* info */
--su   /* success */
--wa   /* warning */
--er   /* error */

/* Interactive States */
--rounded-box    /* border radius */
--rounded-btn    /* button radius */
--animation-btn  /* button animation duration */
--animation-input /* input animation duration */
```

### Accessing Theme Variables

There are three ways to use daisyUI theme variables:

#### 1. Using daisyUI Utility Classes (Recommended)

```rust
view! {
    <div class="bg-base-100 text-base-content border border-base-300">
        // Content automatically themed
    </div>
}
```

#### 2. Using CSS Variables in Inline Styles

```rust
view! {
    <div style="background-color: oklch(var(--p)); color: oklch(var(--pc));">
        // Direct access to theme variables
    </div>
}
```

#### 3. Creating Custom CSS Classes

```css
/* In your input.css or component-specific CSS */
.my-custom-component {
    background-color: oklch(var(--b2));
    border: 1px solid oklch(var(--bc) / 0.2);
    border-radius: var(--rounded-box);
}

.my-custom-component:hover {
    background-color: oklch(var(--b3));
}
```

### Theme-Aware Transitions

```css
.my-component {
    /* Use theme animation variables */
    transition:
        background-color var(--animation-btn),
        transform 200ms ease-in-out;
}
```

---

## Component Development Pattern

### Step-by-Step Development Process

#### 1. Plan Your Component

Before coding, answer:

- What unique functionality does this provide?
- What props should be reactive (use `Signal<T>`)?
- What style variants are needed (enums)?
- Does it need child components?
- What daisyUI classes will you leverage?

#### 2. Create the File Structure

```bash
mkdir -p src/components/my_component
touch src/components/my_component/mod.rs
touch src/components/my_component/component.rs
# Optional: touch src/components/my_component/style.rs
```

#### 3. Define Style Enums (if needed)

```rust
// src/components/my_component/style.rs

/// Size variants for MyComponent
#[derive(Clone, Debug, Default)]
pub enum MyComponentSize {
    #[default]
    Normal,
    Small,
    Large,
}

impl MyComponentSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            MyComponentSize::Normal => "",
            MyComponentSize::Small => "my-component-sm",
            MyComponentSize::Large => "my-component-lg",
        }
    }
}
```

#### 4. Implement the Component

```rust
// src/components/my_component/component.rs

use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn MyComponent(
    /// Size variant
    #[prop(optional, into)]
    size: Signal<MyComponentSize>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "my-component",           // Base class
                    size.get().as_str(),      // Size variant
                    class                      // User classes
                )
            }
        >
            {children()}
        </div>
    }
}
```

#### 5. Export from Module

```rust
// src/components/my_component/mod.rs

mod component;
mod style;  // If you have style enums

pub use component::*;
pub use style::*;  // If you have style enums
```

#### 6. Register in Components Module

```rust
// src/components/mod.rs

mod my_component;
pub use my_component::*;
```

#### 7. Add CSS Classes to input.css

```css
/* In your project's input.css or demo/input.css */
@source inline("my-component my-component-sm my-component-lg");
```

---

## Example: Collapsible Drawer

Let's create a fully-featured collapsible drawer component with minimize/expand functionality.

### Design Specification

**Features:**
- Collapsible state (expanded/minimized)
- Smooth animations leveraging daisyUI transitions
- Icon visibility in minimized state
- Expand/collapse button
- Full theme integration
- Configurable width and position

### Implementation

#### Step 1: Style Enums

```rust
// src/components/collapsible_drawer/style.rs

/// Position of the collapsible drawer
#[derive(Clone, Debug, Default)]
pub enum DrawerPosition {
    #[default]
    Left,
    Right,
}

impl DrawerPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            DrawerPosition::Left => "drawer-left",
            DrawerPosition::Right => "drawer-right",
        }
    }
}

/// Width of the drawer when expanded
#[derive(Clone, Debug, Default)]
pub enum DrawerWidth {
    #[default]
    Normal,    // 16rem / 256px
    Wide,      // 20rem / 320px
    Narrow,    // 12rem / 192px
}

impl DrawerWidth {
    pub fn as_str(&self) -> &'static str {
        match self {
            DrawerWidth::Normal => "w-64",
            DrawerWidth::Wide => "w-80",
            DrawerWidth::Narrow => "w-48",
        }
    }

    pub fn minimized_width(&self) -> &'static str {
        // Minimized width is consistent across all sizes
        "w-16"
    }
}
```

#### Step 2: Component Implementation

```rust
// src/components/collapsible_drawer/component.rs

use super::style::{DrawerPosition, DrawerWidth};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Collapsible Drawer Component
///
/// A custom drawer component that can collapse to show only icons or expand to show full content.
/// Fully integrated with daisyUI theming and styling.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("collapsible-drawer drawer-left drawer-right");
/// ```
///
/// ## Features
/// - Smooth collapse/expand animations
/// - Configurable width and position
/// - Theme-aware styling
/// - Icon-only minimized state
/// - Full content expanded state
///
/// ## Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let (collapsed, set_collapsed) = signal(false);
///
///     view! {
///         <CollapsibleDrawer
///             collapsed=collapsed
///             on_toggle=move |_| set_collapsed.update(|c| *c = !*c)
///             width=DrawerWidth::Normal
///             position=DrawerPosition::Left
///         >
///             <CollapsibleDrawerIcons>
///                 <button class="btn btn-ghost btn-square">
///                     <svg>/* Home icon */</svg>
///                 </button>
///                 <button class="btn btn-ghost btn-square">
///                     <svg>/* Settings icon */</svg>
///                 </button>
///             </CollapsibleDrawerIcons>
///
///             <CollapsibleDrawerContent>
///                 <ul class="menu">
///                     <li><a>Home</a></li>
///                     <li><a>Settings</a></li>
///                 </ul>
///             </CollapsibleDrawerContent>
///         </CollapsibleDrawer>
///     }
/// }
/// ```
///
/// ## Node References
/// - `node_ref` - References the drawer container `<div>` element
#[component]
pub fn CollapsibleDrawer(
    /// Whether the drawer is collapsed (minimized)
    #[prop(into)]
    collapsed: Signal<bool>,

    /// Callback when toggle button is clicked
    #[prop(optional, into)]
    on_toggle: Option<Callback<ev::MouseEvent>>,

    /// Position of the drawer
    #[prop(optional, into)]
    position: Signal<DrawerPosition>,

    /// Width when expanded
    #[prop(optional, into)]
    width: Signal<DrawerWidth>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the drawer container
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Drawer content (use CollapsibleDrawerIcons and CollapsibleDrawerContent)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                let w = width.get();
                let current_width = if collapsed.get() {
                    w.minimized_width()
                } else {
                    w.as_str()
                };

                merge_classes!(
                    "collapsible-drawer",
                    "fixed top-0 h-screen",
                    "bg-base-200 border-r border-base-300",
                    "transition-all duration-300 ease-in-out",
                    "flex flex-col",
                    "shadow-lg",
                    current_width,
                    position.get().as_str(),
                    class
                )
            }
        >
            // Toggle button
            <div class="p-2 flex justify-end border-b border-base-300">
                <button
                    class="btn btn-ghost btn-sm btn-square"
                    on:click=move |ev| {
                        if let Some(callback) = on_toggle {
                            callback.run(ev);
                        }
                    }
                >
                    {move || if collapsed.get() {
                        view! {
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 5l7 7-7 7M5 5l7 7-7 7" />
                            </svg>
                        }.into_any()
                    } else {
                        view! {
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
                            </svg>
                        }.into_any()
                    }}
                </button>
            </div>

            // Content area
            <div class="flex-1 overflow-y-auto">
                {children()}
            </div>
        </div>
    }
}

/// # Collapsible Drawer Icons Component
///
/// Container for icon-only buttons that remain visible when the drawer is collapsed.
/// Place icon buttons (btn-square) inside this component.
#[component]
pub fn CollapsibleDrawerIcons(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Icon buttons
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || merge_classes!("flex flex-col gap-2 p-2", class)>
            {children()}
        </div>
    }
}

/// # Collapsible Drawer Content Component
///
/// Container for full content that appears when the drawer is expanded.
/// Use this with the parent's `collapsed` signal to conditionally render.
#[component]
pub fn CollapsibleDrawerContent(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Full content (menus, lists, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || merge_classes!("p-4", class)>
            {children()}
        </div>
    }
}
```

#### Step 3: Module Setup

```rust
// src/components/collapsible_drawer/mod.rs

mod component;
mod style;

pub use component::*;
pub use style::*;
```

```rust
// Add to src/components/mod.rs

mod collapsible_drawer;
pub use collapsible_drawer::*;
```

#### Step 4: CSS Integration

```css
/* In demo/input.css or your project's input.css */

@import "tailwindcss";
@plugin "daisyui";

/* Collapsible Drawer */
@source inline("collapsible-drawer drawer-left drawer-right");

/* Custom styling if needed */
.collapsible-drawer {
    /* All styling uses daisyUI theme variables */
    z-index: 10;
}

.drawer-left {
    left: 0;
}

.drawer-right {
    right: 0;
}
```

#### Step 5: Usage Example

```rust
// Example implementation in your app

use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn App() -> impl IntoView {
    let (collapsed, set_collapsed) = signal(false);

    view! {
        <div class="relative min-h-screen">
            <CollapsibleDrawer
                collapsed=collapsed
                on_toggle=move |_| set_collapsed.update(|c| *c = !*c)
                width=DrawerWidth::Normal
                position=DrawerPosition::Left
            >
                // Icons visible when collapsed
                <CollapsibleDrawerIcons>
                    <button class="btn btn-ghost btn-square" title="Home">
                        <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                        </svg>
                    </button>

                    <button class="btn btn-ghost btn-square" title="Settings">
                        <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                    </button>
                </CollapsibleDrawerIcons>

                // Full content when expanded
                <Show when=move || !collapsed.get()>
                    <CollapsibleDrawerContent>
                        <ul class="menu bg-base-100 rounded-box">
                            <li><a href="#home">
                                <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                                </svg>
                                Home
                            </a></li>
                            <li><a href="#settings">
                                <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                </svg>
                                Settings
                            </a></li>
                        </ul>
                    </CollapsibleDrawerContent>
                </Show>
            </CollapsibleDrawer>

            // Main content area with padding to account for drawer
            <div class=move || {
                let drawer_width = if collapsed.get() { "ml-16" } else { "ml-64" };
                format!("transition-all duration-300 {}", drawer_width)
            }>
                <div class="p-8">
                    <h1 class="text-3xl font-bold">"Main Content"</h1>
                    <p>"This content shifts when the drawer collapses/expands"</p>
                </div>
            </div>
        </div>
    }
}
```

---

## Best Practices

### 1. **Always Use Theme Variables**

❌ **Bad:**
```rust
view! {
    <div style="background-color: #1a1a1a; color: #ffffff;">
        "Hardcoded colors break theming"
    </div>
}
```

✅ **Good:**
```rust
view! {
    <div class="bg-base-200 text-base-content">
        "Uses theme variables"
    </div>
}
```

### 2. **Leverage Existing daisyUI Classes**

Before creating custom CSS, check if daisyUI already provides the styling:

```rust
// Use daisyUI's built-in classes for common patterns
view! {
    <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
            <h2 class="card-title">"Title"</h2>
            <p>"Content"</p>
            <div class="card-actions justify-end">
                <button class="btn btn-primary">"Action"</button>
            </div>
        </div>
    </div>
}
```

### 3. **Make Components Reactive**

Use `Signal<T>` for props that can change:

```rust
#[component]
pub fn MyComponent(
    // ✅ Reactive - updates when changed
    #[prop(into)]
    is_active: Signal<bool>,

    // ✅ Good for static data
    #[prop(optional, into)]
    title: &'static str,
) -> impl IntoView {
    view! {
        <div class:active=is_active>
            {title}
        </div>
    }
}
```

### 4. **Support Spread Attributes**

Always allow users to add custom attributes:

```rust
#[component]
pub fn MyComponent(
    // Standard props...

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("my-component", class)
            // Users can add: attr:data-id="123"
            //                 on:click=handler
            //                 class:active=signal
        >
            {children()}
        </div>
    }
}
```

### 5. **Follow Naming Conventions**

- **Components**: PascalCase (e.g., `CollapsibleDrawer`)
- **Enums**: PascalCase (e.g., `DrawerPosition`)
- **CSS Classes**: kebab-case (e.g., `collapsible-drawer`)
- **Files**: snake_case (e.g., `collapsible_drawer/`)

### 6. **Provide Child Components**

Break complex components into composable parts:

```rust
// Parent component
#[component]
pub fn Card(...) -> impl IntoView { }

// Child components for structure
#[component]
pub fn CardBody(...) -> impl IntoView { }

#[component]
pub fn CardActions(...) -> impl IntoView { }

// Usage:
view! {
    <Card>
        <CardBody>
            <p>"Content"</p>
        </CardBody>
        <CardActions>
            <button>"Action"</button>
        </CardActions>
    </Card>
}
```

### 7. **Optimize Performance**

Minimize reactive dependencies:

```rust
// ❌ Creates new closure on every render
class=move || format!("base-class {}", some_signal.get())

// ✅ Only recreates when signal changes
class=move || merge_classes!("base-class", some_signal.get())
```

---

## Testing & Documentation

### Component Documentation Template

```rust
/// # ComponentName Component
///
/// Brief description of what the component does and its unique functionality.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("component-class variant-1 variant-2");
/// ```
///
/// ## Features
/// - Feature 1
/// - Feature 2
/// - Feature 3
///
/// ## Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::*;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <ComponentName prop1="value">
///             "Content"
///         </ComponentName>
///     }
/// }
/// ```
///
/// ## Node References
/// - `node_ref` - References the container element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
///
/// ## Theme Integration
/// Automatically uses theme variables:
/// - `--b2` for background
/// - `--bc` for text color
/// - `--rounded-box` for border radius
#[component]
pub fn ComponentName(...) -> impl IntoView {
    // Implementation
}
```

### Testing Strategy

1. **Visual Testing**: Add to demo app for manual testing
2. **Theme Testing**: Test with multiple daisyUI themes
3. **Responsive Testing**: Test at different viewport sizes
4. **Accessibility Testing**: Verify ARIA attributes and keyboard navigation

### Documentation Checklist

Before considering a component complete:

- [ ] Comprehensive doc comments on component
- [ ] Doc comments on all public props
- [ ] Usage example in doc comments
- [ ] CSS classes documented
- [ ] Theme variables used are documented
- [ ] Node references documented with MDN links
- [ ] Example added to demo app
- [ ] README.md updated with new component

---

## Summary

Creating custom components for leptos-daisyui-rs involves:

1. **Planning**: Define functionality, props, and variants
2. **Structure**: Follow the three-layer pattern (enums, props, implementation)
3. **Theme Integration**: Use daisyUI CSS variables and utility classes
4. **Reactivity**: Leverage Leptos Signals for dynamic behavior
5. **Extensibility**: Support spread attributes and NodeRef
6. **Documentation**: Comprehensive docs with examples
7. **Testing**: Visual, theme, responsive, and accessibility testing

By following these patterns, your custom components will seamlessly integrate with the existing library while providing unique functionality tailored to your needs.

---

## Additional Resources

- [daisyUI Theme Variables](https://daisyui.com/docs/themes/)
- [Leptos Book - Components](https://book.leptos.dev/view/03_components.html)
- [Leptos Book - Reactivity](https://book.leptos.dev/reactivity/)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)

Happy component building! 🎨

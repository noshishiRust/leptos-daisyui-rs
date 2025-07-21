# daisyUI Components for Leptos

This crate is a daisyUI 5 components library for Leptos, providing type-safe, reactive wrappers for daisyUI 5 components.

""At present, it is assumed to be used for CSR.""

> 🚧 **Work in Progress**  
> This project is currently under active development.  
> The design and usage are still evolving, and breaking changes can be expected.  
> Feedback is welcome!

## How to use

### Install

Include this crate in your dependencies.

```sh
cargo add leptos-daisyui-rs
```

### Code

You can use components as follows: Tailwind CSS (v4) is used, so you can insert additional classes.


```rust
use leptos-daisyui-rs as daisyui'

use daisyui:components:::Accordion;

#[components]
fn Demo () -> impl IntoView {
    view! {
        <Accordion name="demo" checked=Signal::new(true) class="bg-base-100">
            <AccordionTitle class="text-lg">{"Accordion Title"}</AccordionTitle>
            <AccordionContent class="p-4">
                {"This is the content of the accordion. It can be any HTML content."}
            </AccordionContent>
        </Accordion>
    }
}
```

### CSS Install

As a note at build time, since the class names included in daisyUI are included in the crate, please refer to each component you use inline as follows.

```css input.css
@import "tailwindcss";
@plugin "daisyui";
@source "../src/**/*.rs";

/* Accordion */
@source inline("collapse collapse-title collapse-content collapse-arrow collapse-plus collapse-open collapse-close");
```

If you want to include everything first [daisyui-components.css](. /stytles/daisyui-components.css).

> 🚧 **There is room for optimization** 
> I still refer to class names by force in this area, so in the future I would like to include only the classes used in the build.

## How to Code

This section describes a more in-depth implementation.

###  Wrapper Components

Basically, this library implements a component that wraps a simple HTML element and hides the design part of daisyUI inside.

Therefore, it is designed to be flexible enough to add attributes and event listeners to the top HTML element using [Spread (Leptos Book)](https://book.leptos.dev/view/03_components.html#spreading-attributes-onto-components) .

For example, take a look at the following Button component:

```rust
use leptos::prelude::*;
use leptos::html::{Button as HTMLButton};
use leptos_daisyui_rs::components::*;;


let active = Signal::derive(move || some_condition());
let node_ref = NodeRef::<HTMLButton>::new();

// It is also possible to access the DOM API directly through NodeRef.
Effect::new(move || {
    let node_ref = node_ref.clone();
    let button = node_ref.get();
    if let Some(button) = button {
        if button.check_validity() {
            log::info!("Button is valid");
        } else {
            log::warn!("Button is invalid");
        }
    }
});

...

<Buttton
    // This is already defined as a property.
    color=ButtonColor::Neutral
    shape=ButtonShape::Square
    active=active
    node_ref=node_ref
    class="my-custom-class"

    // Attributes of the Top HTML element can be added using the `attr` modifier.
    //
    // - HTMLButtonElement: https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement
    // - HTMLElement: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement
    attr:name="my-button"
    attr:r#tytpe="button"

    // You can also add style or class attributes using the `class` and `style` modifiers.
    //
    // - Leptos ClassAttribute: https://docs.rs/leptos/latest/leptos/attr/global/trait.ClassAttribute.html
    // - Leptos StyleAttribute: https://docs.rs/leptos/latest/leptos/attr/global/trait.StyleAttribute.html
    class:btn=true
    style:font="normal"
    class:btn-block=true

    // Of course, event listeners belonging to Element can be added using the `on` modifier.
    //
    // - Leptos OnAttribute: https://docs.rs/leptos/latest/leptos/attr/global/trait.OnAttribute.html
    // - Leptos GlobalOnAttribute: https://docs.rs/leptos/latest/leptos/attr/global/trait.GlobalOnAttributes.html
    on:click=move |ev| {
        // Handle click event
        log::info!("Button clicked: {:?}", ev);
    }

    // You can also add custom properties using the `prop` modifier.
    //
    // Leptos PropAttribute: https://docs.rs/leptos/latest/leptos/attr/global/trait.PropAttribute.html
    prop:command="show-popover"
>
    "Button!"
</Buttton>
```

### What you can't do

While the above consists of top HTML elements that match daisyUI components to some degree, the CSS design should be more flexible than it should be. For example, we think it would be good to have a link (anchor tag) with a Button design.

If you would like to use the same design but use the internal configuration HTML elements, we would be glad to receive your contribution!!

As a workaround, a wrapper component that only assigns attributes to child components can be considered. For example

```rust
use leptos::prelude::*;
use leptos::tachys::html::class::class as class_fn;

#[component]
pub fn FullWrapperButton(children: Children) -> impl IntoView {
    // It can be added without overwriting it by making it a conditional class.
    children().add_any_attr(class_fn(("btn", true)))
}

...

<FullWrapperButton>
    <a href="/some-link">
        "Link Button"
    </a>
</FullWrapperButton>

<FullWrapperButton>
    <button>
        "Button"
    </button
</FullWrapperButton>
```


## Implementation Status

| Component | Status | Source Path | daisyUI Docs |
|-----------|--------|-------------|--------------|
| Accordion | ✅ | [src](src/components/accordion/) | [docs](https://daisyui.com/components/accordion/) |
| Alert | ✅ | [src](src/components/alert/) | [docs](https://daisyui.com/components/alert/) |
| Avatar | ✅ | [src](src/components/avatar/) | [docs](https://daisyui.com/components/avatar/) |
| Badge | ✅ | [src](src/components/badge/) | [docs](https://daisyui.com/components/badge/) |
| Breadcrumbs | ✅ | [src](src/components/breadcrumbs/) | [docs](https://daisyui.com/components/breadcrumbs/) |
| Button | ✅ | [src](src/components/button/) | [docs](https://daisyui.com/components/button/) |
| Calendar | - | [src](src/components/calendar/) | [docs](https://daisyui.com/components/calendar/) |
| Card | ✅ | [src](src/components/card/) | [docs](https://daisyui.com/components/card/) |
| Carousel | ✅ | [src](src/components/carousel/)  | [docs](https://daisyui.com/components/carousel/) |
| Chat | ✅ | [src](src/components/chat/) | [docs](https://daisyui.com/components/chat/) |
| Checkbox | ✅ | [src](src/components/checkbox/) | [docs](https://daisyui.com/components/checkbox/) |
| Collapse | ✅ | [src](src/components/collapse/) | [docs](https://daisyui.com/components/collapse/) |
| Countdown | ✅ | [src](src/components/countdown/) | [docs](https://daisyui.com/components/countdown/) |
| Diff | ✅ | [src](src/components/diff/) | [docs](https://daisyui.com/components/diff/) |
| Divider | ✅ | [src](src/components/divider/) | [docs](https://daisyui.com/components/divider/) |
| Dock | ✅ | [src](src/components/dock/) | [docs](https://daisyui.com/components/dock/) |
| Drawer | ✅ | [src](src/components/drawer/) | [docs](https://daisyui.com/components/drawer/) |
| Dropdown | ✅ | [src](src/components/dropdown/) | [docs](https://daisyui.com/components/dropdown/) |
| Fieldset | ✅ | [src](src/components/fieldset/) | [docs](https://daisyui.com/components/fieldset/) |
| File Input | ✅ | [src](src/components/file_input/) | [docs](https://daisyui.com/components/file-input/) |
| Filter | ✅ | [src](src/components/filter/) | [docs](https://daisyui.com/components/filter/) |
| Footer | ✅ | [src](src/components/footer/) | [docs](https://daisyui.com/components/footer/) |
| Hero | ✅ | [src](src/components/hero/) | [docs](https://daisyui.com/components/hero/) |
| Indicator | ✅ | [src](src/components/indicator/) | [docs](https://daisyui.com/components/indicator/) |
| Input | ✅ | [src](src/components/input/) | [docs](https://daisyui.com/components/input/) |
| Join | ✅ | [src](src/components/join/) | [docs](https://daisyui.com/components/join/) |
| Kbd | ✅ | [src](src/components/kbd/) | [docs](https://daisyui.com/components/kbd/) |
| Label | ✅ | [src](src/components/label/) | [docs](https://daisyui.com/components/label/) |
| Link | ✅ | [src](src/components/link/) | [docs](https://daisyui.com/components/link/) |
| List | ✅ | [src](src/components/list/) | [docs](https://daisyui.com/components/list/) |
| Loading | ✅ | [src](src/components/loading/) | [docs](https://daisyui.com/components/loading/) |
| Mask | ✅ | [src](src/components/mask/) | [docs](https://daisyui.com/components/mask/) |
| Menu | ✅ | [src](src/components/menu/) | [docs](https://daisyui.com/components/menu/) |
| Mockup Browser | ✅ | [src](src/components/mockup_browser/) | [docs](https://daisyui.com/components/mockup-browser/) |
| Mockup Code | ✅ | [src](src/components/mockup_code/) | [docs](https://daisyui.com/components/mockup-code/) |
| Mockup Phone | ✅ | [src](src/components/mockup_phone/) | [docs](https://daisyui.com/components/mockup-phone/) |
| Mockup Window | ✅ | [src](src/components/mockup_window/) | [docs](https://daisyui.com/components/mockup-window/) |
| Modal | ✅ | [src](src/components/modal/) | [docs](https://daisyui.com/components/modal/) |
| Navbar | ✅ | [src](src/components/navbar/) | [docs](https://daisyui.com/components/navbar/) |
| Pagination | ✅ | [src](src/components/pagination/) | [docs](https://daisyui.com/components/pagination/) |
| Progress | ✅ | [src](src/components/progress/) | [docs](https://daisyui.com/components/progress/) |
| Radial Progress | ✅ | [src](src/components/radial_progress/) | [docs](https://daisyui.com/components/radial-progress/) |
| Radio | ✅ | [src](src/components/radio/) | [docs](https://daisyui.com/components/radio/) |
| Range | ✅ | [src](src/components/range/) | [docs](https://daisyui.com/components/range/) |
| Rating | ✅ | [src](src/components/rating/) | [docs](https://daisyui.com/components/rating/) |
| Select | ✅ | [src](src/components/select/) | [docs](https://daisyui.com/components/select/) |
| Skeleton | ✅ | [src](src/components/skeleton/) | [docs](https://daisyui.com/components/skeleton/) |
| Stack | ✅ | [src](src/components/stack/) | [docs](https://daisyui.com/components/stack/) |
| Stats | ✅ | [src](src/components/stats/) | [docs](https://daisyui.com/components/stat/) |
| Status | ✅ | [src](src/components/status/) | [docs](https://daisyui.com/components/status/) |
| Steps | ✅ | [src](src/components/steps/) | [docs](https://daisyui.com/components/steps/) |
| Swap | ✅ | [src](src/components/swap/) | [docs](https://daisyui.com/components/swap/) |
| Tab | ✅ | [src](src/components/tab/) | [docs](https://daisyui.com/components/tab/) |
| Table | ✅ | [src](src/components/table/) | [docs](https://daisyui.com/components/table/) |
| Textarea | ✅ | [src](src/components/textarea/) | [docs](https://daisyui.com/components/textarea/) |
| Theme Controller | ✅ | [src](src/components/theme_controller/) | [docs](https://daisyui.com/components/theme-controller/) |
| Timeline | ✅ | [src](src/components/timeline/) | [docs](https://daisyui.com/components/timeline/) |
| Toast | ✅ | [src](src/components/toast/) | [docs](https://daisyui.com/components/toast/) |
| Toggle | ✅ | [src](src/components/toggle/) | [docs](https://daisyui.com/components/toggle/) |
| Validator | ✅ | [src](src/components/validator/) | [docs](https://daisyui.com/components/validator/) |

**Progress: 56/57 components implemented**


## TODO utility
- utility hooks
    - [ ] toggle
    - [ ] validator
    - [ ] modal
    - [ ] popover
    - etc ...
- utility provder
    - [ ] Theme controller
    - [ ] Toast Manager
    - etc ...

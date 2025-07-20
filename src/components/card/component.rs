use super::style::{CardSize, CardStyle};
use crate::merge_classes;
use leptos::prelude::*;

/// # Card Components
///
/// The `Card` component is the main container for card-based layouts, providing a structured
/// way to organize content with consistent styling and spacing. It supports various visual
/// styles, sizes, and layout options to accommodate different design needs.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("card card-title card-body card-actions card-boader card-dash card-side image-full card-xs card-sm card-md card-lg card-xl");
/// ```
///
/// ## Usage Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::card::*;
///
/// #[component]
/// fn StyledCard() -> impl IntoView {
///     view! {
///         <Card style=CardStyle::Border size=CardSize::Lg>
///             <CardBody>
///                 <CardTitle>"Product Card"</CardTitle>
///                 <p>"Product description and details go here."</p>
///                 <CardActions>
///                     <button class="btn btn-primary">"Buy Now"</button>
///                     <button class="btn btn-ghost">"Learn More"</button>
///                 </CardActions>
///             </CardBody>
///         </Card>
///     }
/// }
/// ```
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Card(
    /// Visual style variant for the card.
    #[prop(optional, into)]
    style: Signal<CardStyle>,

    /// Size preset for the card.
    #[prop(optional, into)]
    size: Signal<CardSize>,

    /// Enables horizontal side-by-side layout.
    #[prop(optional, into)]
    side: Signal<bool>,

    /// Makes background images cover the entire card area.

    #[prop(optional, into)]
    image_full: Signal<bool>,

    /// Additional CSS classes to apply to the card.

    #[prop(optional, into)]
    class: &'static str,

    /// Child elements to render inside the card.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                merge_classes!(
                    "card",
                style.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            class:card-side=side
            class:image-full=image_full
        >
            {children()}
        </div>
    }
}

/// # Card Body Component
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn CardBody(
    /// Additional CSS classes to apply to the card body.

    #[prop(optional, into)]
    class: &'static str,

    /// Child elements to render inside the card body.
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("card-body", class)>{children()}</div> }
}

/// # Card Title Component
///
/// ## Node References
/// - `node_ref` - References the top `<h2>` element ([HTMLHeadingElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement))
#[component]
pub fn CardTitle(
    /// Additional CSS classes to apply to the card title.

    #[prop(optional, into)]
    class: &'static str,

    /// Content to render as the card title.
    children: Children,
) -> impl IntoView {
    view! { <h2 class=move || merge_classes!("card-title", class)>{children()}</h2> }
}

/// # Card Actions Component
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement
#[component]
pub fn CardActions(
    /// Additional CSS classes to apply to the card actions container.
    ///
    /// These classes will be merged with the base `card-actions` class,
    /// allowing for custom layout, spacing, and alignment.
    #[prop(optional, into)]
    class: &'static str,

    /// Interactive elements to render in the actions area.
    ///
    /// This typically includes buttons, links, or other controls
    /// that allow users to interact with the card content.
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("card-actions", class)>{children()}</div> }
}

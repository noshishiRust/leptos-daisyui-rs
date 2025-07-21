use super::style::{CardSize, CardStyle};
use crate::merge_classes;
use leptos::html::{Div, H2};
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

    /// Node reference for the card container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child elements to render inside the card.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
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

    /// Node reference for the card body container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child elements to render inside the card body.
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("card-body", class)>
            {children()}
        </div>
    }
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

    /// Node reference for the card title text element
    #[prop(optional)]
    node_ref: NodeRef<H2>,

    /// Content to render as the card title.
    children: Children,
) -> impl IntoView {
    view! {
        <h2 node_ref=node_ref class=move || merge_classes!("card-title", class)>
            {children()}
        </h2>
    }
}

/// # Card Actions Component
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement
#[component]
pub fn CardActions(
    /// Additional CSS classes to apply to the card actions container.
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the card action container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Interactive elements to render in the actions area.
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("card-actions", class)>
            {children()}
        </div>
    }
}

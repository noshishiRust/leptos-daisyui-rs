use super::style::{CarouselDirection, CarouselModifier};
use crate::merge_classes;
use leptos::prelude::*;

/// # Carousel Component
///
/// A scrollable container for displaying images or content in a horizontal or vertical layout.
/// Supports various alignment and direction modifiers.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("carousel carousel-item carousel-start carousel-center carousel-end carousel-horizontal carousel-vertical");
/// ```
///
/// ## Node References
/// - No node_ref available - uses div element
#[component]
pub fn Carousel(
    /// Visual modifier for carousel alignment
    #[prop(optional, into)] 
    modifier: Signal<CarouselModifier>,

    /// Direction of carousel scroll (horizontal or vertical)
    #[prop(optional, into)] 
    direction: Signal<CarouselDirection>,

    /// Additional CSS classes to apply to the carousel container
    #[prop(optional, into)]
    class: &'static str,

    /// Child [`CarouselItem`] components
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            merge_classes!(
                "carousel",
                modifier.get().as_str(),
                direction.get().as_str(),
                class
            )
        }>{children()}</div>
    }
}

/// # Carousel Item Component
///
/// An individual item within a carousel. Use Tailwind CSS classes like `w-full`
/// to control item dimensions and appearance.
///
/// ## Node References
/// - No node_ref available - uses div element
#[component]
pub fn CarouselItem(
    /// Additional CSS classes to apply to the carousel item
    #[prop(optional, into)]
    class: &'static str,

    /// Content for this carousel item
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("carousel-item", class)>{children()}</div> }
}

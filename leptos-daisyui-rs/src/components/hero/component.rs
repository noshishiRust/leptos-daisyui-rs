use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Hero Component
///
/// A large banner section component for displaying prominent content like titles,
/// descriptions, and call-to-action elements, often with background images.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("hero hero-content hero-overlay");
/// ```
///
/// ## Node References
/// - `node_ref` - References the outer `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Hero(
    /// Additional CSS classes to apply to the hero container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the hero `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child components, typically [`HeroContent`] and optionally [`HeroOverlay`]
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("hero", class)>
            {children()}
        </div>
    }
}

/// # Hero Content Component
///
/// The main content area within a hero section, typically containing
/// headings, text, and action buttons.
///
/// ## Node References
/// - `node_ref` - References the content `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn HeroContent(
    /// Additional CSS classes to apply to the hero content
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the hero content `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content to display within the hero section
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("hero-content", class)>
            {children()}
        </div>
    }
}

/// # Hero Overlay Component
///
/// An overlay element that can be placed over background images to improve
/// text readability by adding a semi-transparent color layer.
///
/// ## Node References
/// - `node_ref` - References the overlay `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn HeroOverlay(
    /// Additional CSS classes to apply to the overlay
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the overlay `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! { <div node_ref=node_ref class=move || merge_classes!("hero-overlay", class)></div> }
}

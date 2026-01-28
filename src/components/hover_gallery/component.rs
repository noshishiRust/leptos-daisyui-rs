use crate::merge_classes;
use leptos::{html::Figure, prelude::*};

/// # Hover Gallery Component
///
/// A reactive Leptos wrapper for daisyUI's hover-gallery component. A container of images where
/// the first image is visible by default and other images show up when hovering horizontally.
/// Supports up to 10 images.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("hover-gallery");
/// ```
///
/// ## How It Works
/// The component creates invisible columns for each non-first image. Hovering over these columns
/// horizontally reveals the corresponding image. This is particularly useful for:
/// - E-commerce product displays
/// - Portfolios
/// - Image galleries requiring quick visual comparisons
///
/// ## Node References
/// - `node_ref` - References the `<figure>` element ([HTMLElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement))
#[component]
pub fn HoverGallery(
    /// Additional CSS classes for the gallery container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the figure element
    #[prop(optional)]
    node_ref: NodeRef<Figure>,

    /// Image elements (supports up to 10 images)
    children: Children,
) -> impl IntoView {
    view! {
        <figure node_ref=node_ref class=move || merge_classes!("hover-gallery", class)>
            {children()}
        </figure>
    }
}

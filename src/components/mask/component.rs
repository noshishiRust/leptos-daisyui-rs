use super::style::MaskType;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Mask Component
///
/// Crops content to various shapes using CSS masks. Useful for creating
/// shaped avatars, decorative elements, and custom layouts.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("mask mask-squircle mask-heart mask-hexagon mask-decagon mask-pentagon mask-diamond mask-square mask-circle mask-star mask-triangle mask-half-1 mask-half-2");
/// ```
///
/// ## Node References
/// - `node_ref` - References the mask `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Mask(
    /// Shape type for the mask
    #[prop(optional, into)] 
    mask_type: Signal<MaskType>,

    /// Additional CSS classes to apply to the mask container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the mask `<div>` element
    #[prop(optional)] 
    node_ref: NodeRef<Div>,

    /// Content to be masked (images, text, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "mask",
                mask_type.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}

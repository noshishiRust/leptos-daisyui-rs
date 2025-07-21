use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Skeleton Component
///
/// A loading placeholder component that displays an animated shimmer effect
/// while content is being loaded. Use Tailwind CSS classes to set dimensions.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("skeleton");
/// ```
///
/// ## Node References
/// - `node_ref` - References the skeleton `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Skeleton(
    /// Additional CSS classes to apply to the skeleton (typically width/height)
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the skeleton `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Optional child content (typically empty for pure skeleton effect)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("skeleton", class)>
            {children()}
        </div>
    }
}

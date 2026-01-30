use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Scrollbar Component
///
/// A container with custom scrollbar styling for overflow content.
/// Provides consistent scrollbar appearance across browsers.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("overflow-auto overflow-y-auto overflow-x-auto");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Scrollbar(
    /// Maximum height of the scrollable area
    #[prop(optional, into)]
    max_height: Signal<String>,

    /// Enable horizontal scrolling
    #[prop(optional, into)]
    horizontal: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content to make scrollable
    children: Children,
) -> impl IntoView {
    let overflow_class = Signal::derive(move || {
        if horizontal.get() {
            "overflow-auto"
        } else {
            "overflow-y-auto"
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!(overflow_class.get(), class)
            style=move || {
                let max_h = max_height.get();
                if !max_h.is_empty() {
                    format!("max-height: {}", max_h)
                } else {
                    String::new()
                }
            }
        >

            {children()}
        </div>
    }
}

/// # Mockup Window Component
///
/// A reactive Leptos wrapper for daisyUI's mockup window component that provides
/// a styled desktop window frame for displaying content.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("mockup-window");
/// ```
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))

use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn MockupWindow(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the div element
    #[prop(optional)] node_ref: NodeRef<Div>,
    /// Content displayed within the window frame
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("mockup-window", class)>
            {children()}
        </div>
    }
}
